use bevy::{
    input::keyboard::KeyboardInput, input_focus::InputFocus,
    picking::mesh_picking::MeshPickingPlugin, prelude::*,
};

use crate::data::GameData;

mod automatons;
mod camera;
mod config;
mod data;
mod environment;
mod interface;
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

    app.init_resource::<InputFocus>();

    app.insert_resource(data::GameData::restore());

    app.add_plugins(MeshPickingPlugin);

    app.add_plugins(environment::EnvironmentPlugin);
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(interface::InterfacePlugin);

    app.add_plugins(portal::PortalPlugin);
    app.add_plugins(automatons::HellmitePlugin);
    app.add_plugins(automatons::AbyssopodPlugin);
    app.add_plugins(automatons::GapingDubinePlugin);
    app.add_plugins(automatons::GazingHokuPlugin);
    app.add_plugins(automatons::LorgnerPlugin);
    app.add_plugins(automatons::PelteLacertePlugin);
    app.add_plugins(automatons::StruthiosPlugin);
    app.add_plugins(automatons::WoolyChionoescentPlugin);

    app.add_systems(Update, automatons::update_automatons);

    app.run();
}
