use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy::sprite::collide_aabb::collide;

use std::{thread, time};
use rand::{thread_rng, Rng};

const GRAVITY: f32 = 0.5;

const PLAYER_SIZE: (f32, f32) = (20., 20.);
const BRICK_SIZE: (f32, f32) = (60., 20.);


#[derive(Debug, PartialEq, Clone)]
struct Materials {
    player_material: Handle<ColorMaterial>,
    brick_material: Handle<ColorMaterial>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Player {
    velocity: f32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Brick;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        player_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        brick_material: materials.add(Color::rgb(0.7, 0.1, 0.2).into()),
    });
}

fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    // FIXME: This is a hack allowing us to spawn the player after the bricks
    //        start showing up. However, this causes the initial window to stay
    //        stuck for 3s. Need to find a better way to achieve this.
    thread::sleep(time::Duration::from_secs(3));
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
        });
}

fn player_movement_input(keyboard_input: Res<Input<KeyCode>>, mut transformations: Query<&mut Transform, With<Player>>) {
    for key_pressed in keyboard_input.get_pressed() {
        // XXX: Not sure which one is the better choice here:
        // for mut transformation in animals.iter_mut() {
        if let Some(mut transformation) = transformations.iter_mut().next() {
            match key_pressed {
                KeyCode::Left => transformation.translation.x -= 3.,
                KeyCode::Right => transformation.translation.x += 3.,
                _ => {},
            }
        }
    }
}

fn player_movement(mut transformations: Query<&mut Transform, With<Player>>, mut player: Query<&mut Player>) {
    if let Some(mut transformation) = transformations.iter_mut().next() {
        if let Some(mut player) = player.iter_mut().next() {
            player.velocity -= GRAVITY;
            transformation.translation.y += player.velocity;
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
        .add_startup_stage("game_setup", SystemStage::single(spawn_player.system()))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.5))
                .with_system(spawn_brick.system()),
        )
        .add_system(player_movement_input.system())
        .add_system(player_movement.system())
        .add_system(brick_movement.system())
        .add_system(player_brick_collision.system())
        .add_plugins(DefaultPlugins)
        .run();
}
