use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)] 
pub enum Orientation { 
    Up, 
    Down, 
    Left,
    Right,
}

#[derive(Component)]
pub struct Player
{
    pub target: Vec3, 
    pub speed: f32,
    pub orientation: Orientation,
    pub prev_orientation: Orientation,
    pub frame_timer: Timer,
    pub is_first_frame: bool,
}