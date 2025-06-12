use bevy::prelude::*;
// use std::f32::consts::PI;
use rand::Rng;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct Robot {
    pub name: String,
}

pub fn spawn_robot(mut commands: Commands) {
    commands.spawn((
        Robot {
            name: "hella-1".to_string(),
        },
        Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    ));
}

pub fn change_position(query: Query<&mut Position, With<Robot>>) {
    for mut pos in query {
        // println!("your position, [{}, {}]", pos.x, pos.y);
        pos.x = rand::random::<f32>();
        pos.y = rand::random::<f32>();
    }
}

pub fn print_positon(query: Query<&Position, With<Robot>>) {
    for pos in &query {
        println!("your position, [{}, {}]", pos.x, pos.y);
    }
}

pub struct SystemPlugin;
impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_robot);
        app.add_systems(Update, (change_position, print_positon).chain());
    }
}
