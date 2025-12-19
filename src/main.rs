use bevy::prelude::*;

fn main() {
    println!("Pac-snake is now.");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Pac-snake".to_string(),
            ..default()
        }),
        ..default()
    }));

    app.run();
}
