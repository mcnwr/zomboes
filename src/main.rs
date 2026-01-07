#![allow(clippy::type_complexity)]
use bevy::prelude::*;

mod components;
mod plugins;
mod systems;

use plugins::combat::CombatPlugin;
use plugins::dashboard::DashboardPlugin;
use plugins::enemy::EnemyPlugin;
use plugins::game_over::GameOverPlugin;
use plugins::pause::PausePlugin;
use plugins::player::PlayerPlugin;
use plugins::progression::ProgressionPlugin;
use plugins::settings::SettingsPlugin;
use plugins::shop::ShopPlugin;
use plugins::ui::UiPlugin;
use plugins::wave::WavePlugin;
use plugins::world::WorldPlugin;
use systems::camera::setup_camera;
use systems::rendering::y_sort;

mod states;
use states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zombie Terminate".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(PlayerPlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(WavePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(ShopPlugin)
        .add_plugins(GameOverPlugin)
        .add_plugins(PausePlugin)
        .add_plugins(DashboardPlugin)
        .add_plugins(ProgressionPlugin)
        .add_plugins(SettingsPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, y_sort)
        .run();
}
