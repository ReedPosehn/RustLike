use crate::combat::Health;
use crate::player::Player;
use crate::state::AppState;
use crate::{SCALE, TILE};
use bevy::prelude::*;

// ─── gold ─────────────────────────────────────────────────────────────────────

/// Currency. Not an item — see `Inventory` below for usable/equipable items.
/// Dropped by enemies on death (`combat::despawn_dead_enemies`), collected by
/// walking over a `GoldPickup`, reset to 0 on Standard respawn.
#[derive(Component, Default, Debug)]
pub struct Gold(pub u32);

// ─── items ────────────────────────────────────────────────────────────────────

/// A usable inventory item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    HealthPotion,
}

impl Item {
    pub fn display_name(self) -> &'static str {
        match self {
            Item::HealthPotion => "Health Potion",
        }
    }
}

/// Items the player is carrying. Gold remains separate because it is currency,
/// not a usable/equipable item.
#[derive(Component, Default, Debug)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn count(&self, item: Item) -> usize {
        self.items.iter().filter(|held| **held == item).count()
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn remove_one(&mut self, item: Item) -> bool {
        if let Some(index) = self.items.iter().position(|held| *held == item) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }
}

// ─── gold pickups ─────────────────────────────────────────────────────────────

/// Distance within which the player auto-collects a pickup.
const PICKUP_RADIUS: f32 = TILE * 0.8;

/// A pile of gold sitting in the world, dropped by a dead enemy.
#[derive(Component)]
pub struct GoldPickup(pub u32);

/// Spawns a gold pickup at `pos` carrying `amount` gold.
pub fn spawn_gold_pickup(
    commands: &mut Commands,
    asset_server: &AssetServer,
    pos: Vec2,
    amount: u32,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("gold.png"),
            transform: Transform::from_xyz(pos.x, pos.y, 0.5).with_scale(Vec3::splat(SCALE * 0.7)),
            ..Default::default()
        },
        GoldPickup(amount),
    ));
}

/// A health potion sitting in the world.
#[derive(Component)]
pub struct PotionPickup;

/// Spawns a health potion pickup at `pos`.
pub fn spawn_potion_pickup(commands: &mut Commands, asset_server: &AssetServer, pos: Vec2) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("potion.png"),
            transform: Transform::from_xyz(pos.x, pos.y, 0.5).with_scale(Vec3::splat(SCALE * 0.7)),
            ..Default::default()
        },
        PotionPickup,
    ));
}

/// Player walks over a `GoldPickup` — adds its amount to `Gold` and despawns it.
pub fn collect_gold(
    mut p_query: Query<(&Transform, &mut Gold), With<Player>>,
    pickup_query: Query<(Entity, &Transform, &GoldPickup)>,
    mut commands: Commands,
) {
    let Ok((p_transform, mut gold)) = p_query.get_single_mut() else {
        return;
    };
    let player_pos = p_transform.translation.truncate();

    for (entity, transform, pickup) in &pickup_query {
        if player_pos.distance(transform.translation.truncate()) < PICKUP_RADIUS {
            gold.0 += pickup.0;
            commands.entity(entity).despawn();
        }
    }
}

/// Player walks over a potion — adds it to the inventory and despawns the pickup.
pub fn collect_potions(
    mut p_query: Query<(&Transform, &mut Inventory), With<Player>>,
    pickup_query: Query<(Entity, &Transform), With<PotionPickup>>,
    mut commands: Commands,
) {
    let Ok((p_transform, mut inventory)) = p_query.get_single_mut() else {
        return;
    };
    let player_pos = p_transform.translation.truncate();

    for (entity, transform) in &pickup_query {
        if player_pos.distance(transform.translation.truncate()) < PICKUP_RADIUS {
            inventory.add(Item::HealthPotion);
            commands.entity(entity).despawn();
        }
    }
}

/// Despawn any uncollected pickups when leaving the dungeon.
pub fn despawn_pickups(
    gold_query: Query<Entity, With<GoldPickup>>,
    potion_query: Query<Entity, With<PotionPickup>>,
    mut commands: Commands,
) {
    for e in &gold_query {
        commands.entity(e).despawn();
    }
    for e in &potion_query {
        commands.entity(e).despawn();
    }
}

/// Press H while playing to heal with a potion. Consumes one potion and restores
/// at most 35 HP. A potion is not consumed if the player is already at full HP.
pub fn use_health_potion(
    keyboard: Res<Input<KeyCode>>,
    mut p_query: Query<(&mut Health, &mut Inventory), With<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::H) {
        return;
    }
    let Ok((mut health, mut inventory)) = p_query.get_single_mut() else {
        return;
    };
    if health.current >= health.max {
        return;
    }

    if inventory.remove_one(Item::HealthPotion) {
        health.current = (health.current + 35).min(health.max);
    }
}

// ─── inventory view screen ─────────────────────────────────────────────────────

/// Tags the inventory-view UI for cleanup.
#[derive(Component)]
pub struct InventoryUi;

/// I while playing — opens the inventory screen. Freezes gameplay the same
/// way `Paused` does, since every gameplay system is gated on
/// `AppState::Playing` — no extra gating needed elsewhere.
pub fn toggle_inventory_on_key(
    keyboard: Res<Input<KeyCode>>,
    mut next: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::I) {
        next.set(AppState::InventoryView);
    }
}

/// I or Esc while viewing the inventory — closes it and resumes play.
pub fn handle_inventory_input(
    keyboard: Res<Input<KeyCode>>,
    mut next: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::I) || keyboard.just_pressed(KeyCode::Escape) {
        next.set(AppState::Playing);
    }
}

pub fn setup_inventory_view(
    mut commands: Commands,
    p_query: Query<&Inventory, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let potion_count = p_query
        .get_single()
        .map_or(0, |inv| inv.count(Item::HealthPotion));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..Default::default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.82).into(),
                z_index: ZIndex::Global(200),
                ..Default::default()
            },
            InventoryUi,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "INVENTORY",
                TextStyle {
                    font_size: 48.0,
                    color: Color::rgb(0.92, 0.92, 0.92),
                    ..Default::default()
                },
            ));

            if potion_count == 0 {
                root.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|row| {
                    row.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(32.0),
                            height: Val::Px(32.0),
                            ..Default::default()
                        },
                        image: UiImage {
                            texture: asset_server.load("potion.png"),
                            flip_x: false,
                            flip_y: false,
                        },
                        ..Default::default()
                    });
                    row.spawn(TextBundle::from_section(
                        format!("{}: 0", Item::HealthPotion.display_name()),
                        TextStyle {
                            font_size: 18.0,
                            color: Color::rgb(0.60, 0.60, 0.60),
                            ..Default::default()
                        },
                    ));
                });
            } else {
                root.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|row| {
                    row.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(32.0),
                            height: Val::Px(32.0),
                            ..Default::default()
                        },
                        image: UiImage {
                            texture: asset_server.load("potion.png"),
                            flip_x: false,
                            flip_y: false,
                        },
                        ..Default::default()
                    });
                    row.spawn(TextBundle::from_section(
                        format!("{} x{}", Item::HealthPotion.display_name(), potion_count),
                        TextStyle {
                            font_size: 18.0,
                            color: Color::rgb(0.80, 0.90, 0.80),
                            ..Default::default()
                        },
                    ));
                });
            }

            root.spawn(TextBundle::from_section(
                "Press H to use potion   |   Press I or Esc to close",
                TextStyle {
                    font_size: 18.0,
                    color: Color::rgb(0.65, 0.65, 0.65),
                    ..Default::default()
                },
            ));
        });
}

pub fn cleanup_inventory_view(query: Query<Entity, With<InventoryUi>>, mut commands: Commands) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}
