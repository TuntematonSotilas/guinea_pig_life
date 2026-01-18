use bevy::prelude::*;

#[derive(Component)]
pub struct Player
{
    pub target: Vec3, 
    pub speed: f32,
}