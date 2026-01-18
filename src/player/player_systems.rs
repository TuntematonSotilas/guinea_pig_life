use bevy::prelude::*;

use crate::player::player_types::Player;

pub fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player { target: Vec3::new(0., 0., 0.), speed: 200.},
        Sprite::from_image(
            asset_server.load("player.png"),
        )
    ));
}

pub fn move_player(time: Res<Time>, 
    mut player: Single<(&mut Transform, &Player)>,) {
        
    let direction = player.1.target - player.0.translation; 
    let distance = direction.length(); 
    if distance > 1.0 { 
        let step = player.1.speed * time.delta_secs(); 
        if step >= distance { 
            // the player have reached the target 
            player.0.translation = player.1.target; 
        } else { 
            // move the target
            let movement = direction.normalize() * step; 
            player.0.translation += movement;
         }
    }
}

pub fn mouse_click_system(buttons: Res<ButtonInput<MouseButton>>, 
    camera_q: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut player: Single<&mut Player>) {

    // Left button was pressed  
    if buttons.just_pressed(MouseButton::Left) {
        // Calculate a world position based on the cursor's position
        let (camera, camera_transform) = *camera_q;
        if let Some(cursor_position) = window.cursor_position()
            && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                player.target = Vec3::new(world_pos.x, world_pos.y, 0.0);
        }
    }
}