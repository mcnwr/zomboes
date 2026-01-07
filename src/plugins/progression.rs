use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GlobalPlayerStats {
    pub total_money: u32,
    pub level: u32,
    pub weapon_upgrade_level: u32,
    pub max_ammo_level: u32,
    pub unlocked_shotgun: bool,
    pub unlocked_rifle: bool,
}

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalPlayerStats>();
    }
}
