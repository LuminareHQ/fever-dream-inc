use bevy::{picking::mesh_picking::MeshPickingPlugin, prelude::*};

mod camera;
mod data;
mod environment;
mod interface;
mod materials;
mod portal;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            title: "Fever Dream Incremental".to_string(),
            ..default()
        }),
        ..default()
    }));

    app.insert_resource(data::GameData { currency: 0 });

    app.add_plugins(MeshPickingPlugin);

    app.add_plugins(environment::EnvironmentPlugin);
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(interface::InterfacePlugin);

    app.add_plugins(portal::PortalPlugin);

    app.run();
}
