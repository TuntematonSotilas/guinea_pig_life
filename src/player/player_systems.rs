use bevy::prelude::*;

use crate::player::player_types::Player;

pub fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Player { dest: Vec2::new(0., 0.)},
        Sprite::from_image(
        asset_server.load("player.png"),
    )));
}

pub fn move_player(time: Res<Time>, 
    mut player_pos: Single<&mut Transform, With<Player>>,
    player: Single<&Player>) {
    
    let mut new_pos = Vec3::new(player_pos.translation.x, player_pos.translation.y, 0.);
    
    if player.dest.x - 4. < player_pos.translation.x 
        || player_pos.translation.x < player.dest.x + 4. 
        || player.dest.y - 4. < player_pos.translation.y 
        || player_pos.translation.y < player.dest.y + 4. {

        println!("player_pos {:?}", player_pos.translation);

        if player_pos.translation.x < player.dest.x {
            new_pos.x += 150. * time.delta_secs();
        } else if player_pos.translation.x > player.dest.x {
            new_pos.x -= 150. * time.delta_secs();
        }

        if player_pos.translation.y < player.dest.y {
            new_pos.y += 150. * time.delta_secs();
        } else if player_pos.translation.y > player.dest.y {
            new_pos.y -= 150. * time.delta_secs();
        }
        
    }

    // println!("player_pos {:?}", player_pos.translation);
    // println!("dest {:?}", player.dest);
    // println!("new_pos {:?}", new_pos);

    player_pos.translation = new_pos;
    

}

pub fn mouse_click_system(buttons: Res<ButtonInput<MouseButton>>, 
    camera_q: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut player: Single<&mut Player>) {
        
        // Left button was released  
        if buttons.just_released(MouseButton::Left) {
            // Calculate a world position based on the cursor's position
            let (camera, camera_transform) = *camera_q;
            if let Some(cursor_position) = window.cursor_position()
                && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                    println!("world_pos {:?}", world_pos);
                    player.dest = world_pos;
            }
        }
}