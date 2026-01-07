use bevy::prelude::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    #[default]
    Medium,
    Hard,
}

impl Difficulty {
    pub fn max_waves(&self) -> u32 {
        match self {
            Difficulty::Easy => 1,
            Difficulty::Medium => 2,
            Difficulty::Hard => 3,
        }
    }
}

#[derive(Resource, Default)]
pub struct GameSettings {
    pub difficulty: Option<Difficulty>,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>();
    }
}
