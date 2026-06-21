use bevy::prelude::*;
use crate::state::AppState;

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

const CONTROLS: [(&str, &str); 6] = [
    ("WASD / Arrows", "Move"),
    ("F",             "Melee attack"),
    ("SPACE",         "Ranged / magic attack"),
    ("ESC",           "Pause / resume"),
    ("Walk onto stairs", "Enter / exit dungeon"),
    ("SPACE (game over)", "Respawn"),
];

pub fn setup_pause_menu(mut commands: Commands) {
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

            // ── Options stub ────────────────────────────────────────────────
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
                    "Difficulty     — coming soon (permadeath vs. respawn)",
                    TextStyle { font_size: 15.0, color: Color::rgb(0.50, 0.50, 0.50), ..Default::default() },
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