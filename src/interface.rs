use crate::{
    audio::AudioState,
    data::{AutomatonVariant, GameData, UnlockRequirement},
};
use bevy::{
    color::palettes::css::WHITE,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::system::SystemParam,
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
static CONTROL_TITLE_FONT_SIZE: f32 = 22.0;
static CONTROL_TEXT_FONT_SIZE: f32 = 18.0;
static VARIANT_PANEL_BUTTON_FONT_SIZE: f32 = 14.0;
static VARIANT_PANEL_STAT_FONT_SIZE: f32 = 14.0;
const VARIANT_PANEL_STAT_WIDTH: f32 = 110.0;
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
        app.insert_resource(InterfaceState::default());
        app.add_systems(Startup, setup.after(crate::audio::start_background_audio));
        app.add_systems(
            Update,
            (
                update_score,
                update_variant_panel,
                sync_audio_controls,
                update_music_volume_slider_style,
                update_interaction_sound_checkbox_style,
                update_audio_panel_visibility,
            ),
        );
    }
}

#[derive(Resource, Default)]
pub struct InterfaceState {
    pub hovered_automaton: Option<AutomatonVariant>,
    /// Currently-selected variant. Set when a player clicks a purchase ring;
    /// cleared by the close button or by clicking outside the panel.
    pub selected_automaton: Option<AutomatonVariant>,
}

pub fn set_hovered_automaton<E: EntityEvent>(
    new_hovered: Option<AutomatonVariant>,
) -> impl Fn(On<E>, ResMut<InterfaceState>) {
    move |_event, mut interface_state| {
        interface_state.hovered_automaton = new_hovered;
    }
}

#[derive(Component)]
struct VariantPanel;

#[derive(Component)]
struct VariantPanelTitle;

#[derive(Component)]
struct VariantPanelStat(VariantStat);

#[derive(Clone, Copy)]
enum VariantStat {
    Owned,
    Generated,
    Rate,
    Level,
}

#[derive(Component, Clone, Copy)]
enum VariantPanelButton {
    Summon,
    LevelUp,
}

#[derive(Component)]
struct VariantPanelButtonLabel;

#[derive(Component)]
struct AudioControlPanel;

#[derive(Component)]
struct AudioControlPanelExpanded;

#[derive(Component)]
struct AudioControlPanelCollapsed;

#[derive(Component)]
struct AudioControlPanelGearIcon;

#[derive(Component)]
struct AudioPanelAnim {
    progress: f32,
}

const AUDIO_PANEL_COLLAPSED_SIZE: f32 = 32.0;
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

type MusicVolumeSliderThumbQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Node, &'static mut BackgroundColor),
    (With<MusicVolumeSliderThumb>, Without<MusicVolumeSliderFill>),
>;

type MusicVolumeSliderFillQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut Node,
    (With<MusicVolumeSliderFill>, Without<MusicVolumeSliderThumb>),
>;

type AudioPanelQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static Hovered,
        &'static mut AudioPanelAnim,
        &'static mut Node,
    ),
    (
        With<AudioControlPanel>,
        Without<AudioControlPanelExpanded>,
        Without<AudioControlPanelCollapsed>,
    ),
>;

type ExpandedAudioPanelQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static mut Node),
    (
        With<AudioControlPanelExpanded>,
        Without<AudioControlPanelCollapsed>,
        Without<AudioControlPanel>,
    ),
>;

type CollapsedAudioPanelQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Node, Option<&'static Children>),
    (
        With<AudioControlPanelCollapsed>,
        Without<AudioControlPanelExpanded>,
        Without<AudioControlPanel>,
    ),
>;

#[derive(SystemParam)]
struct AudioPanelVisibilityQueries<'w, 's> {
    windows: Query<'w, 's, &'static Window>,
    panels: AudioPanelQuery<'w, 's>,
    children: Query<'w, 's, &'static Children>,
    expanded: ExpandedAudioPanelQuery<'w, 's>,
    collapsed: CollapsedAudioPanelQuery<'w, 's>,
    text_colors: Query<'w, 's, &'static mut TextColor>,
    gear_icons: Query<'w, 's, &'static mut ImageNode, With<AudioControlPanelGearIcon>>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio_state: Res<AudioState>) {
    let font_handle = asset_server.load(FONT_PATH);
    let gear_icon: Handle<Image> = asset_server.load("icons/gear.png");

    commands.spawn((
        Node {
            padding: UiRect::all(px(8)),
            ..default()
        },
        Pickable::IGNORE,
        children![(
            TextLayout::new_with_justify(Justify::Center),
            Name::new("score_text"),
            TextFont {
                font: font_handle.clone(),
                font_size: SCORE_FONT_SIZE,
                ..default()
            },
            Text::new("00.00"),
            TextColor(WHITE.into()),
            Pickable::IGNORE,
        )],
    ));

    spawn_variant_panel(&mut commands, &font_handle);
    spawn_audio_controls(&mut commands, &font_handle, &gear_icon, &audio_state);

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
                Text::new("FPS: 0"),
                TextColor(WHITE.into()),
            ),],
        ));
    }
}

fn spawn_audio_controls(
    commands: &mut Commands,
    font_handle: &Handle<Font>,
    gear_icon: &Handle<Image>,
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
                    btn.spawn((
                        AudioControlPanelGearIcon,
                        Node {
                            width: px(32),
                            height: px(32),
                            ..default()
                        },
                        ImageNode::new(gear_icon.clone()).with_color(CONTROL_TEXT),
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
                        .spawn((Node {
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
    mut thumbs: MusicVolumeSliderThumbQuery<'_, '_>,
    mut fills: MusicVolumeSliderFillQuery<'_, '_>,
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

fn update_score(
    data: Res<GameData>,
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
    mut fps_refresh: Local<f32>,
    mut fps_display: Local<Option<f64>>,
    mut query: Query<(&mut Text, &Name), With<Name>>,
) {
    const FPS_REFRESH_INTERVAL: f32 = 0.5;
    *fps_refresh -= time.delta_secs();
    let fps = if *fps_refresh <= 0.0 {
        *fps_refresh = FPS_REFRESH_INTERVAL;
        let value = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|d| d.smoothed());
        if value.is_some() {
            *fps_display = value;
        }
        *fps_display
    } else {
        *fps_display
    };

    for (mut text, name) in query.iter_mut() {
        match name.as_str() {
            "score_text" => text.0 = format!("{:.2}", data.get_currency()),
            "fps_text" => {
                if let Some(fps) = fps {
                    text.0 = format!("FPS: {:.0}", fps);
                }
            }
            _ => {}
        }
    }
}

fn prereq_not_met(variant: AutomatonVariant, game_data: &GameData) -> String {
    match game_data.unmet_unlock_requirement(variant) {
        Some(UnlockRequirement::PreviousAutomaton {
            variant: required_variant,
            quantity,
        }) => format!(
            "Requires {} {}",
            quantity,
            required_variant.label_for_quantity(quantity)
        ),
        Some(UnlockRequirement::FirstPurchaseCost) => {
            format!(
                "Requires {} Entropy",
                game_data.get_cost_to_add_source(variant)
            )
        }
        Some(UnlockRequirement::None) | None => "Locked".to_string(),
    }
}

fn update_audio_panel_visibility(
    time: Res<Time>,
    mut queries: AudioPanelVisibilityQueries<'_, '_>,
) {
    let dt = time.delta_secs();
    let cursor_in_window = queries
        .windows
        .iter()
        .any(|w| w.cursor_position().is_some());
    for (panel_entity, hovered, mut anim, mut panel_node) in &mut queries.panels {
        let target = if cursor_in_window && hovered.get() {
            1.0
        } else {
            0.0
        };
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

        let mut expanded_entity: Option<Entity> = None;
        for child in queries.children.iter_descendants(panel_entity) {
            if let Ok((entity, mut node)) = queries.expanded.get_mut(child) {
                node.display = if anim.progress > 0.001 {
                    Display::Flex
                } else {
                    Display::None
                };
                expanded_entity = Some(entity);
            }
            if let Ok((mut node, btn_children)) = queries.collapsed.get_mut(child) {
                node.display = if anim.progress < 0.999 {
                    Display::Flex
                } else {
                    Display::None
                };
                let alpha = 1.0 - t;
                if let Some(btn_children) = btn_children {
                    for c in btn_children.iter() {
                        if let Ok(mut color) = queries.text_colors.get_mut(c) {
                            color.0 = CONTROL_TEXT.with_alpha(alpha);
                        }
                        if let Ok(mut image) = queries.gear_icons.get_mut(c) {
                            image.color = CONTROL_TEXT.with_alpha(alpha);
                        }
                    }
                }
            }
        }

        if let Some(expanded_entity) = expanded_entity {
            for descendant in queries.children.iter_descendants(expanded_entity) {
                if let Ok(mut color) = queries.text_colors.get_mut(descendant) {
                    let base = color.0;
                    color.0 = base.with_alpha(t);
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

fn spawn_variant_panel(commands: &mut Commands, font_handle: &Handle<Font>) {
    commands
        .spawn((
            VariantPanel,
            Node {
                position_type: PositionType::Absolute,
                top: px(16),
                left: percent(50),
                margin: UiRect::left(px(-360)),
                width: px(720),
                padding: UiRect::all(px(14)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(6)),
                display: Display::None,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Stretch,
                column_gap: px(16),
                ..default()
            },
            BackgroundColor(PANEL_BACKGROUND),
            BorderColor::all(PANEL_BORDER),
        ))
        .with_children(|panel| {
            // Left column: title + stats
            panel
                .spawn(Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(6),
                    ..default()
                })
                .with_children(|info| {
                    info.spawn((
                        VariantPanelTitle,
                        TextLayout::new_with_justify(Justify::Left).with_no_wrap(),
                        TextFont {
                            font: font_handle.clone(),
                            font_size: CONTROL_TITLE_FONT_SIZE,
                            ..default()
                        },
                        Text::new(""),
                        TextColor(CONTROL_TEXT),
                        Pickable::IGNORE,
                    ));

                    info.spawn(Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: px(8),
                        ..default()
                    })
                    .with_children(|row| {
                        for stat in [
                            VariantStat::Owned,
                            VariantStat::Generated,
                            VariantStat::Rate,
                            VariantStat::Level,
                        ] {
                            row.spawn((
                                Node {
                                    width: px(VARIANT_PANEL_STAT_WIDTH),
                                    flex_shrink: 0.0,
                                    flex_grow: 0.0,
                                    overflow: Overflow::clip(),
                                    ..default()
                                },
                                Pickable::IGNORE,
                            ))
                            .with_children(|slot| {
                                slot.spawn((
                                    VariantPanelStat(stat),
                                    TextLayout::new_with_justify(Justify::Left).with_no_wrap(),
                                    TextFont {
                                        font: font_handle.clone(),
                                        font_size: VARIANT_PANEL_STAT_FONT_SIZE,
                                        ..default()
                                    },
                                    Text::new(""),
                                    TextColor(CONTROL_MUTED_TEXT),
                                    Pickable::IGNORE,
                                ));
                            });
                        }
                    });
                });

            // Right column: action buttons stacked vertically
            panel
                .spawn(Node {
                    width: px(200),
                    flex_shrink: 0.0,
                    flex_grow: 0.0,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    row_gap: px(8),
                    ..default()
                })
                .with_children(|actions| {
                    for action in [VariantPanelButton::Summon, VariantPanelButton::LevelUp] {
                        actions
                            .spawn((
                                action,
                                Node {
                                    width: percent(100),
                                    padding: UiRect::axes(px(10), px(8)),
                                    border: UiRect::all(px(1)),
                                    border_radius: BorderRadius::all(px(4)),
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    overflow: Overflow::clip(),
                                    ..default()
                                },
                                BackgroundColor(PANEL_BACKGROUND),
                                BorderColor::all(PANEL_BORDER),
                                Hovered::default(),
                                observe(on_variant_panel_button),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    VariantPanelButtonLabel,
                                    TextLayout::new_with_justify(Justify::Center).with_no_wrap(),
                                    TextFont {
                                        font: font_handle.clone(),
                                        font_size: VARIANT_PANEL_BUTTON_FONT_SIZE,
                                        ..default()
                                    },
                                    Text::new(""),
                                    TextColor(CONTROL_TEXT),
                                    Pickable::IGNORE,
                                ));
                            });
                    }
                });
        });
}

fn update_variant_panel(
    interface_data: Res<InterfaceState>,
    data: Res<GameData>,
    mut panels: Query<&mut Node, With<VariantPanel>>,
    mut titles: Query<&mut Text, With<VariantPanelTitle>>,
    mut stats: Query<
        (&VariantPanelStat, &mut Text),
        (Without<VariantPanelTitle>, Without<VariantPanelButtonLabel>),
    >,
    mut buttons: Query<
        (
            &VariantPanelButton,
            &Hovered,
            &mut Node,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        Without<VariantPanel>,
    >,
    mut button_labels: Query<
        &mut Text,
        (
            With<VariantPanelButtonLabel>,
            Without<VariantPanelTitle>,
            Without<VariantPanelStat>,
        ),
    >,
    mut label_colors: Query<&mut TextColor, With<VariantPanelButtonLabel>>,
) {
    let Some(source) = interface_data.selected_automaton else {
        for mut node in &mut panels {
            node.display = Display::None;
        }
        return;
    };

    for mut node in &mut panels {
        node.display = Display::Flex;
    }

    let quantity = data.get_quantity_owned_by_source(source);
    for mut text in &mut titles {
        text.0 = if source.is_automaton() {
            source.label_for_quantity(quantity).to_string()
        } else {
            "The Portal".to_string()
        };
    }

    let level = data.get_level(source);
    let multiplier = data.level_multiplier(source);
    let rate = data.rate_per_second_by_source(source);
    let generated = data.get_currency_by_source(source);
    let prereq_met = source.is_automaton() && data.prerequisites_met(source);
    let summon_cost = data.get_cost_to_add_source(source);
    let summon_affordable = source.is_automaton() && prereq_met && data.can_afford_source(source);
    let level_up_cost = data.cost_to_level_up(source);
    let level_up_affordable = data.can_level_up(source);

    for (kind, mut text) in &mut stats {
        text.0 = match kind.0 {
            VariantStat::Owned => {
                if source.is_automaton() {
                    format!("Owned: {}", quantity)
                } else {
                    "".into()
                }
            }
            VariantStat::Generated => format!("Generated: {}", generated),
            VariantStat::Rate => {
                if source.is_automaton() {
                    format!("Rate: {:.2}/s", rate)
                } else {
                    "".into()
                }
            }
            VariantStat::Level => {
                if source.is_automaton() {
                    format!("Level {} (x{:.2})", level, multiplier)
                } else {
                    "".into()
                }
            }
        };
    }

    for (action, hovered, mut node, mut bg, mut border, children) in &mut buttons {
        match action {
            VariantPanelButton::Summon => {
                if !source.is_automaton() {
                    node.display = Display::None;
                    continue;
                }
                node.display = Display::Flex;
                let label = if !prereq_met {
                    prereq_not_met(source, &data)
                } else {
                    format!("Summon ({} Entropy)", summon_cost)
                };
                set_button_label(children, &mut button_labels, &label);

                let active = summon_affordable && hovered.get();
                bg.0 = if active {
                    CONTROL_TRACK
                } else {
                    PANEL_BACKGROUND
                };
                border.set_all(if active {
                    CONTROL_ACCENT_HOVERED
                } else if summon_affordable {
                    CONTROL_ACCENT
                } else {
                    PANEL_BORDER
                });
                for child in children.iter() {
                    if let Ok(mut color) = label_colors.get_mut(child) {
                        color.0 = if summon_affordable {
                            CONTROL_TEXT
                        } else {
                            CONTROL_MUTED_TEXT
                        };
                    }
                }
            }
            VariantPanelButton::LevelUp => {
                if !source.is_automaton() {
                    node.display = Display::None;
                    continue;
                }
                node.display = Display::Flex;
                let label = format!(
                    "Level Up ({} {})",
                    level_up_cost,
                    source.label_for_quantity(level_up_cost)
                );
                set_button_label(children, &mut button_labels, &label);

                let active = level_up_affordable && hovered.get();
                bg.0 = if active {
                    CONTROL_TRACK
                } else {
                    PANEL_BACKGROUND
                };
                border.set_all(if active {
                    CONTROL_ACCENT_HOVERED
                } else if level_up_affordable {
                    CONTROL_ACCENT
                } else {
                    PANEL_BORDER
                });
                for child in children.iter() {
                    if let Ok(mut color) = label_colors.get_mut(child) {
                        color.0 = if level_up_affordable {
                            CONTROL_TEXT
                        } else {
                            CONTROL_MUTED_TEXT
                        };
                    }
                }
            }
        }
    }
}

fn set_button_label(
    children: &Children,
    labels: &mut Query<
        &mut Text,
        (
            With<VariantPanelButtonLabel>,
            Without<VariantPanelTitle>,
            Without<VariantPanelStat>,
        ),
    >,
    new_label: &str,
) {
    for child in children.iter() {
        if let Ok(mut text) = labels.get_mut(child) {
            text.0 = new_label.to_string();
        }
    }
}

fn on_variant_panel_button(
    on: On<Pointer<Click>>,
    actions: Query<&VariantPanelButton>,
    interface_data: Res<InterfaceState>,
    mut data: ResMut<GameData>,
) {
    if on.button != PointerButton::Primary {
        return;
    }
    let Ok(action) = actions.get(on.event_target()) else {
        return;
    };
    let Some(source) = interface_data.selected_automaton else {
        return;
    };
    match action {
        VariantPanelButton::Summon => {
            data.purchase_source(source);
        }
        VariantPanelButton::LevelUp => {
            data.level_up(source);
        }
    }
}
