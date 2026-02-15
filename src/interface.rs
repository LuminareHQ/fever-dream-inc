use bevy::{color::palettes::css::WHITE, prelude::*};

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_score_text);
        app.add_systems(Update, buttons);
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
                Name::new("score_text"),
                Text::new("00.00"),
                TextColor(WHITE.into()),
            ),],
        ))
        .insert(Pickable::IGNORE);

    commands.spawn((
        Node {
            left: px(10),
            margin: UiRect::vertical(Val::Auto),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![(
            Button,
            Name::new("hellmite_quantity"),
            TextLayout::new_with_justify(Justify::Center),
            Text::new("00.00"),
            TextColor(WHITE.into()),
        ),],
    ));
}

use crate::data::GameData;
fn update_score_text(data: Res<GameData>, mut query: Query<(&mut Text, &Name), With<Name>>) {
    for (mut text, name) in query.iter_mut() {
        match name.as_str() {
            "score_text" => text.0 = format!("{:.2}", data.get_currency()),
            "hellmite_quantity" => {
                text.0 = format!(
                    "Hellmites: {} -> ${}",
                    data.get_quantity_owned_by_source(crate::data::IncomeSource::Hellmite),
                    data.get_cost_to_add_source(crate::data::IncomeSource::Hellmite)
                )
            }
            _ => {
                error!("Unknown text element with name: {}", name);
            }
        }
    }
}

fn buttons(
    mut interaction_query: Query<(Entity, &Interaction, &Name), Changed<Interaction>>,
    mut game_data: ResMut<GameData>,
) {
    for (entity, interaction, name) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match name.as_str() {
                "hellmite_quantity" => {
                    if game_data.can_afford_source(crate::data::IncomeSource::Hellmite) {
                        game_data.purchase_source(crate::data::IncomeSource::Hellmite);
                    }
                }
                _ => {
                    error!("Unknown button with name: {}", name);
                }
            },
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
