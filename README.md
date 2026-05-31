# RustLike

A top-down roguelike prototype in Rust using the Bevy engine. Features procedural seeded dungeons, four playable classes, real-time enemy AI, melee combat, and a gradient HUD.

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
| F | Melee attack (hits tile in movement direction) |
| ← → on class screen | Browse classes |
| ENTER / SPACE | Confirm class selection / Respawn |
| Walk onto stairs | Enter / exit dungeon |

## Classes

| Class | HP | ATK | Speed |
|---|---|---|---|
| Warrior | 100 | 25 | 150 |
| Mage | 70 | 15 | 130 |
| Paladin | 140 | 20 | 120 |
| Rogue | 80 | 20 | 190 |

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
├── hud.rs              # health bars, damage splats
└── game_over.rs        # death screen, respawn, gravestone
```

## Features

- Procedural seeded dungeons — reproducible with `FIXED_SEED`
- 4 playable classes with unique stats
- Real-time wander/chase enemy AI with wall collision
- Melee combat (class-based damage) + enemy contact damage
- Gradient health bar (green → yellow → red), enemy HP bars, damage splats
- Death screen with gold-loss display; gravestone appears in the next dungeon
- 14 automated tests for map generation and coordinate math

## Roadmap

### ✅ Phase 1–3 — Foundation
Modules, Bevy States, seeded generation, automated tests

### ⚔️ Phase 4 — Enemies & Combat (in progress)
- ✅ Enemy spawning + wander/chase AI
- ✅ Health system, melee, contact damage
- ✅ HUD: gradient bar, enemy bars, damage splats
- ✅ Death screen, respawn, gravestone stub
- ✅ Player class selection
- ⬜ Ranged / magic attacks

### 🎮 Phase 5 — Gameplay Mechanics
Inventory, item drops, dungeon depth, keys/locks

### ✨ Phase 6 — Polish
Sound, save/load, animations, permadeath, camera scrolling

## Project Status

See [`PROJECT_STATUS.md`](PROJECT_STATUS.md) for detailed current state and known issues.

## License

MIT — see [`LICENSE`](LICENSE).