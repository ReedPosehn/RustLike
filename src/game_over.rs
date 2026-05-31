use bevy::prelude::*;
use rand::Rng;
use crate::map::build_dungeon;
use crate::player::Player;
use crate::combat::{Health, Dead};
use crate::state::{AppState, MapState, GameWorld, LastDeathInfo};
use crate::character_select::PlayerClass;
use crate::DungeonSeed;

// ─── gold ─────────────────────────────────────────────────────────────────────

/// Tracks how much gold the player is carrying.
/// Stubbed at 0 until the item/economy system is built — the death
/// screen already displays it and the respawn flow already zeroes it.
#[derive(Component, Default, Debug)]
pub struct Gold(pub u32);

// ─── UI markers ──────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct GameOverUi;

// ─── detect player death ──────────────────────────────────────────────────────

/// Runs every frame in `AppState::Playing`. When the player entity has the
/// `Dead` marker, records death info and transitions to `AppState::GameOver`.
/// The actual map/enemy cleanup happens when `MapState::Hub` is set on respawn.
pub fn check_player_death(
    mut next_app:   ResMut<NextState<AppState>>,
    mut death_info: ResMut<LastDeathInfo>,
    seed:           Res<DungeonSeed>,
    query:          Query<&Gold, (With<Player>, With<Dead>)>,
) {
    let Ok(gold) = query.get_single() else { return };

    death_info.gold_lost = gold.0;
    death_info.seed      = seed.0;

    next_app.set(AppState::GameOver);
}

// ─── game-over screen ─────────────────────────────────────────────────────────

/// Spawns a full-screen overlay with death stats and respawn prompt.
pub fn setup_game_over(
    mut commands: Commands,
    death_info:   Res<LastDeathInfo>,
) {
    // Dark overlay — covers the entire screen
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type:   PositionType::Absolute,
                width:           Val::Percent(100.0),
                height:          Val::Percent(100.0),
                flex_direction:  FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items:     AlignItems::Center,
                row_gap:         Val::Px(18.0),
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.80).into(),
            z_index: ZIndex::Global(100),
            ..Default::default()
        },
        GameOverUi,
    ))
    .with_children(|root| {
        // "YOU DIED"
        root.spawn(TextBundle::from_section(
            "YOU DIED",
            TextStyle {
                font_size: 72.0,
                color:     Color::rgb(0.85, 0.08, 0.08),
                ..Default::default()
            },
        ));

        // Gold lost
        root.spawn(TextBundle::from_section(
            format!("Gold lost:  {}", death_info.gold_lost),
            TextStyle {
                font_size: 28.0,
                color:     Color::rgb(0.85, 0.75, 0.20),
                ..Default::default()
            },
        ));

        // Spacer
        root.spawn(NodeBundle {
            style: Style { height: Val::Px(12.0), ..Default::default() },
            ..Default::default()
        });

        // Respawn prompt
        root.spawn(TextBundle::from_section(
            "Press SPACE to respawn",
            TextStyle {
                font_size: 24.0,
                color:     Color::rgb(0.75, 0.75, 0.75),
                ..Default::default()
            },
        ));
    });
}

/// Despawn the game-over overlay when leaving the GameOver state.
pub fn cleanup_game_over(
    query:        Query<Entity, With<GameOverUi>>,
    mut commands: Commands,
) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

// ─── respawn ──────────────────────────────────────────────────────────────────

/// Handles the SPACE key on the game-over screen.
/// Generates a new dungeon seed, rebuilds the dungeon, restores the player,
/// and transitions back to Playing → Hub (which triggers all cleanup).
pub fn handle_respawn_input(
    keyboard:       Res<Input<KeyCode>>,
    mut next_app:   ResMut<NextState<AppState>>,
    mut next_map:   ResMut<NextState<MapState>>,
    mut world:      ResMut<GameWorld>,
    mut seed_res:   ResMut<DungeonSeed>,
    mut p_query:    Query<(Entity, &mut Health), With<Player>>,
    class_res_r:    Res<PlayerClass>,
    mut commands:   Commands,
) {
    if !keyboard.just_pressed(KeyCode::Space) { return; }

    // Generate a new dungeon with a fresh seed.
    let new_seed = rand::thread_rng().gen::<u64>();
    println!("Respawn — new dungeon seed: {new_seed}");
    let (new_dungeon, new_stairs, new_rooms) = build_dungeon(new_seed);
    world.dungeon        = new_dungeon;
    world.dungeon_stairs = new_stairs;
    world.dungeon_rooms  = new_rooms;
    seed_res.0           = new_seed;

    // Restore the player — remove Dead and refill HP to class max.
    let class = *class_res_r;
    if let Ok((player_e, mut health)) = p_query.get_single_mut() {
        health.current = class.max_hp();
        health.max     = class.max_hp();
        commands.entity(player_e).remove::<Dead>();
    }

    // Transition to Hub (triggers OnExit(Dungeon) cleanup) then Playing.
    next_map.set(MapState::Hub);
    next_app.set(AppState::Playing);
}