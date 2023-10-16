use bevy::prelude::*;
use bevy_inspector_egui;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            100.0 / 255.0,
            200.0 / 255.0,
            150.0 / 255.0,
        )))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Decorous".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, simple_setup)
        .run();
}
#[derive(Component)]
struct MainCamera;
fn simple_setup(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), MainCamera));
}
