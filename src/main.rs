use bevy::prelude::*;

fn main() {
    println!("Pac-snake is now.");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            title: "Pac-snake".to_string(),
            ..default()
        }),
        ..default()
    }));

    app.insert_resource(ClearColor(Color::BLACK));

    app.run();
}
