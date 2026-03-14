use bevy::prelude::*;
use rand::Rng;

// ─── constants ───────────────────────────────────────────────────────────────

const TILE_PX: f32 = 32.0;  // source PNG size
const SCALE:   f32 = 1.2;   // global sprite scale
const TILE:    f32 = TILE_PX * SCALE; // world-space tile size (38.4)

/// Player AABB half-extents. Exactly half a tile so the hitbox matches the
/// sprite edges. The 1px corner insets in collision probes prevent
/// false positives on exact tile boundaries.
const HALF_W: f32 = TILE / 2.0;
const HALF_H: f32 = TILE / 2.0;

const SPEED: f32 = 150.0; // world units per second

const DUNGEON_W: usize = 40;
const DUNGEON_H: usize = 30;

// ─── tile types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileKind {
    Grass, Dirt, Stone, Wood, Water, Sand, Gravel, Rock, Wall, Door, Stairs,
}

impl TileKind {
    fn asset(self) -> &'static str {
        match self {
            TileKind::Grass   => "grass.png",
            TileKind::Dirt    => "dirt.png",
            TileKind::Stone   => "stone.png",
            TileKind::Wood    => "wood.png",
            TileKind::Water   => "water.png",
            TileKind::Sand    => "sand.png",
            TileKind::Gravel  => "gravel.png",
            TileKind::Rock    => "rock.png",
            TileKind::Wall    => "wall.png",
            TileKind::Door    => "door.png",
            TileKind::Stairs  => "gravel.png",
        }
    }
    fn solid(self) -> bool {
        matches!(self, TileKind::Water | TileKind::Rock | TileKind::Wall)
    }
}

// ─── tilemap ─────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct Tilemap {
    w: usize,
    h: usize,
    tiles: Vec<Vec<TileKind>>,
}

impl Tilemap {
    fn new(w: usize, h: usize, fill: TileKind) -> Self {
        Tilemap { w, h, tiles: vec![vec![fill; w]; h] }
    }

    fn get(&self, col: usize, row: usize) -> TileKind { self.tiles[row][col] }
    fn set(&mut self, col: usize, row: usize, kind: TileKind) { self.tiles[row][col] = kind; }

    /// World-space centre of tile (col, row).
    /// col 0 is the leftmost column; row 0 is the bottom row.
    fn tile_center(&self, col: usize, row: usize) -> Vec2 {
        Vec2::new(
            (col as f32 - self.w as f32 / 2.0) * TILE + TILE / 2.0,
            (row as f32 - self.h as f32 / 2.0) * TILE + TILE / 2.0,
        )
    }

    /// Returns the (col, row) of the tile that contains world point `p`,
    /// or None if `p` is outside the map.
    fn world_to_tile(&self, p: Vec2) -> Option<(usize, usize)> {
        // Tile C occupies x in [(C - w/2)*TILE, (C+1 - w/2)*TILE).
        // Inverting: col = floor(px / TILE + w/2)
        let col = (p.x / TILE + self.w as f32 / 2.0).floor() as isize;
        let row = (p.y / TILE + self.h as f32 / 2.0).floor() as isize;
        if col >= 0 && row >= 0 && (col as usize) < self.w && (row as usize) < self.h {
            Some((col as usize, row as usize))
        } else {
            None
        }
    }

    /// True if world point `p` is inside a solid tile (or outside the map).
    fn solid_at(&self, p: Vec2) -> bool {
        self.world_to_tile(p).map_or(true, |(c, r)| self.get(c, r).solid())
    }

    /// True if world point `p` is inside a stair tile.
    fn stairs_at(&self, p: Vec2) -> bool {
        self.world_to_tile(p).map_or(false, |(c, r)| self.get(c, r) == TileKind::Stairs)
    }
}

// ─── map builders ────────────────────────────────────────────────────────────

fn build_hub() -> (Tilemap, Vec2) {
    let mut m = Tilemap::new(20, 15, TileKind::Grass);
    for r in 5..12 { for c in 5..12  { m.set(c, r, TileKind::Stone); } }
    for r in 2..5  { for c in 12..16 { m.set(c, r, TileKind::Wood);  } }
    m.set(12, 4, TileKind::Door);
    for r in 8..11 { for c in 1..3   { m.set(c, r, TileKind::Water); } }
    for r in 2..4  { for c in 2..4   { m.set(c, r, TileKind::Rock);  } }
    let (sc, sr) = (18usize, 1usize);
    m.set(sc, sr, TileKind::Stairs);
    (m.clone(), m.tile_center(sc, sr))
}

struct Room { x: usize, y: usize, w: usize, h: usize }
impl Room {
    fn center(&self) -> (usize, usize) { (self.x + self.w / 2, self.y + self.h / 2) }
    fn overlaps(&self, o: &Room) -> bool {
        !(self.x + self.w <= o.x || o.x + o.w <= self.x
          || self.y + self.h <= o.y || o.y + o.h <= self.y)
    }
}

fn build_dungeon() -> (Tilemap, Vec2) {
    let mut m = Tilemap::new(DUNGEON_W, DUNGEON_H, TileKind::Wall);
    let mut rng = rand::thread_rng();
    let mut rooms: Vec<Room> = Vec::new();

    for _ in 0..40 {
        let rw = rng.gen_range(5..12);
        let rh = rng.gen_range(4..9);
        let rx = rng.gen_range(1..DUNGEON_W.saturating_sub(rw + 1));
        let ry = rng.gen_range(1..DUNGEON_H.saturating_sub(rh + 1));
        let room = Room { x: rx, y: ry, w: rw, h: rh };
        if !rooms.iter().any(|r| r.overlaps(&room)) {
            for row in room.y..room.y + room.h {
                for col in room.x..room.x + room.w {
                    m.set(col, row, TileKind::Dirt);
                }
            }
            rooms.push(room);
        }
        if rooms.len() >= 12 { break; }
    }

    // Connect rooms with L-shaped corridors
    for i in 1..rooms.len() {
        let (ax, ay) = rooms[i - 1].center();
        let (bx, by) = rooms[i].center();
        let (lx, hx) = if ax < bx { (ax, bx) } else { (bx, ax) };
        for col in lx..=hx { m.set(col, ay, TileKind::Dirt); }
        let (ly, hy) = if ay < by { (ay, by) } else { (by, ay) };
        for row in ly..=hy { m.set(bx, row, TileKind::Dirt); }
    }

    // Place stairs at the centre of the last room
    let stair_pos = if let Some(last) = rooms.last() {
        let (sc, sr) = last.center();
        m.set(sc, sr, TileKind::Stairs);
        m.tile_center(sc, sr)
    } else {
        Vec2::ZERO
    };

    (m, stair_pos)
}

// ─── ECS types ───────────────────────────────────────────────────────────────

#[derive(Component)] struct Player;
#[derive(Component)] struct TileMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapState { Hub, Dungeon }

#[derive(Resource)]
struct World {
    state:          MapState,
    hub:            Tilemap,
    hub_stairs:     Vec2,
    dungeon:        Tilemap,
    dungeon_stairs: Vec2,
}
impl World {
    fn current(&self) -> &Tilemap {
        match self.state { MapState::Hub => &self.hub, MapState::Dungeon => &self.dungeon }
    }
}

/// Counts down each frame after a stair transition; the transition cannot
/// re-fire until it reaches zero. 60 frames ≈ 1 s at 60 fps.
#[derive(Resource)] struct StairCooldown(u32);
const COOLDOWN_FRAMES: u32 = 60;

// ─── main ────────────────────────────────────────────────────────────────────

fn main() {
    let (hub,     hub_stairs)     = build_hub();
    let (dungeon, dungeon_stairs) = build_dungeon();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RustLike".into(),
                resolution: (1280.0, 720.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(World { state: MapState::Hub, hub, hub_stairs, dungeon, dungeon_stairs })
        .insert_resource(StairCooldown(0))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, stair_transition))
        .run();
}

// ─── systems ─────────────────────────────────────────────────────────────────

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, world: Res<World>) {
    commands.spawn(Camera2dBundle::default());
    spawn_map(&mut commands, &asset_server, world.current());

    // Spawn the player at the hub centre tile (10, 7), away from the stair tile.
    let start = world.hub.tile_center(10, 7);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("warrior.png"),
            transform: Transform::from_xyz(start.x, start.y, 1.0)
                .with_scale(Vec3::splat(SCALE)),
            ..Default::default()
        },
        Player,
    ));
}

fn spawn_map(commands: &mut Commands, asset_server: &AssetServer, map: &Tilemap) {
    for row in 0..map.h {
        for col in 0..map.w {
            let kind = map.get(col, row);
            let pos  = map.tile_center(col, row);
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(kind.asset()),
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0)
                        .with_scale(Vec3::splat(SCALE)),
                    ..Default::default()
                },
                TileMarker,
            ));
        }
    }
}

// AABB collision — Anchor::Center means translation = sprite centre.
// Player hitbox: centre ± (HALF_W, HALF_H).
// X and Y axes are resolved independently so the player slides along walls.
// Two corners are tested on each leading face; the 1px inset prevents
// false positives when a corner lands exactly on a tile boundary.
fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    time:     Res<Time>,
    world:    Res<World>,
    mut q:    Query<&mut Transform, With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up)    { dir.y += 1.0; }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down)  { dir.y -= 1.0; }
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left)  { dir.x -= 1.0; }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) { dir.x += 1.0; }
    if dir == Vec2::ZERO { return; }

    let map = world.current();
    let dt  = time.delta_seconds();

    for mut t in &mut q {
        let px = t.translation.x;
        let py = t.translation.y;

        // X axis
        let dx = dir.x * SPEED * dt;
        if dx != 0.0 {
            let nx     = px + dx;
            let face_x = if dx > 0.0 { nx + HALF_W } else { nx - HALF_W };
            let blocked = map.solid_at(Vec2::new(face_x, py - HALF_H + 1.0))
                       || map.solid_at(Vec2::new(face_x, py + HALF_H - 1.0));
            if !blocked { t.translation.x = nx; }
        }

        // Y axis — use the updated x so diagonal corners are handled correctly
        let px = t.translation.x;
        let dy = dir.y * SPEED * dt;
        if dy != 0.0 {
            let ny     = py + dy;
            let face_y = if dy > 0.0 { ny + HALF_H } else { ny - HALF_H };
            let blocked = map.solid_at(Vec2::new(px - HALF_W + 1.0, face_y))
                       || map.solid_at(Vec2::new(px + HALF_W - 1.0, face_y));
            if !blocked { t.translation.y = ny; }
        }
    }
}

fn stair_transition(
    mut world:    ResMut<World>,
    mut cooldown: ResMut<StairCooldown>,
    p_query:      Query<(Entity, &Transform), With<Player>>,
    tile_query:   Query<Entity, With<TileMarker>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if cooldown.0 > 0 { cooldown.0 -= 1; return; }

    let Ok((player_e, transform)) = p_query.get_single() else { return };
    let centre = Vec2::new(transform.translation.x, transform.translation.y);
    if !world.current().stairs_at(centre) { return; }

    world.state = match world.state {
        MapState::Hub     => MapState::Dungeon,
        MapState::Dungeon => MapState::Hub,
    };

    for e in &tile_query { commands.entity(e).despawn(); }
    spawn_map(&mut commands, &asset_server, world.current());

    // Spawn one tile above the destination stairs — always inside the room
    // interior and guaranteed to be open ground.
    let dest = match world.state {
        MapState::Hub     => world.hub_stairs,
        MapState::Dungeon => world.dungeon_stairs,
    };
    commands.entity(player_e).insert(
        Transform::from_xyz(dest.x, dest.y + TILE, 1.0).with_scale(Vec3::splat(SCALE))
    );

    cooldown.0 = COOLDOWN_FRAMES;
}