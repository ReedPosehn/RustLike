use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;

// base pixel size of each tile asset (the PNGs are 32×32)
const TILE_BASE_SIZE: f32 = 32.0;
// global scale factor applied to everything. changing this will zoom the world
// in/out roughly uniformly; 1.0 is original size, 1.2 is 20% larger.
const SCALE: f32 = 1.2;
// effective size we use for positioning and collision math.
const TILE_SIZE: f32 = TILE_BASE_SIZE * SCALE;


const DUNGEON_WIDTH: usize = 40;
const DUNGEON_HEIGHT: usize = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Hub,
    Dungeon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Grass,
    Dirt,
    Stone,
    Wood,
    Water,
    Sand,
    Gravel,
    Rock,
    Wall,
    Door,
    Stairs,
}

impl TileType {
    fn asset_path(&self) -> &str {
        match self {
            TileType::Grass => "grass.png",
            TileType::Dirt => "dirt.png",
            TileType::Stone => "stone.png",
            TileType::Wood => "wood.png",
            TileType::Water => "water.png",
            TileType::Sand => "sand.png",
            TileType::Gravel => "gravel.png",
            TileType::Rock => "rock.png",
            TileType::Wall => "wall.png",
            TileType::Door => "door.png",
            TileType::Stairs => "stone.png",
        }
    }

    fn is_solid(&self) -> bool {
        matches!(self, TileType::Water | TileType::Rock | TileType::Wall)
    }
}

#[derive(Component)]
struct Player;


#[derive(Component)]
struct Tile {
    tile_type: TileType,
}

#[derive(Component, Clone, Copy)]
struct TileEntity;

#[derive(Resource)]
struct GameData {
    state: GameState,
    hub: Tilemap,
    hub_stairs: (f32, f32),
    dungeon: Tilemap,
    dungeon_stairs: (f32, f32),
}

#[derive(Clone)]
struct Tilemap {
    width: usize,
    height: usize,
    tiles: Vec<Vec<TileType>>,
}

impl Tilemap {
    fn new(width: usize, height: usize) -> Self {
        Tilemap {
            width,
            height,
            tiles: vec![vec![TileType::Grass; width]; height],
        }
    }

    fn spawn_hub(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.tiles[y][x] = TileType::Grass;
            }
        }

        for y in 5..12 {
            for x in 5..12 {
                self.tiles[y][x] = TileType::Stone;
            }
        }

        for y in 2..5 {
            for x in 12..16 {
                self.tiles[y][x] = TileType::Wood;
            }
        }

        self.tiles[4][12] = TileType::Door;

        // Stairs to dungeon - place at top right for visibility
        self.tiles[1][18] = TileType::Stairs;

        for y in 8..11 {
            for x in 1..3 {
                self.tiles[y][x] = TileType::Water;
            }
        }

        for y in 2..4 {
            for x in 2..4 {
                self.tiles[y][x] = TileType::Rock;
            }
        }
    }

    fn is_solid(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return true;
        }
        self.tiles[y][x].is_solid()
    }

    fn is_stairs(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.tiles[y][x] == TileType::Stairs
    }
}


fn main() {
    let mut hub = Tilemap::new(20, 15);
    hub.spawn_hub();
    // compute hub stairs world position
    let hub_stairs = ((18.0 - hub.width as f32 / 2.0) * TILE_SIZE + TILE_SIZE / 2.0,
                      (1.0 - hub.height as f32 / 2.0) * TILE_SIZE + TILE_SIZE / 2.0);

    let (dungeon, dungeon_stairs) = generate_dungeon();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RustLike".into(),
                resolution: (1280.0, 720.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(GameData {
            state: GameState::Hub,
            hub,
            hub_stairs,
            dungeon,
            dungeon_stairs,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, update_on_stairs))
        .run();
}

// Dungeon generation
#[derive(Clone, Copy)]
struct Room {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn generate_dungeon() -> (Tilemap, (f32,f32)) {
    let mut dungeon = Tilemap::new(DUNGEON_WIDTH, DUNGEON_HEIGHT);
    let mut rng = rand::thread_rng();

    // Fill with walls
    for y in 0..dungeon.height {
        for x in 0..dungeon.width {
            dungeon.tiles[y][x] = TileType::Wall;
        }
    }

    let mut rooms: Vec<Room> = Vec::new();

    // Generate 12 random rooms
    for _ in 0..12 {
        let width = rng.gen_range(5..12);
        let height = rng.gen_range(5..12);
        let x = rng.gen_range(1..dungeon.width - width - 1);
        let y = rng.gen_range(1..dungeon.height - height - 1);

        let new_room = Room { x, y, width, height };

        // Check collision with existing rooms
        let mut overlaps = false;
        for room in &rooms {
            if !(new_room.x + new_room.width < room.x
                || new_room.x > room.x + room.width
                || new_room.y + new_room.height < room.y
                || new_room.y > room.y + room.height)
            {
                overlaps = true;
                break;
            }
        }

        if !overlaps {
            // Carve out the room
            for ry in 0..new_room.height {
                for rx in 0..new_room.width {
                    let tile_y = new_room.y + ry;
                    let tile_x = new_room.x + rx;
                    if tile_x < dungeon.width && tile_y < dungeon.height {
                        dungeon.tiles[tile_y][tile_x] = TileType::Dirt;
                    }
                }
            }
            rooms.push(new_room);
        }
    }

    // Connect rooms with corridors
    for i in 1..rooms.len() {
        let prev_room = rooms[i - 1];
        let curr_room = rooms[i];

        let prev_center_x = prev_room.x + prev_room.width / 2;
        let prev_center_y = prev_room.y + prev_room.height / 2;
        let curr_center_x = curr_room.x + curr_room.width / 2;
        let curr_center_y = curr_room.y + curr_room.height / 2;

        // Horizontal corridor
        let min_x = prev_center_x.min(curr_center_x);
        let max_x = prev_center_x.max(curr_center_x);
        for x in min_x..=max_x {
            if x < dungeon.width && prev_center_y < dungeon.height {
                if dungeon.tiles[prev_center_y][x] == TileType::Wall {
                    dungeon.tiles[prev_center_y][x] = TileType::Dirt;
                }
            }
        }

        // Vertical corridor
        let min_y = prev_center_y.min(curr_center_y);
        let max_y = prev_center_y.max(curr_center_y);
        for y in min_y..=max_y {
            if curr_center_x < dungeon.width && y < dungeon.height {
                if dungeon.tiles[y][curr_center_x] == TileType::Wall {
                    dungeon.tiles[y][curr_center_x] = TileType::Dirt;
                }
            }
        }
    }

    // Place stairs in the last room
    let mut stair_world = (0.0,0.0);
    if let Some(last_room) = rooms.last() {
        let stairs_x = last_room.x + last_room.width / 2;
        let stairs_y = last_room.y + last_room.height / 2;
        if stairs_x < dungeon.width && stairs_y < dungeon.height {
            dungeon.tiles[stairs_y][stairs_x] = TileType::Stairs;
        }
        stair_world = ((stairs_x as f32 - dungeon.width as f32/2.0)*TILE_SIZE + TILE_SIZE/2.0,
                      (stairs_y as f32 - dungeon.height as f32/2.0)*TILE_SIZE + TILE_SIZE/2.0);
    }

    (dungeon, stair_world)
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_data: Res<GameData>,
) {
    // Camera
    // camera doesn't need scaling; the world itself is scaled via sprite transforms
    commands.spawn(Camera2dBundle::default());

    // Spawn tiles based on current state
    let tilemap = match game_data.state {
        GameState::Hub => &game_data.hub,
        GameState::Dungeon => &game_data.dungeon,
    };

    spawn_tilemap(&mut commands, &asset_server, tilemap);

    // spawn player at the origin
    let player_texture = asset_server.load("warrior.png");
    commands
        .spawn(SpriteBundle {
            texture: player_texture,
            transform: Transform::from_xyz(0.0, 0.0, 1.0)
                .with_scale(Vec3::splat(SCALE)),
            sprite: Sprite {
                anchor: Anchor::BottomCenter,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}

fn spawn_tilemap(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    tilemap: &Tilemap,
) {
    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            let tile_type = tilemap.tiles[y][x];
            let world_x = (x as f32 - tilemap.width as f32 / 2.0) * TILE_SIZE + TILE_SIZE / 2.0;
            let world_y = (y as f32 - tilemap.height as f32 / 2.0) * TILE_SIZE + TILE_SIZE / 2.0;

            let texture = asset_server.load(tile_type.asset_path());

            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: Vec3::new(world_x, world_y, 0.0),
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Tile { tile_type },
                TileEntity,
            ));
        }
    }
}


/// return true if an AABB centred at `cx,cy` with half extents `half` would
/// overlap any solid tile in `map`. coordinates are in world space.

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    if direction == Vec2::ZERO {
        return;
    }

    let tilemap = match game_data.state {
        GameState::Hub => &game_data.hub,
        GameState::Dungeon => &game_data.dungeon,
    };

    let speed = 150.0;
    let delta = time.delta_seconds();

    for mut transform in &mut query {
        let new_x = transform.translation.x + direction.x * speed * delta;
        let new_y = transform.translation.y + direction.y * speed * delta;

        // convert to tile indices; out-of-bounds should be treated as solid
        let tx = ((new_x / TILE_SIZE) + (tilemap.width as f32 / 2.0)).floor() as isize;
        let ty = ((new_y / TILE_SIZE) + (tilemap.height as f32 / 2.0)).floor() as isize;

        if tx >= 0 && ty >= 0 {
            let ux = tx as usize;
            let uy = ty as usize;
            if !tilemap.is_solid(ux, uy) {
                transform.translation.x = new_x;
                transform.translation.y = new_y;
            }
        }
    }
}

fn update_on_stairs(
    mut game_data: ResMut<GameData>,
    transform_query: Query<&Transform, With<Player>>,
    entity_query: Query<Entity, With<Player>>,
    tile_query: Query<Entity, With<TileEntity>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for transform in &transform_query {
        let (tile_x, tile_y, _tilemap_width) = {
            let tilemap = match game_data.state {
                GameState::Hub => &game_data.hub,
                GameState::Dungeon => &game_data.dungeon,
            };
            let x = ((transform.translation.x / TILE_SIZE) + (tilemap.width as f32 / 2.0)) as usize;
            let y = ((transform.translation.y / TILE_SIZE) + (tilemap.height as f32 / 2.0)) as usize;
            (x, y, tilemap.width)
        };

        let tilemap = match game_data.state {
            GameState::Hub => &game_data.hub,
            GameState::Dungeon => &game_data.dungeon,
        };

        if tilemap.is_stairs(tile_x, tile_y) {
            // Transition to new state
            let old_state = game_data.state;
            game_data.state = match game_data.state {
                GameState::Hub => GameState::Dungeon,
                GameState::Dungeon => GameState::Hub,
            };

            println!("Transitioned from {:?} to {:?}", old_state, game_data.state);

            // Despawn all tiles
            for entity in &tile_query {
                commands.entity(entity).despawn();
            }

            // Spawn new tilemap
            let new_tilemap = match game_data.state {
                GameState::Hub => &game_data.hub,
                GameState::Dungeon => &game_data.dungeon,
            };
            spawn_tilemap(&mut commands, &asset_server, new_tilemap);

            // teleport player to appropriate stairs location and nudge off
            if let Ok(player_entity) = entity_query.get_single() {
                let pos = match game_data.state {
                    GameState::Hub => game_data.hub_stairs,
                    GameState::Dungeon => game_data.dungeon_stairs,
                };
                let mut x = pos.0;
                let mut y = pos.1;
                // nudge player one tile away depending on new state
                match game_data.state {
                    GameState::Hub => x -= TILE_SIZE,     // move left when returning
                    GameState::Dungeon => x += TILE_SIZE, // move right when entering
                }
                commands
                    .entity(player_entity)
                    .insert(Transform::from_xyz(x, y, 1.0));
            }
        }
    }
}
