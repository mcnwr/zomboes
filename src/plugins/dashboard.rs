use crate::states::GameState;
use bevy::prelude::*;

pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Dashboard),
            (setup_dashboard, cleanup_level),
        )
        .add_systems(OnExit(GameState::Dashboard), cleanup_dashboard)
        .add_systems(
            Update,
            (
                dashboard_interactions,
                dashboard_visuals,
                update_dashboard_stats,
            )
                .run_if(in_state(GameState::Dashboard)),
        );
    }
}

fn update_dashboard_stats(
    mut query: Query<&mut Text, With<DashboardStatsText>>,
    stats: Res<crate::plugins::progression::GlobalPlayerStats>,
) {
    for mut text in &mut query {
        text.sections[0].value = format!("Level: {} | Money: ${}", stats.level, stats.total_money);
    }
}

fn cleanup_level(
    mut commands: Commands,
    players: Query<Entity, With<crate::plugins::player::Player>>,
    zombies: Query<Entity, With<crate::components::Zombie>>,
    projectiles: Query<Entity, With<crate::components::Projectile>>,
) {
    for entity in &players {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &zombies {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &projectiles {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct DashboardUI;

#[derive(Component)]
struct DashboardStatsText;

#[derive(Component)]
enum DashboardButton {
    Play,
    UpgradeWeapon,
    UpgradeAmmo,
    UnlockShotgun,
    UnlockRifle,
    DifficultyEasy,
    DifficultyMed,
    DifficultyHard,
    Quit,
}

fn setup_dashboard(
    mut commands: Commands,
    stats: Res<crate::plugins::progression::GlobalPlayerStats>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.1, 1.0).into(), // Dark Grey Background
                ..default()
            },
            DashboardUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "ZOMBIE SURVIVOR",
                TextStyle {
                    font_size: 60.0,
                    color: Color::srgb(0.0, 1.0, 0.0), // Green Title
                    ..default()
                },
            ));

            // ...

            // Global Stats Display
            parent.spawn((
                TextBundle::from_section(
                    format!("Level: {} | Money: ${}", stats.level, stats.total_money),
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                DashboardStatsText,
            ));

            // Difficulty Selector
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Difficulty: ",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));

                    // Easy
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                                ..default()
                            },
                            DashboardButton::DifficultyEasy,
                        ))
                        .with_children(|p| {
                            p.spawn(TextBundle::from_section(
                                "Easy",
                                TextStyle {
                                    font_size: 20.0,
                                    ..default()
                                },
                            ));
                        });

                    // Medium
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.6, 0.6, 0.2).into(),
                                ..default()
                            },
                            DashboardButton::DifficultyMed,
                        ))
                        .with_children(|p| {
                            p.spawn(TextBundle::from_section(
                                "Med",
                                TextStyle {
                                    font_size: 20.0,
                                    ..default()
                                },
                            ));
                        });

                    // Hard
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.6, 0.2, 0.2).into(),
                                ..default()
                            },
                            DashboardButton::DifficultyHard,
                        ))
                        .with_children(|p| {
                            p.spawn(TextBundle::from_section(
                                "Hard",
                                TextStyle {
                                    font_size: 20.0,
                                    ..default()
                                },
                            ));
                        });
                });

            // Play Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                        ..default()
                    },
                    DashboardButton::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "PLAY",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // Shop Section
            parent.spawn(TextBundle::from_section(
                "SHOP",
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(1.0, 0.8, 0.0),
                    ..default()
                },
            ));

            // Shop Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: Color::WHITE.into(),
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.3).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(10.0),
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Upgrade Rate
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(180.0),
                                            height: Val::Px(60.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.2, 0.2, 0.8).into(),
                                        ..default()
                                    },
                                    DashboardButton::UpgradeWeapon,
                                ))
                                .with_children(|p| {
                                    p.spawn(TextBundle::from_section(
                                        "Fire Rate\n$100",
                                        TextStyle {
                                            font_size: 20.0,
                                            ..default()
                                        },
                                    ));
                                });

                            // Upgrade Ammo
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(180.0),
                                            height: Val::Px(60.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.2, 0.2, 0.8).into(),
                                        ..default()
                                    },
                                    DashboardButton::UpgradeAmmo,
                                ))
                                .with_children(|p| {
                                    p.spawn(TextBundle::from_section(
                                        "Max Ammo\n$100",
                                        TextStyle {
                                            font_size: 20.0,
                                            ..default()
                                        },
                                    ));
                                });
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(10.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Unlock Shotgun
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(180.0),
                                            height: Val::Px(60.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.5, 0.0, 0.5).into(),
                                        ..default()
                                    },
                                    DashboardButton::UnlockShotgun,
                                ))
                                .with_children(|p| {
                                    p.spawn(TextBundle::from_section(
                                        "Shotgun\n$500",
                                        TextStyle {
                                            font_size: 20.0,
                                            ..default()
                                        },
                                    ));
                                });

                            // Unlock Rifle
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(180.0),
                                            height: Val::Px(60.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.5, 0.0, 0.5).into(),
                                        ..default()
                                    },
                                    DashboardButton::UnlockRifle,
                                ))
                                .with_children(|p| {
                                    p.spawn(TextBundle::from_section(
                                        "Rifle\n$1000",
                                        TextStyle {
                                            font_size: 20.0,
                                            ..default()
                                        },
                                    ));
                                });
                        });
                });

            // Quit Button (Placeholder logic for now, usually requires AppExit event)
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                        ..default()
                    },
                    DashboardButton::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "QUIT",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

fn cleanup_dashboard(mut commands: Commands, query: Query<Entity, With<DashboardUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn dashboard_interactions(
    mut interaction_query: Query<
        (&Interaction, &DashboardButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
    mut global_stats: ResMut<crate::plugins::progression::GlobalPlayerStats>,
    mut settings: ResMut<crate::plugins::settings::GameSettings>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                DashboardButton::Play => {
                    if settings.difficulty.is_some() {
                        next_state.set(GameState::Playing);
                    } else {
                        info!("Please select a difficulty first!");
                    }
                }

                // Difficulty
                DashboardButton::DifficultyEasy => {
                    settings.difficulty = Some(crate::plugins::settings::Difficulty::Easy);
                    info!("Difficulty: Easy");
                }
                DashboardButton::DifficultyMed => {
                    settings.difficulty = Some(crate::plugins::settings::Difficulty::Medium);
                    info!("Difficulty: Medium");
                }
                DashboardButton::DifficultyHard => {
                    settings.difficulty = Some(crate::plugins::settings::Difficulty::Hard);
                    info!("Difficulty: Hard");
                }

                // Shop
                DashboardButton::UpgradeWeapon => {
                    if global_stats.total_money >= 100 {
                        global_stats.total_money -= 100;
                        global_stats.weapon_upgrade_level += 1;
                        info!("Upgraded Rate: {}", global_stats.weapon_upgrade_level);
                    }
                }
                DashboardButton::UpgradeAmmo => {
                    if global_stats.total_money >= 100 {
                        global_stats.total_money -= 100;
                        global_stats.max_ammo_level += 1;
                        info!("Upgraded Ammo: {}", global_stats.max_ammo_level);
                    }
                }
                DashboardButton::UnlockShotgun => {
                    if global_stats.total_money >= 500 && !global_stats.unlocked_shotgun {
                        global_stats.total_money -= 500;
                        global_stats.unlocked_shotgun = true;
                        info!("Unlocked Shotgun!");
                    }
                }
                DashboardButton::UnlockRifle => {
                    if global_stats.total_money >= 1000 && !global_stats.unlocked_rifle {
                        global_stats.total_money -= 1000;
                        global_stats.unlocked_rifle = true;
                        info!("Unlocked Rifle!");
                    }
                }

                DashboardButton::Quit => {
                    exit.send(AppExit::Success);
                }
            }
        }
    }
}

fn dashboard_visuals(
    mut query: Query<(&Interaction, &DashboardButton, &mut BackgroundColor), With<Button>>,
    settings: Res<crate::plugins::settings::GameSettings>,
    // Add other resources if needed for visual states (like unlocked buttons) but currently only difficulty is stateful
) {
    for (interaction, button, mut color) in &mut query {
        if *interaction == Interaction::Hovered {
            *color = Color::srgb(0.3, 0.3, 0.3).into();
            continue;
        }

        match button {
            DashboardButton::DifficultyEasy => {
                if settings.difficulty == Some(crate::plugins::settings::Difficulty::Easy) {
                    *color = Color::srgb(0.5, 0.5, 0.5).into(); // Gray (Selected)
                } else {
                    *color = Color::srgb(0.2, 0.6, 0.2).into(); // Normal Green
                }
            }
            DashboardButton::DifficultyMed => {
                if settings.difficulty == Some(crate::plugins::settings::Difficulty::Medium) {
                    *color = Color::srgb(0.5, 0.5, 0.5).into(); // Gray (Selected)
                } else {
                    *color = Color::srgb(0.6, 0.6, 0.2).into(); // Yellow
                }
            }
            DashboardButton::DifficultyHard => {
                if settings.difficulty == Some(crate::plugins::settings::Difficulty::Hard) {
                    *color = Color::srgb(0.5, 0.5, 0.5).into(); // Gray (Selected)
                } else {
                    *color = Color::srgb(0.6, 0.2, 0.2).into(); // Red
                }
            }
            DashboardButton::Play => {
                if settings.difficulty.is_none() {
                    *color = Color::srgb(0.1, 0.1, 0.1).into(); // Disabled look
                } else {
                    *color = Color::srgb(0.2, 0.2, 0.2).into(); // Normal
                }
            }
            DashboardButton::UpgradeWeapon => *color = Color::srgb(0.2, 0.2, 0.8).into(),
            DashboardButton::UpgradeAmmo => *color = Color::srgb(0.2, 0.2, 0.8).into(),
            DashboardButton::UnlockShotgun => *color = Color::srgb(0.5, 0.0, 0.5).into(),
            DashboardButton::UnlockRifle => *color = Color::srgb(0.5, 0.0, 0.5).into(),
            _ => *color = Color::srgb(0.2, 0.2, 0.2).into(),
        }
    }
}
