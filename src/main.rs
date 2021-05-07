use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy::sprite::collide_aabb::collide;
// use bevy_window;

use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

const GRAVITY: f32 = 0.5;
const MAX_INPUT_SPEED: f32 = 3.0;

#[derive(Default, Debug, PartialEq, Clone)]
struct Sounds {
    collisions: Vec<Handle<AudioSource>>,
    background: Handle<AudioSource>
}

#[derive(Default, Debug, PartialEq, Clone)]
struct Player {
    material: Handle<ColorMaterial>,
    size: (f32, f32),
    velocity: f32,
    has_spawned: bool,
    dead: bool,
}

#[derive(Default, Debug, PartialEq, Clone)]
struct Brick {
    material: Handle<ColorMaterial>,
    size: (f32, f32),
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Scoreboard {
    score: usize,
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands.insert_resource(Player {
        material: materials.add(asset_server.load("sprites/tux-1.png").into()),
        size: (25.0, 40.0),
        velocity: 25.0,
        has_spawned: false,
        dead: false,
    });
    commands.insert_resource(
        vec![
            Brick {
                material: materials.add(asset_server.load("sprites/platform-1.png").into()),
                size: (60.0, 20.0),
            },
            Brick {
                material: materials.add(asset_server.load("sprites/platform-2.png").into()),
                size: (90.0, 20.0),
            },
        ]
    );
    commands.insert_resource(
        Sounds {
            collisions: vec![
                asset_server.load("sounds/coll-1.mp3"),
                asset_server.load("sounds/coll-2.mp3"),
                asset_server.load("sounds/coll-3.mp3"),
                asset_server.load("sounds/coll-4.mp3"),
            ],
            // FIXME: Replace with some actual background music
            background: asset_server.load("sounds/coll-4.mp3"),
        }
    );
    commands.insert_resource(Scoreboard {
        score: 0,
    });

    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: 0".to_string(),
                    style: TextStyle {
                        font: font_handle.clone(),
                        font_size: 25.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn scoreboard_system(mut transformations: Query<&mut Transform, With<Player>>, player: Res<Player>, mut scoreboard: ResMut<Scoreboard>, mut query: Query<&mut Text>) {
    if let Some(_) = transformations.iter_mut().next() {
        let mut text = query.single_mut().unwrap();
        if player.dead {
            text.sections[0].value = format!("Final Score: {}", scoreboard.score);
        } else {
            scoreboard.score += 1;
            text.sections[0].value = format!("Score: {}", scoreboard.score);
        }
    }
}

fn spawn_player(commands: &mut Commands, player: &ResMut<Player>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: player.material.clone(),
            sprite: Sprite::new(Vec2::new(player.size.0, player.size.1)),
            transform: Transform::from_translation(
                Vec3::new(0., -350., 0.,)),
            ..Default::default()
        })
        .insert(Player {
            material: player.material.clone(),
            size: player.size,
            velocity: player.velocity,
            has_spawned: player.has_spawned,
            dead: player.dead,
        });
}

fn player_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    gamepad_input: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut transformations: Query<&mut Transform,
    With<Player>>,
    mut commands: Commands,
    mut player: ResMut<Player>) {

    let mut spawn = false;
    for _ in gamepad_input.get_pressed() {
        if player.has_spawned {
            break;
        } else {
            player.has_spawned = true;
            spawn = true;
            spawn_player(&mut commands, &player);
        }
    }

    let mut movement_value: Option<f32> = None;
    if let Some(value) = axes.get(GamepadAxis(Gamepad(0), GamepadAxisType::LeftStickX)) {
        movement_value = Some(value * MAX_INPUT_SPEED);
    }
    for key_pressed in keyboard_input.get_pressed() {
        match key_pressed {
            KeyCode::Left => { movement_value = Some(MAX_INPUT_SPEED * -1.0); },
            KeyCode::Right => { movement_value = Some(MAX_INPUT_SPEED); },
            _ => {
                if !spawn && !player.has_spawned {
                    player.has_spawned = true;
                    spawn = true;
                    spawn_player(&mut commands, &player);
                }
            },
        };
    }
    if let Some(movement_value) = movement_value {
        if let Some(mut transformation) = transformations.iter_mut().next() {
            transformation.translation.x += movement_value;
            if movement_value < 0.0 {
                transformation.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            } else if movement_value > 0.0 {
                transformation.rotation = Quat::default();
            }
        }
    }
}

fn player_movement(mut transformations: Query<&mut Transform, With<Player>>, mut player: ResMut<Player>) {
    if let Some(mut transformation) = transformations.iter_mut().next() {
        player.velocity -= GRAVITY;
        transformation.translation.y += player.velocity;
        if transformation.translation.y < -400. {
            player.dead = true;
        }
    }
}

fn spawn_brick(mut commands: Commands, bricks: Res<Vec<Brick>>) {
    let mut rng = thread_rng();
    let random_pos = rng.gen_range(-150.0..=150.0);
    let brick = bricks.choose(&mut rand::thread_rng()).unwrap();
    commands
        .spawn_bundle(SpriteBundle {
            material: brick.material.clone(),
            sprite: Sprite::new(Vec2::new(brick.size.0, brick.size.1)),
            transform: Transform::from_translation(
                Vec3::new(random_pos, 250., 0.,)),
            ..Default::default()
        })
        .insert(Brick {
            material: brick.material.clone(),
            size: brick.size.clone(),
        });
}

fn brick_movement(mut transformations: Query<&mut Transform, With<Brick>>) {
    for mut transformation in transformations.iter_mut() {
        transformation.translation.y -= 1.;
    }
}

fn is_colliding(player: &Transform, player_size: (f32, f32), brick: &Transform, brick_size: (f32, f32)) -> bool {
    collide(
        player.translation, Vec2::new(player_size.0, player_size.1),
        brick.translation, Vec2::new(brick_size.0, brick_size.1)
    ).is_some()
}

fn player_brick_collision(
    mut player: ResMut<Player>,
    bricks: Query<&Brick>,
    player_transformation: Query<&Transform, With<Player>>,
    brick_transformations: Query<&Transform, With<Brick>>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    // The collision should happen only when the player falls on a platform from above
    if player.velocity < 0. {
        if let Some(player_transformation) = player_transformation.iter().next() {
            for (brick, brick_transformation) in bricks.iter().zip(brick_transformations.iter()) {
                if is_colliding(player_transformation, player.size, brick_transformation, brick.size) {
                    player.velocity = 12.;
                    let collision_sound = sounds.collisions.choose(&mut rand::thread_rng()).unwrap();
                    audio.play(collision_sound.clone());
                }
            }
        }
    }
}

fn main() {
    App::build()
        .insert_resource(Msaa {
            samples: 4,
        })
        .insert_resource(WindowDescriptor {
            title: "Rabduction!".to_string(),
            width: 500.0,
            height: 800.0,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.5))
                .with_system(spawn_brick.system()),
        )
        .add_system(player_movement_input.system())
        .add_system(player_movement.system())
        .add_system(brick_movement.system())
        .add_system(player_brick_collision.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(scoreboard_system.system()),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
