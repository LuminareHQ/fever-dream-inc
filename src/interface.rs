use crate::{
    audio::AudioState,
    automatons::{is_automaton_variant, previous_automaton_variant},
    config::get_stats,
    data::{AutomatonVariant, GameData},
};
use bevy::{
    color::palettes::css::WHITE,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input_focus::{
        InputDispatchPlugin,
        tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
    },
    picking::hover::Hovered,
    prelude::*,
    ui::Checked,
    ui_widgets::{
        Checkbox, CoreSliderDragState, Slider, SliderPrecision, SliderRange, SliderStep,
        SliderThumb, SliderValue, TrackClick, UiWidgetsPlugins, ValueChange, observe,
    },
};

static SCORE_FONT_SIZE: f32 = 32.0;
static OTHER_TEXT_FONT_SIZE: f32 = 24.0;
static CONTROL_TITLE_FONT_SIZE: f32 = 22.0;
static CONTROL_TEXT_FONT_SIZE: f32 = 18.0;
static FONT_PATH: &str = "fonts/Squada_One/SquadaOne-Regular.ttf";

const PANEL_BACKGROUND: Color = Color::srgba(0.03, 0.02, 0.04, 1.0);
const PANEL_BORDER: Color = Color::srgba(0.72, 0.62, 0.95, 0.45);
const CONTROL_TEXT: Color = Color::srgb(0.94, 0.91, 1.0);
const CONTROL_MUTED_TEXT: Color = Color::srgb(0.72, 0.68, 0.78);
const CONTROL_TRACK: Color = Color::srgba(0.12, 0.10, 0.16, 0.92);
const CONTROL_ACCENT: Color = Color::srgb(0.76, 0.38, 0.86);
const CONTROL_ACCENT_HOVERED: Color = Color::srgb(0.94, 0.52, 0.88);
const CHECKBOX_BORDER: Color = Color::srgb(0.55, 0.50, 0.62);
const CHECKBOX_BORDER_HOVERED: Color = Color::srgb(0.74, 0.70, 0.82);
const CLEAR: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            UiWidgetsPlugins,
            InputDispatchPlugin,
            TabNavigationPlugin,
        ));
        app.insert_resource(InterfaceState {
            hovered_automaton: None,
        });
        app.add_systems(Startup, setup.after(crate::audio::start_background_audio));
        app.add_systems(
            Update,
            (
                update_interface,
                sync_audio_controls,
                update_music_volume_slider_style,
                update_interaction_sound_checkbox_style,
                update_audio_panel_visibility,
            ),
        );
        app.add_systems(Update, buttons);
    }
}

#[derive(Resource)]
pub struct InterfaceState {
    pub hovered_automaton: Option<AutomatonVariant>,
}

#[derive(Component)]
struct AudioControlPanel;

#[derive(Component)]
struct AudioControlPanelExpanded;

#[derive(Component)]
struct AudioControlPanelCollapsed;

#[derive(Component)]
struct AudioPanelAnim {
    progress: f32,
}

const AUDIO_PANEL_COLLAPSED_SIZE: f32 = 22.0;
const AUDIO_PANEL_EXPANDED_WIDTH: f32 = 286.0;
const AUDIO_PANEL_EXPANDED_HEIGHT: f32 = 110.0;
const AUDIO_PANEL_ANIM_SPEED: f32 = 6.0;

#[derive(Component, Default)]
struct MusicVolumeSlider;

#[derive(Component, Default)]
struct MusicVolumeSliderFill;

#[derive(Component, Default)]
struct MusicVolumeSliderThumb;

#[derive(Component)]
struct MusicVolumeValueText;

#[derive(Component, Default)]
struct InteractionSoundCheckbox;

#[derive(Component, Default)]
struct InteractionSoundCheckboxBox;

#[derive(Component, Default)]
struct InteractionSoundCheckboxMark;

#[derive(Component)]
struct InteractionSoundValueText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio_state: Res<AudioState>) {
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

    spawn_audio_controls(&mut commands, &font_handle, &audio_state);

    // Only show the FPS counter in debug mode
    #[cfg(debug_assertions)]
    {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(10),
                bottom: px(128),
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

fn spawn_audio_controls(
    commands: &mut Commands,
    font_handle: &Handle<Font>,
    audio_state: &AudioState,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(12),
                bottom: px(12),
                width: px(AUDIO_PANEL_COLLAPSED_SIZE),
                height: px(AUDIO_PANEL_COLLAPSED_SIZE),
                padding: UiRect::all(px(0)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(4)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(PANEL_BACKGROUND),
            BorderColor::all(PANEL_BORDER),
            TabGroup::default(),
            AudioControlPanel,
            AudioPanelAnim { progress: 0.0 },
            Hovered::default(),
        ))
        .with_children(|panel| {
            panel
                .spawn((
                    AudioControlPanelCollapsed,
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(0),
                        top: px(0),
                        width: px(AUDIO_PANEL_COLLAPSED_SIZE),
                        height: px(AUDIO_PANEL_COLLAPSED_SIZE),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|btn| {
                    btn.spawn(control_text(
                        font_handle,
                        "\u{2699}",
                        CONTROL_TEXT_FONT_SIZE,
                        CONTROL_TEXT,
                    ));
                });

            panel
                .spawn((
                    AudioControlPanelExpanded,
                    Node {
                        width: px(AUDIO_PANEL_EXPANDED_WIDTH - 22.0),
                        padding: UiRect::all(px(10)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(8),
                        ..default()
                    },
                ))
                .with_children(|expanded| {
                    expanded.spawn(control_text(
                        font_handle,
                        "Audio Settings",
                        CONTROL_TITLE_FONT_SIZE,
                        CONTROL_TEXT,
                    ));

            expanded
                .spawn((
                    Node {
                    width: percent(100),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: px(8),
                    ..default()
                },))
                .with_children(|row| {
                    row.spawn((
                        Node {
                            width: px(52),
                            ..default()
                        },
                        control_text(
                            font_handle,
                            "Music",
                            CONTROL_TEXT_FONT_SIZE,
                            CONTROL_MUTED_TEXT,
                        ),
                    ));
                    row.spawn(music_volume_slider(audio_state.volume));
                    row.spawn((
                        Node {
                            width: px(42),
                            justify_content: JustifyContent::FlexEnd,
                            ..default()
                        },
                        MusicVolumeValueText,
                        control_text(
                            font_handle,
                            music_volume_label(audio_state.volume),
                            CONTROL_TEXT_FONT_SIZE,
                            CONTROL_TEXT,
                        ),
                    ));
                });

            let mut checkbox = expanded.spawn((
                Node {
                    width: percent(100),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: px(8),
                    ..default()
                },
                InteractionSoundCheckbox,
                Checkbox,
                Hovered::default(),
                TabIndex(1),
                observe(update_interaction_sound_from_checkbox),
            ));
            if audio_state.play_pickup {
                checkbox.insert(Checked);
            }
            checkbox.with_children(|row| {
                row.spawn((
                    InteractionSoundCheckboxBox,
                    Node {
                        width: px(16),
                        height: px(16),
                        margin: UiRect::right(px(8)),
                        border: UiRect::all(px(2)),
                        border_radius: BorderRadius::all(px(3)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor::all(CHECKBOX_BORDER),
                ))
                .with_children(|checkbox_box| {
                    checkbox_box.spawn((
                        InteractionSoundCheckboxMark,
                        Node {
                            width: px(8),
                            height: px(8),
                            border_radius: BorderRadius::all(px(2)),
                            ..default()
                        },
                        BackgroundColor(if audio_state.play_pickup {
                            CONTROL_ACCENT
                        } else {
                            CLEAR
                        }),
                    ));
                });
                row.spawn(control_text(
                    font_handle,
                    "Interaction Sound",
                    CONTROL_TEXT_FONT_SIZE,
                    CONTROL_MUTED_TEXT,
                ));
                row.spawn((
                    Node {
                        margin: UiRect::left(auto()),
                        width: px(28),
                        justify_content: JustifyContent::FlexEnd,
                        ..default()
                    },
                    InteractionSoundValueText,
                    control_text(
                        font_handle,
                        interaction_sound_label(audio_state.play_pickup),
                        CONTROL_TEXT_FONT_SIZE,
                        CONTROL_TEXT,
                    ),
                ));
            });
                });
        });
}

fn control_text(
    font_handle: &Handle<Font>,
    text: impl Into<String>,
    font_size: f32,
    color: Color,
) -> impl Bundle {
    (
        Text::new(text.into()),
        TextFont {
            font: font_handle.clone(),
            font_size,
            ..default()
        },
        TextColor(color),
    )
}

fn music_volume_slider(value: f32) -> impl Bundle {
    let value = value.clamp(0.0, 1.0);

    (
        Node {
            width: px(132),
            height: px(18),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            ..default()
        },
        MusicVolumeSlider,
        Slider {
            track_click: TrackClick::Snap,
        },
        SliderValue(value),
        SliderRange::new(0.0, 1.0),
        SliderStep(0.05),
        SliderPrecision(2),
        Hovered::default(),
        TabIndex(0),
        observe(update_music_volume_from_slider),
        Children::spawn((
            Spawn((
                Node {
                    height: px(6),
                    border_radius: BorderRadius::all(px(3)),
                    ..default()
                },
                BackgroundColor(CONTROL_TRACK),
                children![(
                    MusicVolumeSliderFill,
                    Node {
                        width: percent(value * 100.0),
                        height: percent(100),
                        border_radius: BorderRadius::all(px(3)),
                        ..default()
                    },
                    BackgroundColor(CONTROL_ACCENT),
                )],
            )),
            Spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(16),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                children![(
                    MusicVolumeSliderThumb,
                    SliderThumb,
                    Node {
                        width: px(16),
                        height: px(16),
                        position_type: PositionType::Absolute,
                        left: percent(value * 100.0),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(CONTROL_ACCENT),
                )],
            )),
        )),
    )
}

fn update_music_volume_from_slider(
    value_change: On<ValueChange<f32>>,
    mut commands: Commands,
    mut audio_state: ResMut<AudioState>,
    mut game_data: ResMut<GameData>,
) {
    let volume = value_change.value.clamp(0.0, 1.0);
    audio_state.volume = volume;
    game_data.update_audio_settings(volume, audio_state.play_pickup);
    commands
        .entity(value_change.source)
        .insert(SliderValue(volume));
}

fn update_interaction_sound_from_checkbox(
    value_change: On<ValueChange<bool>>,
    mut commands: Commands,
    mut audio_state: ResMut<AudioState>,
    mut game_data: ResMut<GameData>,
) {
    audio_state.play_pickup = value_change.value;
    game_data.update_audio_settings(audio_state.volume, value_change.value);
    if value_change.value {
        commands.entity(value_change.source).insert(Checked);
    } else {
        commands.entity(value_change.source).remove::<Checked>();
    }
}

fn sync_audio_controls(
    mut commands: Commands,
    audio_state: Res<AudioState>,
    sliders: Query<(Entity, &SliderValue), With<MusicVolumeSlider>>,
    checkboxes: Query<(Entity, Has<Checked>), With<InteractionSoundCheckbox>>,
    mut volume_texts: Query<
        &mut Text,
        (
            With<MusicVolumeValueText>,
            Without<InteractionSoundValueText>,
        ),
    >,
    mut interaction_texts: Query<
        &mut Text,
        (
            With<InteractionSoundValueText>,
            Without<MusicVolumeValueText>,
        ),
    >,
) {
    if !audio_state.is_changed() {
        return;
    }

    let volume = audio_state.volume.clamp(0.0, 1.0);
    for (slider, slider_value) in &sliders {
        if (slider_value.0 - volume).abs() > 0.001 {
            commands.entity(slider).insert(SliderValue(volume));
        }
    }

    for (checkbox, checked) in &checkboxes {
        if audio_state.play_pickup != checked {
            if audio_state.play_pickup {
                commands.entity(checkbox).insert(Checked);
            } else {
                commands.entity(checkbox).remove::<Checked>();
            }
        }
    }

    let volume_label = music_volume_label(volume);
    for mut text in &mut volume_texts {
        text.0 = volume_label.clone();
    }

    let sound_label = interaction_sound_label(audio_state.play_pickup);
    for mut text in &mut interaction_texts {
        text.0 = sound_label.to_string();
    }
}

fn update_music_volume_slider_style(
    sliders: Query<
        (
            Entity,
            &SliderValue,
            &SliderRange,
            &Hovered,
            &CoreSliderDragState,
        ),
        With<MusicVolumeSlider>,
    >,
    children: Query<&Children>,
    mut thumbs: Query<
        (&mut Node, &mut BackgroundColor),
        (With<MusicVolumeSliderThumb>, Without<MusicVolumeSliderFill>),
    >,
    mut fills: Query<&mut Node, (With<MusicVolumeSliderFill>, Without<MusicVolumeSliderThumb>)>,
) {
    for (slider, value, range, hovered, drag_state) in &sliders {
        let amount = range.thumb_position(value.0).clamp(0.0, 1.0);
        let thumb_color = if hovered.get() || drag_state.dragging {
            CONTROL_ACCENT_HOVERED
        } else {
            CONTROL_ACCENT
        };

        for child in children.iter_descendants(slider) {
            if let Ok((mut node, mut background)) = thumbs.get_mut(child) {
                node.left = percent(amount * 100.0);
                background.0 = thumb_color;
            }
            if let Ok(mut node) = fills.get_mut(child) {
                node.width = percent(amount * 100.0);
            }
        }
    }
}

fn update_interaction_sound_checkbox_style(
    checkboxes: Query<(&Hovered, Has<Checked>), With<InteractionSoundCheckbox>>,
    mut boxes: Query<&mut BorderColor, With<InteractionSoundCheckboxBox>>,
    mut marks: Query<&mut BackgroundColor, With<InteractionSoundCheckboxMark>>,
) {
    let Some((hovered, checked)) = checkboxes.iter().next() else {
        return;
    };

    let border_color = if hovered.get() {
        CHECKBOX_BORDER_HOVERED
    } else {
        CHECKBOX_BORDER
    };
    for mut border in &mut boxes {
        border.set_all(border_color);
    }

    let mark_color = if checked { CONTROL_ACCENT } else { CLEAR };
    for mut mark in &mut marks {
        mark.0 = mark_color;
    }
}

fn music_volume_label(volume: f32) -> String {
    format!("{:.0}%", volume.clamp(0.0, 1.0) * 100.0)
}

fn interaction_sound_label(play_pickup: bool) -> &'static str {
    if play_pickup { "On" } else { "Off" }
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
                if let Some(source) = interface_data.hovered_automaton {
                    if source != AutomatonVariant::Portal {
                        text.0 =
                            format!("{} {}s", data.get_quantity_owned_by_source(source), source);
                    } else {
                        text.0 = "The Portal".to_string();
                    }
                } else {
                    text.0 = "".to_string();
                }
            }
            "hovered_automoton_total_text" => {
                if let Some(source) = interface_data.hovered_automaton {
                    text.0 = format!("Generated: {}", data.get_currency_by_source(source));
                } else {
                    text.0 = "".to_string();
                }
            }
            "hovered_automoton_rate_text" => {
                if let Some(source) = interface_data.hovered_automaton {
                    if source != AutomatonVariant::Portal {
                        text.0 = format!("Rate: {}/s", format!("{:.2}", hovered_rate),);
                    }
                } else {
                    text.0 = "".to_string();
                }
            }
            "hovered_automaton_cost_text" => {
                if let Some(source) = interface_data.hovered_automaton {
                    if source != AutomatonVariant::Portal {
                        if prerequisites_met(source, &data) {
                            text.0 = format!("Summon For {}", data.get_cost_to_add_source(source));
                        } else {
                            text.0 = prereq_not_met(source);
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

pub fn prerequisites_met(variant: AutomatonVariant, game_data: &GameData) -> bool {
    if !is_automaton_variant(variant) {
        return false;
    }

    let required_previous = get_stats(variant).required_previous;
    if let Some(previous) = previous_automaton_variant(variant) {
        game_data.get_quantity_owned_by_source(previous) >= required_previous
    } else {
        game_data.get_currency() >= required_previous
            || game_data.get_quantity_owned_by_source(variant) > 0
    }
}

fn prereq_not_met(variant: AutomatonVariant) -> String {
    let required_previous = get_stats(variant).required_previous;
    if let Some(previous) = previous_automaton_variant(variant) {
        format!(
            "Requires {} {}",
            required_previous,
            automaton_quantity_label(previous, required_previous)
        )
    } else if is_automaton_variant(variant) {
        format!("Requires {} Entropy", required_previous)
    } else {
        "Unknown automaton".to_string()
    }
}

fn automaton_quantity_label(variant: AutomatonVariant, quantity: u64) -> String {
    if quantity == 1 {
        variant.to_string()
    } else {
        format!("{}s", variant)
    }
}

fn update_audio_panel_visibility(
    time: Res<Time>,
    windows: Query<&Window>,
    mut panel_query: Query<
        (Entity, &Hovered, &mut AudioPanelAnim, &mut Node),
        (
            With<AudioControlPanel>,
            Without<AudioControlPanelExpanded>,
            Without<AudioControlPanelCollapsed>,
        ),
    >,
    children_query: Query<&Children>,
    mut expanded_query: Query<
        &mut Node,
        (
            With<AudioControlPanelExpanded>,
            Without<AudioControlPanelCollapsed>,
            Without<AudioControlPanel>,
        ),
    >,
    mut collapsed_query: Query<
        (&mut Node, Option<&Children>),
        (
            With<AudioControlPanelCollapsed>,
            Without<AudioControlPanelExpanded>,
            Without<AudioControlPanel>,
        ),
    >,
    mut text_color_query: Query<&mut TextColor>,
) {
    let dt = time.delta_secs();
    let cursor_in_window = windows
        .iter()
        .any(|w| w.cursor_position().is_some());
    for (panel_entity, hovered, mut anim, mut panel_node) in &mut panel_query {
        let target = if cursor_in_window && hovered.get() { 1.0 } else { 0.0 };
        let delta = (target - anim.progress) * AUDIO_PANEL_ANIM_SPEED * dt;
        anim.progress = (anim.progress + delta).clamp(0.0, 1.0);
        if (target - anim.progress).abs() < 0.001 {
            anim.progress = target;
        }
        let t = ease_in_out(anim.progress);

        let width = lerp(AUDIO_PANEL_COLLAPSED_SIZE, AUDIO_PANEL_EXPANDED_WIDTH, t);
        let height = lerp(AUDIO_PANEL_COLLAPSED_SIZE, AUDIO_PANEL_EXPANDED_HEIGHT, t);
        panel_node.width = px(width);
        panel_node.height = px(height);

        for child in children_query.iter_descendants(panel_entity) {
            if let Ok(mut node) = expanded_query.get_mut(child) {
                node.display = if anim.progress > 0.001 {
                    Display::Flex
                } else {
                    Display::None
                };
            }
            if let Ok((mut node, btn_children)) = collapsed_query.get_mut(child) {
                node.display = if anim.progress < 0.999 {
                    Display::Flex
                } else {
                    Display::None
                };
                let alpha = 1.0 - t;
                if let Some(btn_children) = btn_children {
                    for c in btn_children.iter() {
                        if let Ok(mut color) = text_color_query.get_mut(c) {
                            color.0 = CONTROL_TEXT.with_alpha(alpha);
                        }
                    }
                }
            }
        }
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}
