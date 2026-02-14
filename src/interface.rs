use bevy::{asset, color::palettes::css::WHITE, prelude::*, ui::update};

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_score_text);
    }
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![(
                TextLayout::new_with_justify(Justify::Center),
                Name::new("Score Text"),
                Text::new("00.00"),
                TextFont {
                    font: asset_server.load("fonts/Rubik_Glitch/RubikGlitch-Regular.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(WHITE.into()),
            )],
        ))
        .insert(Pickable::IGNORE);
}

use crate::data::GameData;
fn update_score_text(
    data: Res<GameData>,
    mut query: Query<&mut Text, With<Name>>,
    time: Res<Time>,
) {
    for mut text in query.iter_mut() {
        text.0 = data.get_currency().to_string();
    }
}
