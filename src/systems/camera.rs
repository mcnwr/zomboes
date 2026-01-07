use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}
