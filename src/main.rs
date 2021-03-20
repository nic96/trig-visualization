mod consts;
mod entity;
mod pause_button;
mod systems;
mod utils;

use crate::entity::{PauseButtonHovered, Paused, Radius, Theta};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[bevy_main]
fn main() {
    let mut app = App::build();

    app.add_resource(WindowDescriptor {
        vsync: true,
        width: 720.,
        height: 720.,
        title: "Trig Visualization".into(),
        ..Default::default()
    })
    .add_resource(ClearColor(Color::rgb_linear(0.01, 0.01, 0.01)))
    .add_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_plugin(ShapePlugin)
        .add_resource(Theta(0.))
        .add_resource(Radius(200.))
        .add_resource(Paused(false))
        .add_resource(PauseButtonHovered(false))
        .init_resource::<pause_button::ButtonMaterials>()
        .add_startup_system(setup.system())
        .add_startup_system(systems::setup.system())
        .add_startup_system(pause_button::setup.system());

    #[cfg(target_arch = "wasm32")]
    app.add_system(systems::web_resize_system.system());

    app.add_system(pause_button::pause_button_system.system())
        .add_system(systems::animate_system.system())
        .add_system(systems::radius_line_system.system())
        .add_system(systems::circle_system.system())
        .add_system(systems::cos_line_system.system())
        .add_system(systems::sin_line_system.system())
        .add_system(systems::tan_line_system.system())
        .add_system(systems::cot_line_system.system())
        .add_system(systems::mouse_click_system.system())
        .add_system(systems::resize_circle_system.system())
        .run();
}

/// set up a simple 3D scene
fn setup(commands: &mut Commands) {
    commands
        // 2d camera
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}
