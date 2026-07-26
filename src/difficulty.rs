use bevy::prelude::*;
use crate::state::AppState;

// ─── difficulty ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Resource)]
pub enum Difficulty {
    #[default]
    Standard,
    Permadeath,
}

impl Difficulty {
    pub const ALL: [Difficulty; 2] = [Difficulty::Standard, Difficulty::Permadeath];

    pub fn display_name(self) -> &'static str {
        match self {
            Difficulty::Standard   => "STANDARD",
            Difficulty::Permadeath => "PERMADEATH",
        }
    }

    pub fn flavour(self) -> &'static str {
        match self {
            Difficulty::Standard   => "Die and respawn.\nNew dungeon, same character, full heal.",
            Difficulty::Permadeath => "Death is final.\nOne life — dying starts a brand new character.",
        }
    }
}

// ─── resources / markers ──────────────────────────────────────────────────────

/// Index into `Difficulty::ALL` for the currently highlighted card.
#[derive(Resource, Default)]
pub struct SelectedDifficultyIndex(pub usize);

/// Tags all difficulty-select UI entities for cleanup.
#[derive(Component)]
pub struct DifficultySelectUi;

// ─── setup ────────────────────────────────────────────────────────────────────

pub fn setup_difficulty_select(
    mut commands: Commands,
    selected:     Res<SelectedDifficultyIndex>,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width:           Val::Percent(100.0),
                height:          Val::Percent(100.0),
                flex_direction:  FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items:     AlignItems::Center,
                row_gap:         Val::Px(32.0),
                ..Default::default()
            },
            background_color: Color::rgb(0.06, 0.06, 0.10).into(),
            ..Default::default()
        },
        DifficultySelectUi,
    ))
    .with_children(|root| {
        root.spawn(TextBundle::from_section(
            "SELECT DIFFICULTY",
            TextStyle {
                font_size: 52.0,
                color:     Color::rgb(0.90, 0.80, 0.30),
                ..Default::default()
            },
        ));

        root.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap:     Val::Px(24.0),
                align_items:    AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|row| {
            for (i, &difficulty) in Difficulty::ALL.iter().enumerate() {
                let highlighted = i == selected.0;
                spawn_difficulty_card(row, difficulty, highlighted);
            }
        });

        root.spawn(TextBundle::from_section(
            "← → to browse     ENTER or SPACE to begin",
            TextStyle {
                font_size: 20.0,
                color:     Color::rgb(0.55, 0.55, 0.55),
                ..Default::default()
            },
        ));
    });
}

fn spawn_difficulty_card(
    parent:      &mut ChildBuilder,
    difficulty:  Difficulty,
    highlighted: bool,
) {
    let border_color = if highlighted {
        Color::rgb(0.90, 0.75, 0.20) // gold highlight
    } else {
        Color::rgb(0.25, 0.25, 0.30) // dark border
    };
    let bg_color = if highlighted {
        Color::rgb(0.18, 0.15, 0.05)
    } else {
        Color::rgb(0.10, 0.10, 0.14)
    };

    // Outer border node
    parent.spawn(NodeBundle {
        style: Style {
            width:           Val::Px(260.0),
            height:          Val::Px(180.0),
            padding:         UiRect::all(Val::Px(3.0)),
            flex_direction:  FlexDirection::Column,
            align_items:     AlignItems::Center,
            ..Default::default()
        },
        background_color: border_color.into(),
        ..Default::default()
    })
    .with_children(|border| {
        // Inner card
        border.spawn(NodeBundle {
            style: Style {
                width:           Val::Percent(100.0),
                height:          Val::Percent(100.0),
                flex_direction:  FlexDirection::Column,
                align_items:     AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                padding:         UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            background_color: bg_color.into(),
            ..Default::default()
        })
        .with_children(|card| {
            card.spawn(TextBundle::from_section(
                difficulty.display_name(),
                TextStyle {
                    font_size: 22.0,
                    color:     if highlighted { Color::rgb(1.0, 0.90, 0.40) } else { Color::WHITE },
                    ..Default::default()
                },
            ));

            card.spawn(TextBundle::from_section(
                difficulty.flavour(),
                TextStyle {
                    font_size: 15.0,
                    color:     Color::rgb(0.70, 0.70, 0.70),
                    ..Default::default()
                },
            ));
        });
    });
}

// ─── cleanup ─────────────────────────────────────────────────────────────────

pub fn cleanup_difficulty_select(
    query:        Query<Entity, With<DifficultySelectUi>>,
    mut commands: Commands,
) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

// ─── input handling ──────────────────────────────────────────────────────────

/// Left/Right to move cursor, Enter/Space to confirm. Rebuilds the UI on
/// cursor change so the highlighted card updates immediately.
pub fn handle_difficulty_input(
    keyboard:         Res<Input<KeyCode>>,
    mut selected:     ResMut<SelectedDifficultyIndex>,
    mut next:         ResMut<NextState<AppState>>,
    mut difficulty_res: ResMut<Difficulty>,
    ui_query:         Query<Entity, With<DifficultySelectUi>>,
    mut commands:     Commands,
) {
    let len = Difficulty::ALL.len();
    let mut changed = false;

    if keyboard.just_pressed(KeyCode::Left) || keyboard.just_pressed(KeyCode::A) {
        selected.0 = (selected.0 + len - 1) % len;
        changed = true;
    }
    if keyboard.just_pressed(KeyCode::Right) || keyboard.just_pressed(KeyCode::D) {
        selected.0 = (selected.0 + 1) % len;
        changed = true;
    }

    // Rebuild cards on cursor move
    if changed {
        for e in &ui_query { commands.entity(e).despawn_recursive(); }

        commands.spawn((
            NodeBundle {
                style: Style {
                    width:           Val::Percent(100.0),
                    height:          Val::Percent(100.0),
                    flex_direction:  FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items:     AlignItems::Center,
                    row_gap:         Val::Px(32.0),
                    ..Default::default()
                },
                background_color: Color::rgb(0.06, 0.06, 0.10).into(),
                ..Default::default()
            },
            DifficultySelectUi,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "SELECT DIFFICULTY",
                TextStyle { font_size: 52.0, color: Color::rgb(0.90, 0.80, 0.30), ..Default::default() },
            ));
            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap:     Val::Px(24.0),
                    align_items:    AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|row| {
                for (i, &difficulty) in Difficulty::ALL.iter().enumerate() {
                    spawn_difficulty_card(row, difficulty, i == selected.0);
                }
            });
            root.spawn(TextBundle::from_section(
                "← → to browse     ENTER or SPACE to begin",
                TextStyle { font_size: 20.0, color: Color::rgb(0.55, 0.55, 0.55), ..Default::default() },
            ));
        });
    }

    // Confirm
    if keyboard.just_pressed(KeyCode::Return) || keyboard.just_pressed(KeyCode::Space) {
        *difficulty_res = Difficulty::ALL[selected.0];
        next.set(AppState::Playing);
    }
}
