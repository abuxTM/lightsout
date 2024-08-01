use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use modules::game::GamePlugin;

mod modules;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Fullscreen,
                    ..default()
                }),
                ..default()
            }), //.set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(0.))
        .add_plugins(RapierDebugRenderPlugin {
            enabled: false,
            ..default()
        })
        .add_plugins((
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::KeyV)),
            GamePlugin,
        ))
        .add_systems(
            Update,
            toggle_rapier_debug.run_if(input_toggle_active(false, KeyCode::KeyB)),
        )
        .run();
}

fn toggle_rapier_debug(mut debug_context: ResMut<DebugRenderContext>) {
    debug_context.enabled = !debug_context.enabled;
}
