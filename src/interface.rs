use bevy::{
    color::palettes::css::WHITE,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.insert_resource(InterfaceState {
            hovered_automaton: None,
        });
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_interface);
        app.add_systems(Update, buttons);
    }
}

#[derive(Resource)]
pub struct InterfaceState {
    pub hovered_automaton: Option<crate::data::AutomatonVariant>,
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font_handle = asset_server.load("fonts/Rubik_Dirt/RubikDirt-Regular.ttf");

    commands
        .spawn((
            Node {
                padding: UiRect::all(px(4)),
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![(
                TextLayout::new_with_justify(Justify::Center),
                Name::new("score_text"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 24.0,
                    ..default()
                },
                Text::new("00.00"),
                TextColor(WHITE.into()),
            ),],
        ))
        .insert(Pickable::IGNORE);

    commands
        .spawn((
            Node {
                padding: UiRect::all(px(4)),
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::End,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            children![
                (
                    TextLayout::new_with_justify(Justify::Center),
                    Name::new("hovered_automaton_name_quantity_text"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    Text::new("00.00"),
                    TextColor(WHITE.into()),
                ),
                (
                    TextLayout::new_with_justify(Justify::Center),
                    Name::new("hovered_automoton_rate_total_text"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    Text::new("00.00"),
                    TextColor(WHITE.into()),
                ),
                (
                    TextLayout::new_with_justify(Justify::Center),
                    Name::new("hovered_automaton_cost_text"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    Text::new("00.00"),
                    TextColor(WHITE.into()),
                ),
            ],
        ))
        .insert(Pickable::IGNORE);

    // Only show the FPS counter in debug mode
    #[cfg(debug_assertions)]
    {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(10),
                top: px(10),
                ..default()
            },
            children![(
                TextLayout::new_with_justify(Justify::Center),
                Name::new("fps_text"),
                Text::new("FPS: 0.00"),
                TextColor(WHITE.into()),
            ),],
        ));
    }
}

use crate::{config::get_stats, data::GameData, interface};
fn update_interface(
    data: Res<GameData>,
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&mut Text, &Name), With<Name>>,
    mut interface_data: Res<InterfaceState>,
) {
    let fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS);

    let hovered_rate: f64 = match interface_data.hovered_automaton {
        Some(variant) => {
            use crate::config::get_stats;

            let currency_per_tick = get_stats(variant).currency_per_tick as f64;
            let ticks_per_second = 1.0 / get_stats(variant).cooldown as f64;
            currency_per_tick * ticks_per_second * data.get_quantity_owned_by_source(variant) as f64
        }
        None => 0.0,
    };

    for (mut text, name) in query.iter_mut() {
        match name.as_str() {
            "score_text" => text.0 = format!("{:.2}", data.get_currency()),
            "hovered_automaton_name_quantity_text" => {
                if let Some(source) = interface_data.hovered_automaton.clone() {
                    text.0 = format!(
                        "{}: {}",
                        source,
                        data.get_quantity_owned_by_source(source.clone())
                    );
                } else {
                    text.0 = "".to_string();
                }
            }
            "hovered_automoton_rate_total_text" => {
                if let Some(source) = interface_data.hovered_automaton.clone() {
                    text.0 = format!(
                        "Rate: {}/s, Generated: {}",
                        format!("{:.2}", hovered_rate),
                        data.get_quantity_owned_by_source(source.clone())
                            * data.get_currency_by_source(source.clone())
                    );
                } else {
                    text.0 = "".to_string();
                }
            }
            "hovered_automaton_cost_text" => {
                if let Some(source) = interface_data.hovered_automaton.clone() {
                    text.0 = format!(": ${}", data.get_cost_to_add_source(source.clone()));
                } else {
                    text.0 = "".to_string();
                }
            }
            "fps_text" => {
                if let Some(fps) = fps.and_then(|d| d.value()) {
                    text.0 = format!("FPS: {:.2}", fps);
                }
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
                    if game_data.can_afford_source(crate::data::AutomatonVariant::Hellmite) {
                        game_data.purchase_source(crate::data::AutomatonVariant::Hellmite);
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
