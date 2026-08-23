use bevy::prelude::*;
use crate::{SCALE, TILE};
use crate::player::Player;
use crate::state::AppState;

// ─── gold ─────────────────────────────────────────────────────────────────────

/// Currency. Not an item — see `Inventory` below for usable/equipable items.
/// Dropped by enemies on death (`combat::despawn_dead_enemies`), collected by
/// walking over a `GoldPickup`, reset to 0 on Standard respawn.
#[derive(Component, Default, Debug)]
pub struct Gold(pub u32);

// ─── items (stub) ────────────────────────────────────────────────────────────

/// Placeholder for future item types (potions, equipment, etc.) — usable or
/// equipable, unlike `Gold`. No variants yet; exists so `Inventory` has a
/// concrete type to hold once item types are added.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {}

/// Items the player is carrying. Kept separate from `Gold` — currency isn't
/// an item. Empty until item types exist.
#[derive(Component, Default, Debug)]
pub struct Inventory {
    #[allow(dead_code)] // read by future item systems
    pub items: Vec<Item>,
}

// ─── gold pickups ─────────────────────────────────────────────────────────────

/// Distance within which the player auto-collects a `GoldPickup`.
const PICKUP_RADIUS: f32 = TILE * 0.8;

/// A pile of gold sitting in the world, dropped by a dead enemy.
#[derive(Component)]
pub struct GoldPickup(pub u32);

/// Spawns a gold pickup at `pos` carrying `amount` gold.
pub fn spawn_gold_pickup(
    commands:     &mut Commands,
    asset_server: &AssetServer,
    pos:          Vec2,
    amount:       u32,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("gold.png"),
            transform: Transform::from_xyz(pos.x, pos.y, 0.5)
                .with_scale(Vec3::splat(SCALE * 0.7)),
            ..Default::default()
        },
        GoldPickup(amount),
    ));
}

/// Player walks over a `GoldPickup` — adds its amount to `Gold` and despawns it.
pub fn collect_gold(
    mut p_query:   Query<(&Transform, &mut Gold), With<Player>>,
    pickup_query:  Query<(Entity, &Transform, &GoldPickup)>,
    mut commands:  Commands,
) {
    let Ok((p_transform, mut gold)) = p_query.get_single_mut() else { return };
    let player_pos = p_transform.translation.truncate();

    for (entity, transform, pickup) in &pickup_query {
        if player_pos.distance(transform.translation.truncate()) < PICKUP_RADIUS {
            gold.0 += pickup.0;
            commands.entity(entity).despawn();
        }
    }
}

/// Despawn any uncollected gold piles when leaving the dungeon.
pub fn despawn_gold_pickups(
    query:        Query<Entity, With<GoldPickup>>,
    mut commands: Commands,
) {
    for e in &query { commands.entity(e).despawn(); }
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
    p_query:      Query<&Inventory, With<Player>>,
) {
    let item_count = p_query.get_single().map_or(0, |inv| inv.items.len());

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
            InventoryUi,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "INVENTORY",
                TextStyle { font_size: 48.0, color: Color::rgb(0.92, 0.92, 0.92), ..Default::default() },
            ));

            let body = if item_count == 0 {
                "No items yet.".to_string()
            } else {
                format!("{item_count} item(s)")
            };
            root.spawn(TextBundle::from_section(
                body,
                TextStyle { font_size: 18.0, color: Color::rgb(0.60, 0.60, 0.60), ..Default::default() },
            ));

            root.spawn(TextBundle::from_section(
                "Press I to close",
                TextStyle { font_size: 18.0, color: Color::rgb(0.65, 0.65, 0.65), ..Default::default() },
            ));
        });
}

pub fn cleanup_inventory_view(
    query:        Query<Entity, With<InventoryUi>>,
    mut commands: Commands,
) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}
