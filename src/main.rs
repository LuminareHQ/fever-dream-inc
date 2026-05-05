use bevy::{input_focus::InputFocus, picking::mesh_picking::MeshPickingPlugin, prelude::*};

mod audio;
mod automatons;
mod camera;
mod data;
mod environment;
mod interface;
mod portal;
mod rand;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            title: "REMM".to_string(),
            ..default()
        }),
        ..default()
    }));

    app.init_resource::<InputFocus>();

    app.insert_resource(data::GameData::restore());

    app.add_plugins(MeshPickingPlugin);
    app.insert_resource(MeshPickingSettings {
        require_markers: false,
        ..default()
    });

    app.add_plugins(audio::AudioPlugin);

    app.add_plugins(environment::EnvironmentPlugin);
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(interface::InterfacePlugin);

    app.add_plugins(portal::PortalPlugin);
    app.add_plugins(automatons::AutomatonsPlugin);

    app.run();
}
