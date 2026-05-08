use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use crate::player::Player;
use crate::combat::{Health, SplatEvent};
use crate::enemies::EnemyMarker;

// ─── constants ───────────────────────────────────────────────────────────────

const BAR_WIDTH:  f32 = 220.0;
const BAR_HEIGHT: f32 = 22.0;
const BAR_BORDER: f32 = 2.0;
const BAR_MARGIN: f32 = 14.0;

const ENEMY_BAR_W: f32 = 32.0;
const ENEMY_BAR_H: f32 = 4.0;
const ENEMY_BAR_Y: f32 = 22.0;
const Z_ENEMY_BAR: f32 = 2.0;

const SPLAT_LIFETIME:   f32 = 0.9;
const SPLAT_RISE_SPEED: f32 = 28.0;
const Z_SPLAT:          f32 = 11.0;

// ─── gradient image builder ───────────────────────────────────────────────────

/// Builds a horizontal gradient Image in memory — no async asset loading.
/// `left` and `right` are [R, G, B] values in 0–255 range.
fn make_gradient(left: [u8; 3], right: [u8; 3], w: u32, h: u32) -> Image {
    let lerp = |a: u8, b: u8, t: f32| -> u8 { (a as f32 + t * (b as f32 - a as f32)) as u8 };
    let mut data = Vec::with_capacity((w * h * 4) as usize);
    for _ in 0..h {
        for x in 0..w {
            let t = if w > 1 { x as f32 / (w - 1) as f32 } else { 0.0 };
            data.push(lerp(left[0], right[0], t));
            data.push(lerp(left[1], right[1], t));
            data.push(lerp(left[2], right[2], t));
            data.push(255u8);
        }
    }
    Image::new(
        Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

// ─── resources / markers ─────────────────────────────────────────────────────

/// Pre-built gradient handles inserted at Startup — always immediately available.
#[derive(Resource)]
pub struct BarAssets {
    pub green:  Handle<Image>,
    pub yellow: Handle<Image>,
    pub red:    Handle<Image>,
}

#[derive(Component)] pub struct HealthBarFill;
#[derive(Component)] pub struct HealthText;
#[derive(Component)] pub struct EnemyBarFor(pub Entity);
#[derive(Component)] pub struct EnemyBarFill;
#[derive(Component)] pub struct DamageSplat { remaining: f32, total: f32 }

// ─── setup ───────────────────────────────────────────────────────────────────

pub fn setup_hud(
    mut commands:  Commands,
    asset_server:  Res<AssetServer>,
    mut images:    ResMut<Assets<Image>>,
) {
    // Build gradients directly in memory — immediately available, no async load.
    let bar_assets = BarAssets {
        green:  images.add(make_gradient([100, 210,  80], [ 30, 120,  30], BAR_WIDTH as u32, BAR_HEIGHT as u32)),
        yellow: images.add(make_gradient([230, 200,  40], [160, 120,  15], BAR_WIDTH as u32, BAR_HEIGHT as u32)),
        red:    images.add(make_gradient([225,  55,  55], [140,  15,  15], BAR_WIDTH as u32, BAR_HEIGHT as u32)),
    };

    // Outer rounded panel
    commands
        .spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top:  Val::Px(BAR_MARGIN),
                left: Val::Px(BAR_MARGIN),
                flex_direction: FlexDirection::Row,
                align_items:    AlignItems::Center,
                column_gap:     Val::Px(8.0),
                padding:        UiRect::all(Val::Px(8.0)),
                ..Default::default()
            },
            image: UiImage {
                texture: asset_server.load("ui_panel.png"),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "HP",
                TextStyle { font_size: 16.0, color: Color::rgb(0.88, 0.88, 0.88), ..Default::default() },
            ));

            // White border wrapper
            root.spawn(NodeBundle {
                style: Style {
                    width:   Val::Px(BAR_WIDTH  + BAR_BORDER * 2.0),
                    height:  Val::Px(BAR_HEIGHT + BAR_BORDER * 2.0),
                    padding: UiRect::all(Val::Px(BAR_BORDER)),
                    ..Default::default()
                },
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.85).into(),
                ..Default::default()
            })
            .with_children(|border| {
                // Dark-red background
                border.spawn(NodeBundle {
                    style: Style {
                        width:  Val::Px(BAR_WIDTH),
                        height: Val::Px(BAR_HEIGHT),
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.20, 0.03, 0.03).into(),
                    ..Default::default()
                })
                .with_children(|bg| {
                    // Gradient fill — sized in Px to avoid CalculatedSize conflicts.
                    // Width is updated each frame by update_health_bar.
                    bg.spawn((
                        ImageBundle {
                            style: Style {
                                width:  Val::Px(BAR_WIDTH),
                                height: Val::Px(BAR_HEIGHT),
                                ..Default::default()
                            },
                            image: UiImage {
                                texture: bar_assets.green.clone(),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        HealthBarFill,
                    ));
                });
            });

            root.spawn((
                TextBundle::from_section(
                    "100 / 100",
                    TextStyle { font_size: 15.0, color: Color::rgb(0.92, 0.92, 0.92), ..Default::default() },
                ),
                HealthText,
            ));
        });

    commands.insert_resource(bar_assets);
}

// ─── update ──────────────────────────────────────────────────────────────────

pub fn update_health_bar(
    p_query:    Query<&Health, With<Player>>,
    bar_assets: Res<BarAssets>,
    mut fill_q: Query<(&mut Style, &mut UiImage), With<HealthBarFill>>,
    mut text_q: Query<&mut Text, With<HealthText>>,
) {
    let Ok(health) = p_query.get_single() else { return };
    let pct = (health.current as f32 / health.max as f32).clamp(0.0, 1.0);

    let texture = if pct > 0.6 {
        bar_assets.green.clone()
    } else if pct > 0.3 {
        bar_assets.yellow.clone()
    } else {
        bar_assets.red.clone()
    };

    for (mut style, mut image) in &mut fill_q {
        // Use Px so Bevy's CalculatedSize doesn't interfere with layout.
        style.width   = Val::Px(BAR_WIDTH * pct);
        image.texture = texture.clone();
    }
    for mut text in &mut text_q {
        text.sections[0].value = format!("{} / {}", health.current, health.max);
    }
}

// ─── enemy health bars ────────────────────────────────────────────────────────

pub fn spawn_enemy_health_bar(commands: &mut Commands, enemy_entity: Entity) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.40, 0.04, 0.04),
                custom_size: Some(Vec2::new(ENEMY_BAR_W, ENEMY_BAR_H)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Z_ENEMY_BAR),
            ..Default::default()
        },
        EnemyBarFor(enemy_entity),
        EnemyMarker,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.10, 0.52, 0.10),
                custom_size: Some(Vec2::new(ENEMY_BAR_W, ENEMY_BAR_H)),
                anchor: bevy::sprite::Anchor::CenterLeft,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Z_ENEMY_BAR + 0.1),
            ..Default::default()
        },
        EnemyBarFor(enemy_entity),
        EnemyBarFill,
        EnemyMarker,
    ));
}

pub fn sync_enemy_bars(
    enemy_q:  Query<(Entity, &Transform, &Health), With<EnemyMarker>>,
    mut bg_q: Query<(&EnemyBarFor, &mut Transform),
        (Without<EnemyBarFill>, With<EnemyBarFor>, Without<Health>)>,
    mut fill_q: Query<(&EnemyBarFor, &mut Transform, &mut Sprite),
        (With<EnemyBarFill>, Without<Health>)>,
) {
    for (bar_for, mut t, mut sprite) in &mut fill_q {
        if let Ok((_, et, health)) = enemy_q.get(bar_for.0) {
            let ep = et.translation;
            t.translation = Vec3::new(ep.x - ENEMY_BAR_W / 2.0, ep.y + ENEMY_BAR_Y, Z_ENEMY_BAR + 0.1);
            let pct = (health.current as f32 / health.max as f32).clamp(0.0, 1.0);
            sprite.color = Color::rgb(0.10, 0.52, 0.10); // always green — bar shrinks to show damage
            sprite.custom_size = Some(Vec2::new(ENEMY_BAR_W * pct, ENEMY_BAR_H));
        }
    }
    for (bar_for, mut t) in &mut bg_q {
        if let Ok((_, et, _)) = enemy_q.get(bar_for.0) {
            let ep = et.translation;
            t.translation = Vec3::new(ep.x, ep.y + ENEMY_BAR_Y, Z_ENEMY_BAR);
        }
    }
}

// ─── damage splats ────────────────────────────────────────────────────────────

pub fn handle_splat_events(
    mut events:   EventReader<SplatEvent>,
    mut commands: Commands,
) {
    for ev in events.iter() {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("-{}", ev.amount),
                    TextStyle { font_size: 18.0, color: Color::rgba(1.0, 0.9, 0.1, 1.0), ..Default::default() },
                ).with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(ev.position.x, ev.position.y + 16.0, Z_SPLAT),
                ..Default::default()
            },
            DamageSplat { remaining: SPLAT_LIFETIME, total: SPLAT_LIFETIME },
        ));
    }
}

pub fn update_damage_splats(
    time:         Res<Time>,
    mut commands: Commands,
    mut query:    Query<(Entity, &mut Transform, &mut Text, &mut DamageSplat)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut transform, mut text, mut splat) in &mut query {
        splat.remaining -= dt;
        if splat.remaining <= 0.0 { commands.entity(entity).despawn(); continue; }
        transform.translation.y += SPLAT_RISE_SPEED * dt;
        let alpha = (splat.remaining / splat.total).clamp(0.0, 1.0);
        if let Some(s) = text.sections.first_mut() { s.style.color.set_a(alpha); }
    }
}