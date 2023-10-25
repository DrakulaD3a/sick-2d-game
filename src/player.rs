use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, handle_movement)
            .add_systems(
                Update,
                spawn_bullet.run_if(input_just_pressed(MouseButton::Left)),
            )
            .add_systems(Update, move_bullet);
    }
}

#[derive(Component)]
struct Player;

fn player_setup(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}

fn handle_movement(mut query: Query<&mut Transform, With<Player>>, keys: Res<Input<KeyCode>>) {
    let mut player = query.single_mut();

    let mut translation = Vec2::ZERO;
    if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::D) {
        translation.x += 1.0;
    }
    if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::A) {
        translation.x -= 1.0;
    }
    if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
        translation.y += 1.0;
    }
    if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
        translation.y -= 1.0;
    }

    translation = translation.normalize_or_zero();

    player.translation.x += translation.x;
    player.translation.y += translation.y;
}

#[derive(Component)]
struct Bullet {
    speed: f32,
    angle: f32,
}

fn spawn_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // TODO: Add pollish to this system
    let window = window_query.single();
    let angle = if let Some(cursor_position) = window.cursor_position() {
        (cursor_position.y - window.height() / 2.).atan2(cursor_position.x - window.width() / 2.)
            + std::f32::consts::PI / 2.
    } else {
        0.0
    };

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("bullet.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: player.single().translation,
                rotation: Quat::from_rotation_z(-angle),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bullet { speed: 1.0, angle });
}

fn move_bullet(mut query: Query<(&mut Transform, &Bullet)>) {
    for mut bullet in query.iter_mut() {
        bullet.0.translation.y += bullet.1.speed * bullet.1.angle.cos();
        bullet.0.translation.x += bullet.1.speed * bullet.1.angle.sin();
    }
}
