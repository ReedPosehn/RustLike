# CLAUDE.md — RustLike

This file gives Claude context about the project so it can assist effectively.

## Project Overview

RustLike is a top-down roguelike prototype in Rust using Bevy 0.11. Features: procedural seeded dungeons, AABB movement, real-time enemy AI, melee combat, character class selection, player death/respawn with gravestone, and a gradient HUD.

**Modules in `src/`:**
| File | Purpose |
|---|---|
| `main.rs` | Constants, resources, app entry, startup systems |
| `map.rs` | `TileKind`, `Tilemap`, `RoomInfo`, builders, `spawn_map`, `TileMarker`, tests |
| `player.rs` | `Player`, `spawn_player`, `player_movement` |
| `state.rs` | `AppState`, `MapState`, `GameWorld`, `LastDeathInfo`, transition systems |
| `character_select.rs` | `PlayerClass`, class selection screen, input handling |
| `enemies.rs` | `EnemyKind`, `Enemy`, `EnemyAi`, spawn/despawn/AI |
| `combat.rs` | `Health`, `Dead`, `Facing`, `DamageEvent`, `SplatEvent`, combat systems |
| `hud.rs` | `BarAssets`, gradient health bar, enemy bars, damage splats |
| `game_over.rs` | `Gold`, game-over screen, respawn logic |

## Build & Run

```bash
cargo run          # build and run
cargo test         # run 14 automated tests
python generate_assets.py   # regenerate all sprites (requires Pillow)
```

To replay a specific dungeon: set `FIXED_SEED = Some(n)` in `main.rs`.

## Application State Flow

```
CharacterSelect  →  Playing  →  GameOver  →  Playing (respawn)
                                   ↑ keeps MapState::Hub after respawn
```

- **`CharacterSelect`** (default) — class select screen over the hub map
- **`Playing`** — all gameplay systems active
- **`GameOver`** — overlay pauses gameplay; SPACE to respawn

**`MapState`** (sub-state, only meaningful in `Playing`):
- `Hub` (default) — hub map active
- `Dungeon` — dungeon map active, enemies alive

## Key Constants (`src/main.rs`)
| Constant | Value | Purpose |
|---|---|---|
| `TILE_PX` | 32.0 | Source PNG size |
| `SCALE` | 1.2 | Global sprite scale |
| `TILE` | 38.4 | World-space tile size |
| `HALF_W / HALF_H` | `TILE / 2.0` | Player/enemy AABB half-extents |
| `ENEMY_SPEED` | 150.0 | Base enemy speed (player speed is per-class) |
| `DUNGEON_W/H` | 33 × 18 | Fits exactly in 1280×720 at TILE=38.4 |
| `FIXED_SEED` | `Option<u64>` | Pin dungeon layout for testing |

## Player Classes (`src/character_select.rs`)
| Class | HP | ATK | Speed | Notes |
|---|---|---|---|---|
| Warrior | 100 | 25 | 150 | Balanced melee |
| Mage | 70 | 15 | 130 | Low HP; ranged stub |
| Paladin | 140 | 20 | 120 | Tank; slowest |
| Rogue | 80 | 20 | 190 | Fastest movement |

`PlayerClass` is both a `Resource` (chosen class) and a `Component` on the player entity. `player_movement` reads speed from it; `player_melee_attack` reads damage from it.

## Tilemap Coordinate System
- Row 0 = bottom, col 0 = left. Y-up matching Bevy world space.
- Tile centre: `x = (col - w/2)*TILE + TILE/2`, same for y.
- Inverse: `col = floor(px / TILE + w/2)` — **no TILE/2 offset** (adding it shifts the grid by half a tile and breaks collision).

## Collision (AABB)
`Anchor::Center` — translation = sprite centre. Both player and enemies use the same two-corner leading-face probes:
- X: `(face_x, py ± (HALF_H - 1))`
- Y: `(px ± (HALF_W - 1), face_y)`
Axes independent → wall sliding. Out-of-bounds = solid.

## Combat (`src/combat.rs`)
All damage flows through `DamageEvent { target, amount, source }`. `DamageSource` variants: `Melee`, `Contact`, `Ranged`*, `Magic`*, `AreaOfEffect`* (* = stubs for future systems). `apply_damage` drains HP, inserts `Dead`, fires `SplatEvent` for floating numbers.

## Enemy AI (`src/enemies.rs`)
`EnemyAi` state machine: `Pausing` → `Walking` → `Chasing`. Chase triggered at 5 tiles, lost at 7 (hysteresis). Uses same AABB collision as player. Per-enemy seeded RNG. Only runs in `MapState::Dungeon`.

## HUD (`src/hud.rs`)
- **Player bar** — gradient fill built in memory (`Assets<Image>::add()`), no async loading. Green→yellow→red as HP drops. 2px white border, rounded dark panel (`ui_panel.png`).
- **Enemy bars** — world-space sprites above each enemy; always green, shrink to show damage.
- **Damage splats** — yellow `-N` text, rises 28px/s, fades over 0.9s.
- `BarAssets` resource holds prebuilt gradient `Handle<Image>` handles.

## Death & Respawn (`src/game_over.rs`)
- `check_player_death` detects `Dead` on player → records `LastDeathInfo { gold_lost, seed }` → `AppState::GameOver`
- `setup_game_over` — dark overlay, "YOU DIED", gold lost, SPACE prompt
- `handle_respawn_input` — generates new seed, rebuilds dungeon in `GameWorld`, restores HP to `class.max_hp()`, removes `Dead`, transitions `MapState::Hub` + `AppState::Playing`
- `spawn_player` guards against duplicate spawns (returns early if `Player` entity exists)
- Gravestone spawned in first room of the NEW dungeon on `on_enter_dungeon` when `LastDeathInfo.seed > 0`

## Seed System
Seed printed at startup. `FIXED_SEED = Some(n)` to pin. `DungeonSeed` resource stores current seed. Enemy placement uses `seed + 1`; each enemy AI uses a further unique offset. New seed generated on each respawn.

## Known Issues & Caveats
- **No ranged/magic attacks** — `DamageSource` variants exist, systems not yet built
- **No inventory or items** — `Gold(u32)` stubbed at 0 on player
- **Enemies don't pathfind** — straight-line chase, can get stuck on corners
- **Enemies overlap each other** — no separation logic
- **Dungeon dimensions** must be recalculated if `SCALE` or resolution changes
- **No camera scrolling** — dungeon sized to window

## Roadmap
1. Modular codebase, Bevy States, seeds, tests (Phase 3)
2. Enemy AI, health/damage system, melee, contact damage
3. Gradient HUD, enemy health bars, damage splats
4. Player death / game-over / gravestone stub
5. Player class selection (Warrior, Mage, Paladin, Rogue)
6. **Next** — Ranged/magic attacks; inventory stub; dungeon depth