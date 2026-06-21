use bevy::prelude::*;
use crate::state::AppState;

// ─── player class ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Resource, Component)]
pub enum PlayerClass {
    #[default]
    Warrior,
    Mage,
    Paladin,
    Rogue,
}

// ─── attack kind ──────────────────────────────────────────────────────────────

/// Whether a class's ranged attack (Space key) is physical or arcane.
/// Determines projectile sprite and `DamageSource`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackKind {
    Ranged,
    Magic,
}

impl PlayerClass {
    pub const ALL: [PlayerClass; 4] = [
        PlayerClass::Warrior,
        PlayerClass::Mage,
        PlayerClass::Paladin,
        PlayerClass::Rogue,
    ];

    pub fn sprite(self) -> &'static str {
        match self {
            PlayerClass::Warrior => "warrior.png",
            PlayerClass::Mage    => "mage.png",
            PlayerClass::Paladin => "paladin.png",
            PlayerClass::Rogue   => "rogue.png",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            PlayerClass::Warrior => "WARRIOR",
            PlayerClass::Mage    => "MAGE",
            PlayerClass::Paladin => "PALADIN",
            PlayerClass::Rogue   => "ROGUE",
        }
    }

    pub fn max_hp(self) -> i32 {
        match self {
            PlayerClass::Warrior => 100,
            PlayerClass::Mage    =>  70,
            PlayerClass::Paladin => 140,
            PlayerClass::Rogue   =>  80,
        }
    }

    pub fn melee_damage(self) -> i32 {
        match self {
            PlayerClass::Warrior => 25,
            PlayerClass::Mage    => 15,
            PlayerClass::Paladin => 20,
            PlayerClass::Rogue   => 20,
        }
    }

    /// Damage dealt by the Space-key ranged/magic attack.
    pub fn ranged_damage(self) -> i32 {
        match self {
            PlayerClass::Warrior => 12,
            PlayerClass::Mage    => 30,
            PlayerClass::Paladin => 10,
            PlayerClass::Rogue   => 18,
        }
    }

    /// Mage casts magic; everyone else fires a physical projectile.
    pub fn attack_kind(self) -> AttackKind {
        match self {
            PlayerClass::Mage => AttackKind::Magic,
            _                 => AttackKind::Ranged,
        }
    }

    pub fn speed(self) -> f32 {
        match self {
            PlayerClass::Warrior => 150.0,
            PlayerClass::Mage    => 130.0,
            PlayerClass::Paladin => 120.0,
            PlayerClass::Rogue   => 190.0,
        }
    }

    pub fn flavour(self) -> &'static str {
        match self {
            PlayerClass::Warrior => "Tough fighter.\nHigh HP, strong melee.",
            PlayerClass::Mage    => "Arcane scholar.\nLow HP, ranged attacks.",
            PlayerClass::Paladin => "Holy guardian.\nHeaviest armour, slowest speed.",
            PlayerClass::Rogue   => "Silent predator.\nFast movement, quick strikes.",
        }
    }
}

// ─── resources / markers ──────────────────────────────────────────────────────

/// Index into `PlayerClass::ALL` for the currently highlighted card.
#[derive(Resource, Default)]
pub struct SelectedClass(pub usize);

/// Tags all character-select UI entities for cleanup.
#[derive(Component)]
pub struct ClassSelectUi;

// ─── setup ────────────────────────────────────────────────────────────────────

pub fn setup_character_select(
    mut commands:   Commands,
    asset_server:   Res<AssetServer>,
    selected:       Res<SelectedClass>,
) {
    // Full-screen dark background
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
        ClassSelectUi,
    ))
    .with_children(|root| {
        // Title
        root.spawn(TextBundle::from_section(
            "SELECT YOUR CLASS",
            TextStyle {
                font_size: 52.0,
                color:     Color::rgb(0.90, 0.80, 0.30),
                ..Default::default()
            },
        ));

        // Cards row
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
            for (i, &class) in PlayerClass::ALL.iter().enumerate() {
                let highlighted = i == selected.0;
                spawn_class_card(row, &asset_server, class, highlighted);
            }
        });

        // Hint
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

fn spawn_class_card(
    parent:       &mut ChildBuilder,
    asset_server: &AssetServer,
    class:        PlayerClass,
    highlighted:  bool,
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
            width:           Val::Px(200.0),
            height:          Val::Px(300.0),
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
            // Class sprite
            card.spawn(ImageBundle {
                style: Style {
                    width:  Val::Px(80.0),
                    height: Val::Px(80.0),
                    ..Default::default()
                },
                image: UiImage {
                    texture: asset_server.load(class.sprite()),
                    ..Default::default()
                },
                ..Default::default()
            });

            // Class name
            card.spawn(TextBundle::from_section(
                class.display_name(),
                TextStyle {
                    font_size: 22.0,
                    color:     if highlighted { Color::rgb(1.0, 0.90, 0.40) } else { Color::WHITE },
                    ..Default::default()
                },
            ));

            // Stats
            card.spawn(TextBundle::from_section(
                format!("HP  {}\nMEL {}\nRNG {}\nSPD {:.0}", class.max_hp(), class.melee_damage(), class.ranged_damage(), class.speed()),
                TextStyle {
                    font_size: 15.0,
                    color:     Color::rgb(0.70, 0.70, 0.70),
                    ..Default::default()
                },
            ));

            // Flavour
            card.spawn(TextBundle::from_section(
                class.flavour(),
                TextStyle {
                    font_size: 13.0,
                    color:     Color::rgb(0.50, 0.55, 0.60),
                    ..Default::default()
                },
            ));
        });
    });
}

// ─── cleanup ─────────────────────────────────────────────────────────────────

pub fn cleanup_character_select(
    query:        Query<Entity, With<ClassSelectUi>>,
    mut commands: Commands,
) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

// ─── input handling ──────────────────────────────────────────────────────────

/// Left/Right to move cursor, Enter/Space to confirm. Rebuilds the UI on
/// cursor change so the highlighted card updates immediately.
pub fn handle_class_input(
    keyboard:        Res<Input<KeyCode>>,
    mut selected:    ResMut<SelectedClass>,
    mut next:        ResMut<NextState<AppState>>,
    mut class_res:   ResMut<PlayerClass>,
    // Rebuild UI on navigation
    ui_query:        Query<Entity, With<ClassSelectUi>>,
    asset_server:    Res<AssetServer>,
    mut commands:    Commands,
) {
    let len = PlayerClass::ALL.len();
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
            ClassSelectUi,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "SELECT YOUR CLASS",
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
                for (i, &class) in PlayerClass::ALL.iter().enumerate() {
                    spawn_class_card(row, &asset_server, class, i == selected.0);
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
        *class_res = PlayerClass::ALL[selected.0];
        next.set(AppState::Playing);
    }
}