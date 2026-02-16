use crate::{audio::AudioState, config::get_stats, data::GameData};
use bevy::{
    color::palettes::css::WHITE,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

static SCORE_FONT_SIZE: f32 = 32.0;
static OTHER_TEXT_FONT_SIZE: f32 = 24.0;
static FONT_PATH: &str = "fonts/Squada_One/SquadaOne-Regular.ttf";

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.insert_resource(InterfaceState {
            hovered_automaton: None,
        });
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_interface);
        app.add_systems(Update, buttons);
        app.add_systems(EguiPrimaryContextPass, egui_system);
    }
}

use bevy_egui::EguiContexts;
fn egui_system(mut ctx: EguiContexts, mut audio_state: ResMut<AudioState>) -> Result {
    use bevy_egui::egui;
    egui::Window::new("Volume Controls")
        .anchor(egui::Align2::LEFT_BOTTOM, [0., 0.])
        .resizable(false)
        .title_bar(false)
        .movable(false)
        .show(ctx.ctx_mut()?, |ui| {
            ui.heading("Volume Controls");
            ui.label("Music Volume");
            ui.add(egui::Slider::new(&mut audio_state.volume, 0.0..=1.0));
            ui.checkbox(&mut audio_state.play_pickup, "Interaction Sound");
        });
    Ok(())
}

#[derive(Resource)]
pub struct InterfaceState {
    pub hovered_automaton: Option<crate::data::AutomatonVariant>,
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font_handle = asset_server.load(FONT_PATH);

    commands
        .spawn((
            Node {
                padding: UiRect::all(px(4)),
                width: percent(100),
                height: percent(100),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            children![
                (
                    TextLayout::new_with_justify(Justify::Center),
                    Name::new("score_text"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: SCORE_FONT_SIZE,
                        ..default()
                    },
                    Text::new("00.00"),
                    TextColor(WHITE.into()),
                ),
                (
                    TextLayout::new_with_justify(Justify::Center),
                    Name::new("hovered_automaton_name_quantity_text"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: OTHER_TEXT_FONT_SIZE,
                        ..default()
                    },
                    Text::new("00.00"),
                    TextColor(WHITE.into()),
                ),
                (
                    TextLayout::new_with_justify(Justify::Center),
                    Name::new("hovered_automoton_total_text"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: OTHER_TEXT_FONT_SIZE,
                        ..default()
                    },
                    Text::new("00.00"),
                    TextColor(WHITE.into()),
                ),
                (
                    TextLayout::new_with_justify(Justify::Center),
                    Name::new("hovered_automoton_rate_text"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: OTHER_TEXT_FONT_SIZE,
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
                        font_size: OTHER_TEXT_FONT_SIZE,
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
                bottom: px(10),
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

fn update_interface(
    data: Res<GameData>,
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&mut Text, &Name), With<Name>>,
    interface_data: Res<InterfaceState>,
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
                    if source.clone() != crate::data::AutomatonVariant::Portal {
                        text.0 = format!(
                            "{} {}s",
                            data.get_quantity_owned_by_source(source.clone()),
                            source,
                        );
                    } else {
                        text.0 = "The Portal".to_string();
                    }
                } else {
                    text.0 = "".to_string();
                }
            }
            "hovered_automoton_total_text" => {
                if let Some(source) = interface_data.hovered_automaton.clone() {
                    text.0 = format!("Generated: {}", data.get_currency_by_source(source.clone()));
                } else {
                    text.0 = "".to_string();
                }
            }
            "hovered_automoton_rate_text" => {
                if let Some(source) = interface_data.hovered_automaton.clone() {
                    if source.clone() != crate::data::AutomatonVariant::Portal {
                        text.0 = format!("Rate: {}/s", format!("{:.2}", hovered_rate),);
                    }
                } else {
                    text.0 = "".to_string();
                }
            }
            "hovered_automaton_cost_text" => {
                if let Some(source) = interface_data.hovered_automaton.clone() {
                    if source.clone() != crate::data::AutomatonVariant::Portal {
                        if prerequisites_met(source.clone(), &data) {
                            text.0 = format!(
                                "Summon For {}",
                                data.get_cost_to_add_source(source.clone())
                            );
                        } else {
                            text.0 = prereq_not_met(source.clone());
                        }
                    }
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
    for (_, interaction, name) in &mut interaction_query {
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

use crate::data::AutomatonVariant;
pub fn prerequisites_met(variant: AutomatonVariant, game_data: &GameData) -> bool {
    match variant {
        AutomatonVariant::Hellmite => {
            game_data.get_currency() >= get_stats(AutomatonVariant::Hellmite).required_previous
                || game_data.get_quantity_owned_by_source(AutomatonVariant::Hellmite) > 0
        }
        AutomatonVariant::Abyssopod => {
            game_data.get_quantity_owned_by_source(AutomatonVariant::Hellmite)
                >= get_stats(AutomatonVariant::Abyssopod).required_previous
        }
        AutomatonVariant::GapingDubine => {
            game_data.get_quantity_owned_by_source(AutomatonVariant::Abyssopod)
                >= get_stats(AutomatonVariant::GapingDubine).required_previous
        }
        AutomatonVariant::GazingHoku => {
            game_data.get_quantity_owned_by_source(AutomatonVariant::GapingDubine)
                >= get_stats(AutomatonVariant::GazingHoku).required_previous
        }
        AutomatonVariant::Lorgner => {
            game_data.get_quantity_owned_by_source(AutomatonVariant::GazingHoku)
                >= get_stats(AutomatonVariant::Lorgner).required_previous
        }
        AutomatonVariant::PelteLacerte => {
            game_data.get_quantity_owned_by_source(AutomatonVariant::Lorgner)
                >= get_stats(AutomatonVariant::PelteLacerte).required_previous
        }
        AutomatonVariant::Struthios => {
            game_data.get_quantity_owned_by_source(AutomatonVariant::PelteLacerte)
                >= get_stats(AutomatonVariant::Struthios).required_previous
        }
        AutomatonVariant::WoolyChionoescent => {
            game_data.get_quantity_owned_by_source(AutomatonVariant::Struthios)
                >= get_stats(AutomatonVariant::WoolyChionoescent).required_previous
        }
        _ => false,
    }
}

fn prereq_not_met(variant: AutomatonVariant) -> String {
    match variant {
        AutomatonVariant::Hellmite => format!(
            "Requires {} Entropy",
            get_stats(AutomatonVariant::Hellmite).required_previous
        ),
        AutomatonVariant::Abyssopod => format!(
            "Requires {} Hellmites",
            get_stats(AutomatonVariant::Abyssopod).required_previous
        ),
        AutomatonVariant::GapingDubine => format!(
            "Requires {} Abyssopods",
            get_stats(AutomatonVariant::GapingDubine).required_previous
        ),
        AutomatonVariant::GazingHoku => format!(
            "Requires {} Gaping Dubines",
            get_stats(AutomatonVariant::GazingHoku).required_previous
        ),
        AutomatonVariant::Lorgner => format!(
            "Requires {} Gazing Hokues",
            get_stats(AutomatonVariant::Lorgner).required_previous
        ),
        AutomatonVariant::PelteLacerte => format!(
            "Requires {} Lorgners",
            get_stats(AutomatonVariant::PelteLacerte).required_previous
        ),
        AutomatonVariant::Struthios => format!(
            "Requires {} Pelte Lacerte",
            get_stats(AutomatonVariant::Struthios).required_previous
        ),
        AutomatonVariant::WoolyChionoescent => format!(
            "Requires {} Struthios",
            get_stats(AutomatonVariant::WoolyChionoescent).required_previous
        ),
        _ => "Unknown automaton".to_string(),
    }
}
