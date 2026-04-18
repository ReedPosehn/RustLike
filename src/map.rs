use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
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

/// Build a dungeon deterministically from `seed`.
/// Pass the same seed to reproduce the exact same layout.
/// The seed is printed to stdout at startup so you can record it.
pub fn build_dungeon(seed: u64) -> (Tilemap, Vec2) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut m   = Tilemap::new(DUNGEON_W, DUNGEON_H, TileKind::Wall);
    let mut rooms: Vec<Room> = Vec::new();

    for _ in 0..40 {
        // Room sizes tuned for the 33×18 dungeon footprint.
        let rw = rng.gen_range(4..9);
        let rh = rng.gen_range(3..6);
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

// ─── tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TILE;

    // ── Tilemap coordinate helpers ───────────────────────────────────────────

    /// tile_center and world_to_tile must be exact inverses of each other.
    /// For every tile in a small map, converting to world space and back
    /// must return the original (col, row).
    #[test]
    fn tile_center_and_world_to_tile_are_inverses() {
        let map = Tilemap::new(33, 18, TileKind::Grass);
        for row in 0..map.h {
            for col in 0..map.w {
                let world = map.tile_center(col, row);
                let result = map.world_to_tile(world);
                assert_eq!(
                    result,
                    Some((col, row)),
                    "round-trip failed for ({col},{row}): world={world:?} -> {result:?}"
                );
            }
        }
    }

    /// Points just inside each edge of a tile must resolve to that tile.
    #[test]
    fn world_to_tile_interior_points() {
        let map = Tilemap::new(10, 10, TileKind::Grass);
        let col = 5usize;
        let row = 5usize;
        let center = map.tile_center(col, row);
        let inset  = 0.1;

        // All four near-edge interior points should map back to (col, row).
        let points = [
            Vec2::new(center.x - TILE / 2.0 + inset, center.y),               // near left edge
            Vec2::new(center.x + TILE / 2.0 - inset, center.y),               // near right edge
            Vec2::new(center.x, center.y - TILE / 2.0 + inset),               // near bottom edge
            Vec2::new(center.x, center.y + TILE / 2.0 - inset),               // near top edge
        ];
        for p in points {
            assert_eq!(map.world_to_tile(p), Some((col, row)), "point {p:?} did not resolve to ({col},{row})");
        }
    }

    /// Points outside the map bounds must return None.
    #[test]
    fn world_to_tile_out_of_bounds_returns_none() {
        let map = Tilemap::new(10, 10, TileKind::Grass);
        // Compute a point well outside the map extents.
        let far = (map.w as f32) * TILE * 2.0;
        assert_eq!(map.world_to_tile(Vec2::new( far,  0.0)), None);
        assert_eq!(map.world_to_tile(Vec2::new(-far,  0.0)), None);
        assert_eq!(map.world_to_tile(Vec2::new( 0.0,  far)), None);
        assert_eq!(map.world_to_tile(Vec2::new( 0.0, -far)), None);
    }

    /// solid_at must return true for out-of-bounds points (treated as walls).
    #[test]
    fn solid_at_out_of_bounds_is_true() {
        let map = Tilemap::new(10, 10, TileKind::Grass);
        let far = (map.w as f32) * TILE * 2.0;
        assert!(map.solid_at(Vec2::new(far, 0.0)));
        assert!(map.solid_at(Vec2::new(0.0, -far)));
    }

    /// solid_at must correctly distinguish solid from open tiles.
    #[test]
    fn solid_at_matches_tile_kind() {
        let mut map = Tilemap::new(5, 5, TileKind::Grass);
        map.set(2, 2, TileKind::Wall);
        let wall_pos  = map.tile_center(2, 2);
        let grass_pos = map.tile_center(1, 1);
        assert!(map.solid_at(wall_pos),   "Wall tile should be solid");
        assert!(!map.solid_at(grass_pos), "Grass tile should not be solid");
    }

    // ── Dungeon generation ───────────────────────────────────────────────────

    /// A freshly generated dungeon must contain at least one room (Dirt tile).
    #[test]
    fn dungeon_contains_at_least_one_room() {
        let (map, _) = build_dungeon(42);
        let has_floor = (0..map.h).any(|r| (0..map.w).any(|c| map.get(c, r) == TileKind::Dirt));
        assert!(has_floor, "Dungeon should contain at least one Dirt (floor) tile");
    }

    /// The dungeon must contain exactly one stair tile.
    #[test]
    fn dungeon_has_exactly_one_stair() {
        let (map, _) = build_dungeon(42);
        let stair_count = (0..map.h)
            .flat_map(|r| (0..map.w).map(move |c| (c, r)))
            .filter(|&(c, r)| map.get(c, r) == TileKind::Stairs)
            .count();
        assert_eq!(stair_count, 1, "Dungeon should have exactly 1 stair tile, found {stair_count}");
    }

    /// The stair tile must be walkable (not solid).
    #[test]
    fn dungeon_stair_is_not_solid() {
        let (map, stair_world) = build_dungeon(42);
        assert!(
            !map.solid_at(stair_world),
            "Stair world position should not be solid"
        );
    }

    /// The stair world position returned by build_dungeon must resolve back
    /// to the actual stair tile.
    #[test]
    fn dungeon_stair_world_pos_matches_tile() {
        let (map, stair_world) = build_dungeon(42);
        let tile = map.world_to_tile(stair_world);
        assert!(tile.is_some(), "Stair world pos should be within map bounds");
        let (c, r) = tile.unwrap();
        assert_eq!(
            map.get(c, r),
            TileKind::Stairs,
            "world_to_tile(stair_world) should point to a Stair tile, got {:?}",
            map.get(c, r)
        );
    }

    /// Dungeon generation must be deterministic — same seed, same layout.
    #[test]
    fn dungeon_is_deterministic() {
        let (map_a, stairs_a) = build_dungeon(99999);
        let (map_b, stairs_b) = build_dungeon(99999);
        assert_eq!(stairs_a, stairs_b, "Stair positions should match for same seed");
        for r in 0..map_a.h {
            for c in 0..map_a.w {
                assert_eq!(
                    map_a.get(c, r), map_b.get(c, r),
                    "Tile ({c},{r}) differs between two runs with the same seed"
                );
            }
        }
    }

    /// Different seeds must produce different layouts (with very high probability).
    #[test]
    fn different_seeds_produce_different_dungeons() {
        let (map_a, _) = build_dungeon(1);
        let (map_b, _) = build_dungeon(2);
        let any_difference = (0..map_a.h).any(|r| {
            (0..map_a.w).any(|c| map_a.get(c, r) != map_b.get(c, r))
        });
        assert!(any_difference, "Different seeds should produce different dungeons");
    }

    /// All stair tiles must be within the map boundary.
    #[test]
    fn dungeon_stair_within_bounds() {
        let (map, stair_world) = build_dungeon(42);
        assert!(
            map.world_to_tile(stair_world).is_some(),
            "Stair world position must be within map bounds"
        );
    }

    // ── Hub ──────────────────────────────────────────────────────────────────

    /// The hub stair tile must exist and be non-solid.
    #[test]
    fn hub_stair_is_not_solid() {
        let (map, stair_world) = build_hub();
        assert!(!map.solid_at(stair_world), "Hub stair should not be solid");
    }

    /// The hub stair world position must resolve to a Stairs tile.
    #[test]
    fn hub_stair_world_pos_matches_tile() {
        let (map, stair_world) = build_hub();
        let tile = map.world_to_tile(stair_world);
        assert!(tile.is_some(), "Hub stair world pos should be within bounds");
        let (c, r) = tile.unwrap();
        assert_eq!(map.get(c, r), TileKind::Stairs);
    }
}