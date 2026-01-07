use crate::components::Projectile;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::states::GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (player_movement, player_aim, player_shoot, weapon_switching)
                    .run_if(in_state(crate::states::GameState::Playing)),
            );
    }
}

fn weapon_switching(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut WeaponStats, With<Player>>,
    global_stats: Res<crate::plugins::progression::GlobalPlayerStats>,
) {
    if let Ok(mut stats) = query.get_single_mut() {
        if input.just_pressed(KeyCode::Digit1) {
            stats.current_weapon = WeaponType::Pistol;
            info!("Switched to Pistol");
        }
        if input.just_pressed(KeyCode::Digit2) && global_stats.unlocked_shotgun {
            stats.current_weapon = WeaponType::Shotgun;
            info!("Switched to Shotgun");
        }
        if input.just_pressed(KeyCode::Digit3) && global_stats.unlocked_rifle {
            stats.current_weapon = WeaponType::Rifle;
            info!("Switched to Rifle");
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component, Clone, Copy, PartialEq)]
pub enum WeaponType {
    Pistol,
    Shotgun,
    Rifle,
}

#[derive(Component)]
pub struct WeaponStats {
    pub fire_rate: f32,
    pub timer: Timer,
    pub current_ammo: u32,
    pub max_ammo: u32,
    pub current_weapon: WeaponType,
}

fn spawn_player(
    mut commands: Commands,
    global_stats: Res<crate::plugins::progression::GlobalPlayerStats>,
) {
    // Calculate fire rate based on global level
    // Base 0.5, decreases by 10% per level
    let base_rate = 0.5;
    let upgrade_factor = 0.9f32.powi(global_stats.weapon_upgrade_level as i32);
    let final_rate = base_rate * upgrade_factor;

    let base_ammo = 30;
    let max_ammo = base_ammo + (global_stats.max_ammo_level * 10);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.3, 0.3, 1.0), // Blue Player
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Player,
        Speed(150.0),
        crate::components::Health {
            current: 100.0,
            max: 100.0,
        },
        WeaponStats {
            fire_rate: final_rate,
            timer: Timer::from_seconds(final_rate, TimerMode::Repeating),
            current_ammo: max_ammo,
            max_ammo,
            current_weapon: WeaponType::Pistol,
        },
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    wall_query: Query<
        (&Transform, &crate::components::Collider),
        (With<crate::components::Wall>, Without<Player>),
    >,
) {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    for (mut transform, speed) in &mut query {
        let proposed =
            transform.translation + direction.extend(0.0) * speed.0 * time.delta_seconds();
        let player_size = Vec2::new(10.0, 10.0); // 1x1 Pixel Player

        // Simple AABB vs AABB check
        let mut collision = false;
        for (w_transform, w_collider) in &wall_query {
            let p_min = proposed.truncate() - player_size / 2.0;
            let p_max = proposed.truncate() + player_size / 2.0;
            let w_min = w_transform.translation.truncate() - w_collider.size / 2.0;
            let w_max = w_transform.translation.truncate() + w_collider.size / 2.0;

            if p_min.x < w_max.x && p_max.x > w_min.x && p_min.y < w_max.y && p_max.y > w_min.y {
                collision = true;
                break;
            }
        }

        if !collision {
            transform.translation = proposed;
        }
    }
}

// System to rotate player towards mouse cursor
pub fn player_aim(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for mut transform in &mut player_q {
            let player_pos = transform.translation.truncate();
            let diff = world_position - player_pos;
            let angle = diff.y.atan2(diff.x);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

fn player_shoot(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&Transform, &mut WeaponStats), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((transform, mut stats)) = query.get_single_mut() {
        stats.timer.tick(time.delta());

        if mouse_input.pressed(MouseButton::Left)
            && stats.timer.finished()
            && stats.current_ammo > 0
        {
            let aim_dir = transform.rotation * Vec3::X;
            let damage = match stats.current_weapon {
                WeaponType::Pistol => 10.0,
                WeaponType::Shotgun => 25.0,
                WeaponType::Rifle => 20.0,
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 1.0, 0.0), // Yellow Bullet
                        custom_size: Some(Vec2::new(5.0, 5.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(transform.translation),
                    ..default()
                },
                Projectile {
                    velocity: aim_dir.truncate() * 400.0,
                    lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                    damage,
                },
            ));

            stats.current_ammo -= 1;
            stats.timer.reset();

            // Update timer duration in case upgrade happened
            let fire_rate = stats.fire_rate;
            stats
                .timer
                .set_duration(std::time::Duration::from_secs_f32(fire_rate));
            stats.timer.reset();
        } else {
            // Click sound or reload hint?
        }
    }
}
