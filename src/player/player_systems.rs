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

pub fn mouse_click_system(
    buttons: Res<ButtonInput<MouseButton>>, 
    touches: Res<Touches>,
    camera_q: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut player: Single<&mut Player>) {

    let mut click_pos = None;
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(win_pos) = window.cursor_position() {
            click_pos = Some(win_pos);
        }
    } else {
        for finger in touches.iter() {
                if touches.just_pressed(finger.id()) {
                    click_pos = Some(finger.position());
                }
            }
    }

    // Left button was pressed  
    if let Some(click_pos) = click_pos {
        // Calculate a world position based on the cursor's position
        let (camera, camera_transform) = *camera_q;
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, click_pos) {
                player.target = Vec3::new(world_pos.x, world_pos.y, 0.0);
        }
    }
}