use bevy::{prelude::*, window::PrimaryWindow};

const CAMERA_SCALE: f32 = 3.0;
const PLAYER_SPEED: f32 = 80.;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    InGame,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();

        app.add_systems(Startup, setup);
        app.add_systems(OnEnter(GameState::InGame), on_enter_game);
        app.add_systems(Update, (update_player_rotation, update_player_movement).chain());
    }
}

// TODO: Split into multiple files, eventually...

#[derive(Component)]
pub struct Player;

/// On app start...
fn setup(mut commands: Commands) {
    println!("\n\n\n"); // Anti-spam 3000
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1. / CAMERA_SCALE,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

/// When entering game (spawn player, etc.)
fn on_enter_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Player
    let image = asset_server.load("textures/pac.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Player,
        Sprite::from_atlas_image(
            image,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
    ));
}

// TODO: Control scheme enum resource
fn update_player_rotation(
    window: Single<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Player>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera.single().unwrap();

    let Ok(target_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    for mut transform in query.iter_mut() {
        let pos = transform.translation.truncate();
        let angle = Vec2::X.angle_to(target_pos - pos);

        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn update_player_movement(mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let dt = time.delta_secs();

        let movement_direction = (transform.rotation * Vec3::X).normalize();
        let movement_distance = PLAYER_SPEED * dt;
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;
    }
}