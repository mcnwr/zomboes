use bevy::prelude::*;
use crate::plugins::wave::WaveState;

#[derive(Component)]
struct WaveText;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct MoneyText;

#[derive(Component)]
struct AmmoText;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, (update_wave_ui, update_health_ui, update_money_ui, update_ammo_ui));
    }
}

fn update_money_ui(
    wallet: Res<crate::plugins::shop::Wallet>,
    mut query: Query<&mut Text, With<MoneyText>>,
) {
    for mut text in &mut query {
        text.sections[0].value = format!("Money: ${}", wallet.money);
    }
}

fn update_health_ui(
    player_query: Query<&crate::components::Health, With<crate::plugins::player::Player>>,
    mut text_query: Query<&mut Text, With<HealthText>>,
) {
    if let Ok(health) = player_query.get_single() {
        for mut text in &mut text_query {
            text.sections[0].value = format!("Health: {:.0}/{:.0}", health.current, health.max);
        }
    }
}

fn setup_ui(mut commands: Commands) {
    // HUD Node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Left side
            parent.spawn((
                TextBundle::from_section(
                    "Health: 100",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
                }),
                HealthText,
            ));

            // Ammo
            parent.spawn((
                TextBundle::from_section(
                    "Ammo: 30/30",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
                }),
                AmmoText,
            ));

            // Right side: Wave Info & Money
             parent.spawn((
                TextBundle::from_section(
                    "Money: $0",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(40.0), // Below Wave
                    right: Val::Px(10.0),
                    ..default()
                }),
                MoneyText,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Wave: 1",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..default()
                }),
                WaveText,
            ));
        });
}

fn update_wave_ui(
    wave_state: Res<WaveState>,
    mut query: Query<&mut Text, With<WaveText>>,
) {
    for mut text in &mut query {
        text.sections[0].value = format!("Wave: {}", wave_state.current_wave);
    }
}

fn update_ammo_ui(
    player_query: Query<&crate::plugins::player::WeaponStats, With<crate::plugins::player::Player>>,
    mut text_query: Query<&mut Text, With<AmmoText>>,
) {
    if let Ok(stats) = player_query.get_single() {
        for mut text in &mut text_query {
            text.sections[0].value = format!("Ammo: {}/{}", stats.current_ammo, stats.max_ammo);
        }
    }
}
