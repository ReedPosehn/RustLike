use bevy::prelude::*;
use rand::Rng;
use crate::{TILE, DUNGEON_W, DUNGEON_H};

// ─── tile types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Grass, Dirt, Stone, Wood, Water, Sand, Gravel, Rock, Wall, Door, Stairs,
}

impl TileKind {
    pub fn asset(self) -> &'static str {
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
    pub fn solid(self) -> bool {
        matches!(self, TileKind::Water | TileKind::Rock | TileKind::Wall)
    }
}

// ─── tilemap ─────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Tilemap {
    pub w: usize,
    pub h: usize,
    tiles: Vec<Vec<TileKind>>,
}

impl Tilemap {
    pub fn new(w: usize, h: usize, fill: TileKind) -> Self {
        Tilemap { w, h, tiles: vec![vec![fill; w]; h] }
    }

    pub fn get(&self, col: usize, row: usize) -> TileKind { self.tiles[row][col] }
    pub fn set(&mut self, col: usize, row: usize, kind: TileKind) { self.tiles[row][col] = kind; }

    /// World-space centre of tile (col, row).
    /// col 0 is the leftmost column; row 0 is the bottom row.
    pub fn tile_center(&self, col: usize, row: usize) -> Vec2 {
        Vec2::new(
            (col as f32 - self.w as f32 / 2.0) * TILE + TILE / 2.0,
            (row as f32 - self.h as f32 / 2.0) * TILE + TILE / 2.0,
        )
    }

    /// Returns the (col, row) of the tile that contains world point `p`,
    /// or None if `p` is outside the map.
    pub fn world_to_tile(&self, p: Vec2) -> Option<(usize, usize)> {
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
    pub fn solid_at(&self, p: Vec2) -> bool {
        self.world_to_tile(p).map_or(true, |(c, r)| self.get(c, r).solid())
    }

    /// True if world point `p` is inside a stair tile.
    pub fn stairs_at(&self, p: Vec2) -> bool {
        self.world_to_tile(p).map_or(false, |(c, r)| self.get(c, r) == TileKind::Stairs)
    }
}

// ─── ECS marker ──────────────────────────────────────────────────────────────

/// Tags every spawned tile sprite so they can be bulk-despawned on transitions.
#[derive(Component)]
pub struct TileMarker;

/// Spawn all tile sprites for `map` into the world.
pub fn spawn_map(commands: &mut Commands, asset_server: &AssetServer, map: &Tilemap) {
    use crate::SCALE;
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

// ─── map builders ────────────────────────────────────────────────────────────

pub fn build_hub() -> (Tilemap, Vec2) {
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

pub fn build_dungeon() -> (Tilemap, Vec2) {
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