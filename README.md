# RustLike

A top-down roguelike prototype in Rust using the Bevy engine. Features procedural seeded dungeons, four playable classes, real-time enemy AI, melee + ranged/magic combat, and a gradient HUD with a pause menu.

## Getting Started

```bash
git clone https://github.com/ReedPosehn/RustLike.git
cd RustLike

pip install pillow
python generate_assets.py   # generate sprites

cargo run                   # build and play
cargo test                  # run automated tests
```

To replay a dungeon: copy the seed printed at startup and set `FIXED_SEED = Some(n)` in `src/main.rs`.

## Controls

| Key | Action |
|---|---|
| WASD / Arrows | Move |
| F | Melee attack (facing direction) |
| SPACE | Ranged / magic attack (facing direction) |
| ESC | Pause / resume |
| ← → on class screen | Browse classes |
| ENTER / SPACE | Confirm class selection / Respawn |
| Walk onto stairs | Enter / exit dungeon |

## Classes

| Class | HP | Melee | Ranged | Speed | Attack Type |
|---|---|---|---|---|---|
| Warrior | 100 | 25 | 12 | 150 | Arrow |
| Mage | 70 | 15 | 30 | 130 | Magic bolt |
| Paladin | 140 | 20 | 10 | 120 | Arrow |
| Rogue | 80 | 20 | 18 | 190 | Arrow |

## Source Layout

```
src/
├── main.rs             # constants, app entry, startup
├── map.rs              # tilemap, dungeon generation, tests
├── player.rs           # movement, class-aware stats
├── state.rs            # AppState, MapState, world transitions
├── character_select.rs # class selection screen
├── enemies.rs          # enemy AI and spawning
├── combat.rs           # health, damage events, melee
├── projectile.rs       # ranged/magic attacks
├── hud.rs              # health bars, damage splats
├── game_over.rs        # death screen, respawn, gravestone
└── pause.rs            # pause menu, controls reference
```

## Features

- Procedural seeded dungeons — reproducible with `FIXED_SEED`
- 4 playable classes with unique stats and attack styles
- Real-time wander/chase enemy AI with wall collision
- Melee combat (F) and ranged/magic combat (SPACE), both class-based damage
- Gradient health bar, enemy HP bars, floating damage numbers
- Death screen with gold-loss display; gravestone appears in the next dungeon
- Pause menu with a controls reference and options stub
- 14 automated tests for map generation and coordinate math

## Roadmap

### ✅ Phase 1–3 — Foundation
Modules, Bevy States, seeded generation, automated tests

### ⚔️ Phase 4 — Enemies & Combat (complete)
Enemy AI, health system, melee + ranged/magic attacks, HUD, death/respawn, class selection, pause menu

### 🎮 Phase 5 — Gameplay Mechanics (next)
- Difficulty selector (permadeath vs. respawn)
- Inventory and item pickup
- Dungeon depth and difficulty scaling
- Keys, locks, themed rooms

### ✨ Phase 6 — Polish
Sound, save/load, animations, interactive pause options, camera scrolling

## Project Status

See [`PROJECT_STATUS.md`](PROJECT_STATUS.md) for detailed current state and known issues.

## License

MIT — see [`LICENSE`](LICENSE).