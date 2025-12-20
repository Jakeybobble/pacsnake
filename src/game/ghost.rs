use bevy::prelude::*;
use rand::prelude::*;

pub fn ghost_plugin(app: &mut App) {
    app.add_observer(on_spawn_ghost);
}

#[derive(Component)]
pub struct Ghost;

#[derive(Event, Deref)]
pub struct SpawnGhost(pub Vec2);

fn on_spawn_ghost(
    spawn_ghost: On<SpawnGhost>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load("textures/ghosts.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut rng = rand::rng();

    commands.spawn((
        Ghost,
        Transform::from_translation(spawn_ghost.extend(0.)),
        Sprite::from_atlas_image(
            image,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: rng.random_range(0..=3),
            },
        ),
    ));

}
