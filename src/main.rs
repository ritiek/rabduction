use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy::sprite::collide_aabb::collide;
// use bevy_window;

use rand::{thread_rng, Rng};

const GRAVITY: f32 = 0.5;

const PLAYER_SIZE: (f32, f32) = (20., 20.);
const BRICK_SIZE: (f32, f32) = (60., 20.);


#[derive(Default, Debug, PartialEq, Clone)]
struct Materials {
    player_material: Handle<ColorMaterial>,
    brick_material: Handle<ColorMaterial>,
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Player {
    velocity: f32,
    dead: bool,
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Brick;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Scoreboard {
    score: usize,
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(Materials {
        player_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        brick_material: materials.add(Color::rgb(0.7, 0.1, 0.2).into()),
    });
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

fn scoreboard_system(mut transformations: Query<&mut Transform, With<Player>>, player: Query<&Player>, mut scoreboard: ResMut<Scoreboard>, mut query: Query<&mut Text>) {
    if let Some(_) = transformations.iter_mut().next() {
        if let Some(player) = player.iter().next() {
            let mut text = query.single_mut().unwrap();
            if player.dead {
                text.sections[0].value = format!("Final Score: {}", scoreboard.score);
            } else{
                scoreboard.score += 1;
                text.sections[0].value = format!("Score: {}", scoreboard.score);
            }
        }
    }
}

fn spawn_player(commands: &mut Commands, materials: &Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_material.clone(),
            sprite: Sprite::new(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
            transform: Transform::from_translation(
                Vec3::new(0., -350., 0.,)),
            ..Default::default()
        })
        .insert(Player {
            velocity: 25.0,
            dead: false,
        });
}

fn player_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut transformations: Query<&mut Transform,
    With<Player>>,
    mut commands: Commands,
    materials: Res<Materials>) {
    for key_pressed in keyboard_input.get_pressed() {
        if let Some(mut transformation) = transformations.iter_mut().next() {
            match key_pressed {
                KeyCode::Left => transformation.translation.x -= 3.,
                KeyCode::Right => transformation.translation.x += 3.,
                _ => {},
            }
        } else {
            match key_pressed {
                _ => spawn_player(&mut commands, &materials),
            }
        }
    }
}

fn player_movement(mut transformations: Query<&mut Transform, With<Player>>, mut player: Query<&mut Player>) {
    if let Some(mut transformation) = transformations.iter_mut().next() {
        if let Some(mut player) = player.iter_mut().next() {
            player.velocity -= GRAVITY;
            transformation.translation.y += player.velocity;
            if transformation.translation.y < -350. {
                player.dead = true;
            }
        }
    }
}

fn spawn_brick(mut commands: Commands, materials: Res<Materials>) {
    let mut rng = thread_rng();
    let random_pos = rng.gen_range(-150.0..=150.0);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.brick_material.clone(),
            sprite: Sprite::new(Vec2::new(BRICK_SIZE.0, BRICK_SIZE.1)),
            transform: Transform::from_translation(
                Vec3::new(random_pos, 250., 0.,)),
            ..Default::default()
        })
        .insert(Brick);
}

fn brick_movement(mut transformations: Query<&mut Transform, With<Brick>>) {
    for mut transformation in transformations.iter_mut() {
        transformation.translation.y -= 1.;
    }
}

fn is_colliding(player: &Transform, brick: &Transform) -> bool {
    collide(
        player.translation, Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1),
        brick.translation, Vec2::new(BRICK_SIZE.0, BRICK_SIZE.1)
    ).is_some()
}

fn player_brick_collision(
    mut player: Query<&mut Player>,
    player_transformation: Query<&Transform, With<Player>>,
    brick_transformations: Query<&Transform, With<Brick>>
) {
    if let Some(mut player) = player.iter_mut().next() {
        // The collision should happen only when the player is coming down
        if player.velocity < 0. {
            if let Some(player_transformation) = player_transformation.iter().next() {
                for brick_transformation in brick_transformations.iter() {
                    if is_colliding(player_transformation, brick_transformation) {
                        player.velocity = 12.;
                    }
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
