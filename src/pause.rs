use bevy::prelude::*;
use crate::state::AppState;
use crate::difficulty::Difficulty;

// ─── marker ──────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct PauseUi;

// ─── toggle ──────────────────────────────────────────────────────────────────

/// Esc while playing — opens the pause menu.
/// Kept as a separate system (rather than one toggle) so each direction is
/// gated to fire only from its own state — avoids any chance of double-firing
/// in the same frame.
pub fn toggle_pause_on_escape(
    keyboard: Res<Input<KeyCode>>,
    mut next: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next.set(AppState::Paused);
    }
}

/// Esc while paused — closes the pause menu and resumes play.
pub fn handle_pause_input(
    keyboard: Res<Input<KeyCode>>,
    mut next: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next.set(AppState::Playing);
    }
}

// ─── UI ──────────────────────────────────────────────────────────────────────

const CONTROLS: [(&str, &str); 8] = [
    ("WASD / Arrows", "Move"),
    ("F",             "Melee attack"),
    ("SPACE",         "Ranged / magic attack"),
    ("I",             "Open / close inventory"),
    ("ESC",           "Pause / resume"),
    ("Walk onto stairs", "Enter / exit dungeon"),
    ("SPACE (game over)", "Respawn"),
    ("TAB (paused)",  "Toggle difficulty"),
];

pub fn setup_pause_menu(mut commands: Commands, difficulty: Res<Difficulty>) {
    build_pause_menu(&mut commands, *difficulty);
}

/// Builds the full pause-menu UI tree. Shared by `setup_pause_menu` and the
/// options toggle, which despawns and rebuilds the whole tree on TAB — same
/// rebuild-on-change pattern `character_select::handle_class_input` uses,
/// since Bevy 0.11 doesn't clean up a parent's `Children` list when a single
/// child is despawned with plain `despawn()` (only `despawn_recursive`), so
/// swapping just the difficulty line in place would leave a dangling
/// reference and panic the UI clipping system.
fn build_pause_menu(commands: &mut Commands, difficulty: Difficulty) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type:   PositionType::Absolute,
                    width:           Val::Percent(100.0),
                    height:          Val::Percent(100.0),
                    flex_direction:  FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items:     AlignItems::Center,
                    row_gap:         Val::Px(20.0),
                    ..Default::default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.82).into(),
                z_index: ZIndex::Global(200),
                ..Default::default()
            },
            PauseUi,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "PAUSED",
                TextStyle { font_size: 56.0, color: Color::rgb(0.92, 0.92, 0.92), ..Default::default() },
            ));

            // ── Controls panel ──────────────────────────────────────────────
            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items:    AlignItems::Center,
                    padding:        UiRect::all(Val::Px(20.0)),
                    row_gap:        Val::Px(10.0),
                    ..Default::default()
                },
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.06).into(),
                ..Default::default()
            })
            .with_children(|panel| {
                panel.spawn(TextBundle::from_section(
                    "CONTROLS",
                    TextStyle { font_size: 20.0, color: Color::rgb(0.88, 0.76, 0.25), ..Default::default() },
                ));

                // Two-column layout (keys | actions) — flexbox columns rather
                // than monospace padding, since the default font isn't fixed-width.
                panel.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap:     Val::Px(28.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|row| {
                    row.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(6.0),
                            align_items: AlignItems::FlexEnd,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|col| {
                        for (key, _) in CONTROLS {
                            col.spawn(TextBundle::from_section(
                                key,
                                TextStyle { font_size: 16.0, color: Color::rgb(0.95, 0.85, 0.40), ..Default::default() },
                            ));
                        }
                    });

                    row.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(6.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|col| {
                        for (_, action) in CONTROLS {
                            col.spawn(TextBundle::from_section(
                                action,
                                TextStyle { font_size: 16.0, color: Color::rgb(0.82, 0.82, 0.82), ..Default::default() },
                            ));
                        }
                    });
                });
            });

            // ── Options ─────────────────────────────────────────────────────
            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items:    AlignItems::Center,
                    padding:        UiRect::all(Val::Px(20.0)),
                    row_gap:        Val::Px(6.0),
                    ..Default::default()
                },
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.06).into(),
                ..Default::default()
            })
            .with_children(|panel| {
                panel.spawn(TextBundle::from_section(
                    "OPTIONS",
                    TextStyle { font_size: 20.0, color: Color::rgb(0.88, 0.76, 0.25), ..Default::default() },
                ));
                panel.spawn(TextBundle::from_section(
                    "Volume         — coming soon",
                    TextStyle { font_size: 15.0, color: Color::rgb(0.50, 0.50, 0.50), ..Default::default() },
                ));
                panel.spawn(TextBundle::from_section(
                    difficulty_line_text(difficulty),
                    TextStyle { font_size: 15.0, color: Color::rgb(0.75, 0.75, 0.75), ..Default::default() },
                ));
            });

            root.spawn(TextBundle::from_section(
                "Press ESC to resume",
                TextStyle { font_size: 18.0, color: Color::rgb(0.65, 0.65, 0.65), ..Default::default() },
            ));
        });
}

pub fn cleanup_pause_menu(
    query:        Query<Entity, With<PauseUi>>,
    mut commands: Commands,
) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

fn difficulty_line_text(difficulty: Difficulty) -> String {
    format!("Difficulty: {}   (TAB to change)", difficulty.display_name())
}

/// TAB while paused — flips the `Difficulty` resource and rebuilds the pause
/// menu so the Options panel reflects the new value.
pub fn handle_pause_options_input(
    keyboard:       Res<Input<KeyCode>>,
    mut difficulty: ResMut<Difficulty>,
    ui_query:       Query<Entity, With<PauseUi>>,
    mut commands:   Commands,
) {
    if !keyboard.just_pressed(KeyCode::Tab) { return; }

    *difficulty = match *difficulty {
        Difficulty::Standard   => Difficulty::Permadeath,
        Difficulty::Permadeath => Difficulty::Standard,
    };

    for e in &ui_query { commands.entity(e).despawn_recursive(); }
    build_pause_menu(&mut commands, *difficulty);
}