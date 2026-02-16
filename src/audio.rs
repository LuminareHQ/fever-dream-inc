use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct AudioPlugin;

static BACKGROUND_AUDIO: [&str; 14] = [
    "audio/Dark Fantasy Studio- Witchcraft/mp3/1-Dark Fantasy Studio- The story behind her smile.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/2-Dark Fantasy Studio- Communication.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/3-Dark Fantasy Studio-  I'm sorry.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/4-Dark Fantasy Studio-  Sirens.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/5-Dark Fantasy Studio- The crypt.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/6-Dark Fantasy Studio- Between two worlds.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/7-Dark Fantasy Studio- Sacrifice.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/8-Dark Fantasy Studio- As if it comes.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/9-Dark Fantasy Studio- Deep.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/10-Dark Fantasy Studio- Panic attack.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/11-Dark Fantasy Studio- Lost in the maze.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/12-Dark Fantasy Studio- Behind the door.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/13-Dark Fantasy Studio- Paradigm.mp3",
    "audio/Dark Fantasy Studio- Witchcraft/mp3/14-Dark Fantasy Studio- Past and secrets.mp3",
];

pub static PICKUP_AUDIO: &str =
    "audio/splice/ESM_Scifi_UI_Button_Glitch_Morph_Mechanism_Texture_Futuristic.wav";

#[derive(Resource, Debug)]
pub struct AudioState {
    pub volume: f32,
    pub pickup_handle: Option<Handle<bevy_kira_audio::AudioSource>>,
    pub play_pickup: bool,
    pub current_track: usize,
    pub current_track_handle: Option<Handle<AudioInstance>>,
}

#[derive(Resource)]
pub struct InteractionChannel;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioState {
            volume: 0.25,
            current_track: 0,
            pickup_handle: None,
            play_pickup: true,
            current_track_handle: None,
        });
        app.add_plugins(bevy_kira_audio::AudioPlugin);
        app.add_audio_channel::<InteractionChannel>();
        app.add_systems(Startup, start_background_audio);
        app.add_systems(Update, play_next_track_on_end);
        app.add_systems(Update, update_player_volume);
    }
}

fn start_background_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    interaction: Res<AudioChannel<InteractionChannel>>,
    mut state: ResMut<AudioState>,
) {
    state.pickup_handle = Some(asset_server.load(PICKUP_AUDIO));

    interaction.set_volume(-55.0);

    state.current_track = rand::random_range(0..BACKGROUND_AUDIO.len());
    state.current_track_handle = Some(
        audio
            .play(asset_server.load(BACKGROUND_AUDIO[state.current_track]))
            .handle(),
    );
}

fn play_next_track_on_end(
    audio: Res<Audio>,
    mut state: ResMut<AudioState>,
    asset_server: Res<AssetServer>,
) {
    if let Some(handle) = &state.current_track_handle {
        if audio.state(handle) == PlaybackState::Stopped {
            state.current_track = (state.current_track + 1) % BACKGROUND_AUDIO.len();
            state.current_track_handle = Some(
                audio
                    .play(asset_server.load(BACKGROUND_AUDIO[state.current_track]))
                    .handle(),
            );
        }
    }
}

fn update_player_volume(audio: Res<Audio>, state: Res<AudioState>) {
    // Convert linear volume (0.0 to 1.0) to decibels (-80 dB to 0 dB)
    let volume_db = if state.volume <= 0.0 {
        -80.0 // Minimum volume in dB
    } else {
        20.0 * state.volume.log10() // Convert to dB
    };
    audio.set_volume(volume_db);
}
