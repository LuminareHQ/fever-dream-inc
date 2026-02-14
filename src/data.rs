use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct GameData {
    pub currency: u64,
}
