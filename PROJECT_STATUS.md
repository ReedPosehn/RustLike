# RustLike Project Status

## 🚧 Current Context

- **Language & Framework:** Rust + Bevy 0.11
- **Modules:** `main`, `map`, `player`, `state`, `character_select`, `difficulty`, `enemies`, `combat`, `projectile`, `hud`, `game_over`, `inventory`, `pause` (14 source files)
- **Assets:** 32×32 PNGs from `generate_assets.py` (Pillow)

## ✅ Features Implemented

1. **Tilemap & Dungeon** — Seed-based rooms, L-corridors, stairs. Fits 1280×720.
2. **AABB Movement** — Wall sliding, independent X/Y axes, per-class speed.
3. **Bevy States** — `AppState` (CharacterSelect → DifficultySelect → Playing ⇄ Paused/InventoryView → GameOver) + `MapState` (Hub/Dungeon). `OnEnter`/`OnExit` handle all lifecycle.
4. **Character Selection** — 4 classes (Warrior, Mage, Paladin, Rogue) with distinct HP/melee/ranged/speed stats.
5. **Enemy Spawning & AI** — Goblins, orcs, skeletons. Wander/chase state machine with hysteresis.
6. **Combat Foundation** — `DamageEvent` system. All damage flows through one event.
7. **Player Melee** — F key, facing direction, class-based damage.
8. **Ranged/Magic Attacks** — SPACE key fires an arrow (Warrior/Paladin/Rogue) or magic bolt (Mage), class-based damage and `DamageSource`.
9. **Enemy Contact Damage** — 0.8s cooldown timer.
10. **HUD** — Gradient health bar, enemy bars, damage splats.
11. **Player Death & Respawn** — `GameOver` state, gravestone spawns in next dungeon, new seed each death.
12. **Pause Menu** — ESC toggles; controls reference includes inventory/potion controls, and the difficulty option is live; volume remains a stub.
13. **Difficulty** — Standard/permadeath selection before playing and live pause-menu toggle.
14. **Gold and Potions** — Gold drops and pickups, health potion drops, inventory collection, potion icon/count UI, and `H` healing.
15. **Automated Tests** — 14 tests in `map.rs`.

## 📂 Source Structure

```
src/
├── main.rs              — constants, resources, app entry, startup systems
├── map.rs               — TileKind, Tilemap, builders, spawn_map, tests
├── player.rs            — Player, spawn_player (class-aware), player_movement
├── state.rs              — AppState, MapState, GameWorld, LastDeathInfo, transitions
├── character_select.rs  — PlayerClass, AttackKind, class select screen
├── difficulty.rs        — Standard/permadeath difficulty selection and resource
├── inventory.rs         — Gold, Item, Inventory, pickups and inventory UI
├── enemies.rs            — EnemyKind, Enemy, EnemyAi, spawn/despawn/AI
├── combat.rs             — Health, DamageEvent, Facing, combat systems
├── projectile.rs         — Projectile, fire/move/cleanup systems
├── hud.rs                — BarAssets, health bars, damage splats
├── game_over.rs          — Gold, game-over screen, respawn logic
└── pause.rs              — Pause menu, controls reference, options stub
```

## 🛠 Recent Work

### Session — Ranged attacks, pause menu, bug fixes (current)
- **Ranged/magic attacks** — `projectile.rs` added. SPACE fires class-appropriate projectile (arrow or magic bolt). Single combined update system (movement + wall collision + enemy collision + lifetime) avoids double-despawn races.
- **Pause menu** — `AppState::Paused`. ESC toggles from `Playing`. Controls reference includes inventory/potion controls, and the difficulty option is live; volume remains a stub.
- **Bug fix: targeting** — combat queries filtered by `EnemyMarker` (also present on health bar sprites) now filter by `Enemy` instead, across melee, contact damage, and projectile hit detection.
- **Bug fix: frozen projectiles** — `despawn_projectiles` added on `OnExit(MapState::Dungeon)`. Previously, dying or returning to the hub mid-flight left the projectile frozen forever (its own update system requires both `Dungeon` and `Playing`, which death/exit breaks).
- **Bug fix: lingering health bars** — `despawn_dead_enemies` now also despawns the dead enemy's `EnemyBarFor`-tagged bar entities, which previously had no path to cleanup.
- **Gating fix** — dungeon-only combat systems now require `MapState::Dungeon AND AppState::Playing` (previously just `Dungeon`), so enemies/projectiles properly freeze during the game-over and pause screens instead of continuing to act in the background.

### Session — Character select + death polish
- Character select screen, `PlayerClass` resource/component, class-driven stats.
- `spawn_player` moved to `OnEnter(Playing)` with duplicate-spawn guard.
- Warning cleanup (`SPEED` → `ENEMY_SPEED`, removed dead code, `#[allow(dead_code)]` on stubs).

### Session — Death, respawn, gravestone
- `GameOver` state, dark overlay UI, recorded gold lost, and gravestone in next dungeon.

### Session — HUD & combat foundation
- Gradient health bar (in-memory), enemy bars, damage splats, `DamageEvent`/`SplatEvent` pipeline.

### Session — Health potion inventory
- Added `Item::HealthPotion`, inventory counts, potion pickups, and `H`-key healing for 35 HP.
- Defeated enemies have a 20% chance to drop a potion alongside normal gold.
- The inventory screen displays a 32×32 potion icon and current count.
- Added `potion.png` generation and preserved pickup cleanup on dungeon exit.

## 📌 Known Issues

- Health potions are the only item type so far; equipment and additional items remain planned.
- Enemies don't pathfind — can get stuck on corners.
- Enemies can overlap each other.
- Dungeon sizing is hardcoded to the 1280×720 window.
- The pause-menu volume option remains a visual stub; difficulty is live.

## 🔮 Roadmap

### ✅ Phase 1–3 — Foundation, collision, architecture, tests
### ⚔️ Phase 4 — Enemies & Combat (essentially complete)
- ✅ Spawning, wander/chase AI
- ✅ Health, DamageEvent, melee, contact damage
- ✅ HUD, enemy bars, damage splats
- ✅ Death / game-over / gravestone
- ✅ Player class selection
- ✅ Ranged / magic attacks
- ✅ Pause menu
### 🎮 Phase 5 — Gameplay Mechanics (next)
- ✅ Difficulty selector (permadeath vs. standard respawn)
- ✅ Gold and health potion inventory mechanics
- Dungeon depth / difficulty scaling
- Additional item types, equipment, and themed rooms
### ✨ Phase 6 — Polish
- Full HUD (kill counter, gold, messages)
- Sound, save/load, animations, permadeath
- Interactive pause options (volume, difficulty)