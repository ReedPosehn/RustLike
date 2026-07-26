# CLAUDE.md — RustLike

This file gives Claude context about the project so it can assist effectively.

## Project Overview

RustLike is a top-down roguelike prototype in Rust using Bevy 0.11. Features: procedural seeded dungeons, AABB movement, real-time enemy AI, melee + ranged/magic combat, character class selection, player death/respawn with gravestone, a gradient HUD, and a pause menu.

**Modules in `src/`:**
| File | Purpose |
|---|---|
| `main.rs` | Constants, resources, app entry, startup systems |
| `map.rs` | `TileKind`, `Tilemap`, `RoomInfo`, builders, `spawn_map`, `TileMarker`, tests |
| `player.rs` | `Player`, `spawn_player`, `player_movement` |
| `state.rs` | `AppState`, `MapState`, `GameWorld`, `LastDeathInfo`, transition systems |
| `character_select.rs` | `PlayerClass`, `AttackKind`, class selection screen |
| `difficulty.rs` | `Difficulty` (Standard/Permadeath), difficulty selection screen |
| `enemies.rs` | `EnemyKind`, `Enemy`, `EnemyAi`, spawn/despawn/AI |
| `combat.rs` | `Health`, `Dead`, `Facing`, `DamageEvent`, `SplatEvent`, combat systems |
| `projectile.rs` | `Projectile`, ranged/magic attack firing and movement |
| `hud.rs` | `BarAssets`, gradient health bar, enemy bars, damage splats |
| `game_over.rs` | `Gold`, game-over screen, respawn logic |
| `pause.rs` | Pause menu, controls reference, options stub |

## Build & Run

```bash
cargo run          # build and run
cargo test         # run 14 automated tests
python generate_assets.py   # regenerate all sprites (requires Pillow)
```

To replay a specific dungeon: set `FIXED_SEED = Some(n)` in `main.rs`.

## Application State Flow

```
CharacterSelect → DifficultySelect → Playing ⇄ Paused
        ↑                               ↓
        └──────── GameOver (Permadeath) ┤
                                         └→ Playing (Standard: respawn, new seed)
```

- **`CharacterSelect`** (default) — class select screen over the hub map
- **`DifficultySelect`** — Standard vs. Permadeath select screen, entered right after confirming a class
- **`Playing`** — all gameplay systems active
- **`Paused`** — Esc toggles from `Playing`; freezes gameplay, shows controls reference + a partially-live options panel (difficulty toggle)
- **`GameOver`** — overlay pauses gameplay; SPACE respawns in place (Standard) or starts a brand new game via `CharacterSelect` (Permadeath)

**`MapState`** (sub-state, only meaningful in `Playing`):
- `Hub` (default) — hub map active
- `Dungeon` — dungeon map active, enemies alive

**Important gating pattern:** dungeon-only combat systems (`enemy_ai`, `player_melee_attack`, `enemy_contact_damage`, `fire_ranged_attack`, `update_projectiles`, `despawn_dead_enemies`) require **both** `in_state(MapState::Dungeon)` **and** `in_state(AppState::Playing)` via `.and_then(...)`. This matters because dying or pausing only changes `AppState`, not `MapState` — without the combined condition, enemies/projectiles would keep acting during the game-over or pause screen.

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
| Class | HP | Melee | Ranged | Speed | Attack Kind |
|---|---|---|---|---|---|
| Warrior | 100 | 25 | 12 | 150 | Ranged (arrow) |
| Mage | 70 | 15 | 30 | 130 | Magic (bolt) |
| Paladin | 140 | 20 | 10 | 120 | Ranged (arrow) |
| Rogue | 80 | 20 | 18 | 190 | Ranged (arrow) |

`PlayerClass` is both a `Resource` (chosen class) and a `Component` on the player entity. `player_movement` reads speed from it; `player_melee_attack` and `fire_ranged_attack` read damage from it. Only Mage uses `AttackKind::Magic` (purple bolt); the rest fire a physical arrow.

## Controls
| Key | Action |
|---|---|
| WASD / Arrows | Move |
| F | Melee attack (facing direction) |
| SPACE | Ranged/magic attack (facing direction) |
| ESC | Pause / resume |
| ← → or A/D (CharacterSelect) | Browse classes |
| ENTER / SPACE (CharacterSelect) | Confirm class |
| ← → or A/D (DifficultySelect) | Browse Standard / Permadeath |
| ENTER / SPACE (DifficultySelect) | Confirm difficulty |
| TAB (Paused) | Toggle difficulty |
| SPACE (GameOver) | Respawn (Standard) / start new game (Permadeath) |

## Tilemap Coordinate System
- Row 0 = bottom, col 0 = left. Y-up matching Bevy world space.
- Tile centre: `x = (col - w/2)*TILE + TILE/2`, same for y.
- Inverse: `col = floor(px / TILE + w/2)` — **no TILE/2 offset** (adding it shifts the grid by half a tile and breaks collision).

## Collision (AABB)
`Anchor::Center` — translation = sprite centre. Player and enemies use two-corner leading-face probes on independent X/Y axes (wall sliding). Out-of-bounds = solid.

## Combat (`src/combat.rs`, `src/projectile.rs`)
All damage flows through `DamageEvent { target, amount, source }`. `DamageSource`: `Melee`, `Contact`, `Ranged`, `Magic`, `AreaOfEffect`* (* = stub). `apply_damage` drains HP, inserts `Dead`, fires `SplatEvent`.

**Targeting bug fixed:** enemy queries used to filter by `EnemyMarker`, which is *also* tagged on the enemy health bar sprites (so they could be a silent miss target). All combat-targeting queries (`player_melee_attack`, `enemy_contact_damage`, `projectile` hit detection, `despawn_dead_enemies`) now filter by the `Enemy` component instead, which only exists on the real enemy entity.

**Projectiles** (`projectile.rs`): `fire_ranged_attack` (SPACE) spawns a sprite ahead of the player, rotated to face direction. `update_projectiles` is a single combined system — movement, wall collision, enemy collision, lifetime expiry all in one pass, so an entity is never double-despawned by separate systems racing. `despawn_projectiles` runs on `OnExit(MapState::Dungeon)` to clean up any in-flight projectile when leaving the dungeon (by stairs or by dying) — without this they'd freeze on screen forever since their own update system stops running outside Dungeon+Playing.

**Dead enemy cleanup:** `despawn_dead_enemies` now also despawns any `EnemyBarFor`-tagged health bar entities tracking the dead enemy — previously the bars were separate entities that never received `Dead` themselves, so they were left behind frozen at the enemy's last position.

## Enemy AI (`src/enemies.rs`)
`EnemyAi` state machine: `Pausing` → `Walking` → `Chasing`. Chase at 5 tiles, lose at 7 (hysteresis). Same AABB collision as player. Per-enemy seeded RNG. Only runs in `MapState::Dungeon` (and now also gated to `AppState::Playing`, see above).

## HUD (`src/hud.rs`)
- **Player bar** — gradient fill built in memory (`Assets<Image>::add()`, no async load). Green→yellow→red. 2px white border, rounded dark panel.
- **Enemy bars** — world-space sprites; always green, shrink to show damage; properly cleaned up on enemy death (see Combat above).
- **Damage splats** — yellow `-N` text, rises and fades.

## Death & Respawn (`src/game_over.rs`)
- `check_player_death` → `LastDeathInfo { gold_lost, seed }` → `AppState::GameOver`
- `handle_respawn_input` — new seed, rebuild dungeon, restore HP to `class.max_hp()`, remove `Dead`, → `MapState::Hub` + `AppState::Playing`
- `spawn_player` guards against duplicate spawns (checks for existing `Player` entity)
- Gravestone spawns in first room of the new dungeon when `LastDeathInfo.seed > 0`

## Pause Menu (`src/pause.rs`)
Two one-directional systems (`toggle_pause_on_escape` in `Playing`, `handle_pause_input` in `Paused`) rather than a single toggle — avoids any chance of double-firing in one frame. Shows a controls reference (two-column flexbox layout — Bevy's default font isn't monospace so text padding wouldn't align) and an Options panel. Difficulty is now live — TAB (`handle_pause_options_input`) flips the `Difficulty` resource and rebuilds the whole pause menu tree (`build_pause_menu`, shared with `setup_pause_menu`) to reflect it. It rebuilds the whole tree rather than patching just the difficulty text node because Bevy 0.11 doesn't clean up a parent's `Children` list when a single child is despawned with plain `despawn()` (only `despawn_recursive` does) — swapping one line in place left a dangling reference that panicked the UI clipping system. Volume remains a grayed-out stub. Because all gameplay systems are gated by `AppState::Playing`, pausing required no changes to any existing system — they simply stop running.

## Difficulty (`src/difficulty.rs`)
`Difficulty` (Standard / Permadeath) is a `Resource` set via the `DifficultySelect` screen (mirrors `character_select.rs`'s card-based UI and rebuild-on-navigate pattern) right after class selection, and can be flipped later from the pause menu (see above). `check_player_death`/`setup_game_over`/`handle_respawn_input` (`game_over.rs`) read it to branch death behavior: **Standard** respawns the existing player in place with a new dungeon seed and a full heal (unchanged from before); **Permadeath** despawns the player entity outright and routes back to `AppState::CharacterSelect`, so the next class/difficulty pick spawns a genuinely new character (`spawn_player` already guards on "no existing `Player` entity").

## Seed System
Seed printed at startup. `FIXED_SEED = Some(n)` to pin. `DungeonSeed` resource stores current seed. Enemy placement uses `seed + 1`. New seed generated on each respawn.

## Known Issues & Caveats
- **No inventory or items** — `Gold(u32)` stubbed at 0 on player
- **Enemies don't pathfind** — straight-line chase, can get stuck on corners
- **Enemies overlap each other** — no separation logic
- **Dungeon dimensions** must be recalculated if `SCALE` or resolution changes
- **No camera scrolling** — dungeon sized to window
- **Volume option is a visual stub only** — Difficulty is now live (TAB in the pause menu), Volume is not

## Roadmap
1. Modular codebase, Bevy States, seeds, tests (Phase 3)
2. Enemy AI, health/damage system, melee, contact damage
3. Gradient HUD, enemy health bars, damage splats
4. Player death / game-over / gravestone stub
5. Player class selection (Warrior, Mage, Paladin, Rogue)
6. Ranged/magic attacks (SPACE), class-based projectile type
7. Pause menu with controls reference + options stub
8. **Done** — Difficulty selector (permadeath vs. standard respawn), a `DifficultySelect` state between `CharacterSelect` and `Playing`, with the toggle also live in the pause menu's Options panel
9. **Next** — Inventory stub, dungeon depth/difficulty scaling