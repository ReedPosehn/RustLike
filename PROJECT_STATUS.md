# RustLike Project Status

## 🚧 Current Context

- **Language & Framework:** Rust + Bevy 0.11
- **Modules:** `main`, `map`, `player`, `state`, `character_select`, `enemies`, `combat`, `hud`, `game_over` (9 source files)
- **Assets:** 32×32 PNGs from `generate_assets.py` (Pillow)

## ✅ Features Implemented

1. **Tilemap & Dungeon** — Seed-based rooms, L-corridors, stairs. Fits 1280×720.
2. **AABB Movement** — Wall sliding, independent X/Y axes, per-class speed.
3. **Bevy States** — `AppState` (CharacterSelect → Playing → GameOver) + `MapState` (Hub/Dungeon). `OnEnter`/`OnExit` handle all lifecycle.
4. **Character Selection** — 4 classes (Warrior, Mage, Paladin, Rogue). Card-based UI. Different HP, ATK, speed per class.
5. **Enemy Spawning & AI** — Goblins, orcs, skeletons. Wander/chase state machine with hysteresis. Wall-aware AABB.
6. **Combat Foundation** — `DamageEvent` system. All damage flows through one event.
7. **Player Melee** — F key, facing direction, class-based damage.
8. **Enemy Contact Damage** — 0.8s cooldown timer.
9. **HUD** — Gradient health bar (in-memory, no async load), white border, rounded dark panel. Enemy bars. Damage splats.
10. **Player Death & Respawn** — `GameOver` state, "YOU DIED" screen, SPACE to respawn with new dungeon seed.
11. **Gravestone** — Spawns in first room of new dungeon on respawn. `LastDeathInfo` stores gold/seed.
12. **Gold Stub** — `Gold(u32)` component on player (always 0 until economy is built).
13. **Automated Tests** — 14 tests in `map.rs`.

## 📂 Source Structure

```
src/
├── main.rs              — constants, resources, app entry, startup systems
├── map.rs               — TileKind, Tilemap, builders, spawn_map, tests
├── player.rs            — Player, spawn_player (class-aware), player_movement
├── state.rs             — AppState, MapState, GameWorld, LastDeathInfo, transitions
├── character_select.rs  — PlayerClass, class select screen, input handling
├── enemies.rs           — EnemyKind, Enemy, EnemyAi, spawn/despawn/AI
├── combat.rs            — Health, DamageEvent, Facing, combat systems
├── hud.rs               — BarAssets, health bars, damage splats
└── game_over.rs         — Gold, game-over screen, respawn logic
```

## 🛠 Recent Work

### Session — Character select + death polish (current)
- **Character select screen** — 4 class cards, keyboard navigation, gold border on selected. Left/Right to browse, Enter/Space to confirm.
- **`PlayerClass`** as both Resource and Component. Speed, HP, melee damage all driven by class.
- **`AppState::CharacterSelect`** as new default. Hub map visible behind the screen.
- **`spawn_player`** moved to `OnEnter(AppState::Playing)` with guard against duplicate spawns on respawn.
- **`SPEED` → `ENEMY_SPEED`** — constant renamed to clarify it only applies to enemies.
- **Warning cleanup** — removed unused `random_dir`, `PLAYER_MELEE_DAMAGE`, `stairs_for`; added `#[allow(dead_code)]` on intentional stubs (`DamageSource` variants, `Enemy::kind`).

### Session — Death, respawn, gravestone
- `GameOver` state with dark overlay UI.
- `Gold(u32)` stub on player, shown on death screen.
- Respawn regenerates dungeon with new seed; gravestone in first room.

### Session — HUD & combat
- Gradient health bar built in Rust memory (no PNG, no async load issues).
- Enemy health bars (green, shrink on damage). Damage splats.
- `DamageEvent` / `SplatEvent` pipeline.

## 📌 Known Issues

- No ranged/magic attacks yet (`DamageSource` variants stubbed).
- No inventory or item drops.
- Enemies don't pathfind — can get stuck on corners.
- Dungeon sizing hardcoded to window resolution.

## 🔮 Roadmap

### ✅ Phase 1–3 — Foundation, collision, architecture, tests
### ⚔️ Phase 4 — Enemies & Combat
- ✅ Spawning, wander/chase AI
- ✅ Health, DamageEvent, melee, contact damage
- ✅ HUD, enemy bars, damage splats
- ✅ Death / game-over / gravestone
- ✅ Player class selection
- ⬜ Ranged / magic attacks
### 🎮 Phase 5 — Gameplay Mechanics
- Inventory and item pickup
- Dungeon depth / difficulty scaling
- Keys, locks, themed rooms
### ✨ Phase 6 — Polish
- Full HUD (kill counter, gold, messages)
- Sound, save/load, animations, permadeath