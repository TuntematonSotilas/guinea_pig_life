use bevy::{log, prelude::*};

use crate::player::player_types::*;

pub fn add_player(
    mut commands: Commands, 
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>) {

    let texture = asset_server.load("player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Player { 
            target: Vec3::new(0., 0., 0.), 
            speed: 200.,
            orientation: Orientation::Right,
            old_orientation: Orientation::Right,
        },
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 5,
            }),
            ..default()
        }
    ));
}

pub fn move_player(time: Res<Time>, mut player_q: Single<(&mut Transform, &Player)>,) {
    let direction = player_q.1.target - player_q.0.translation; 
    let distance = direction.length(); 
    if distance > 1.0 { 
        let step =  player_q.1.speed * time.delta_secs(); 
        if step >= distance { 
            // the player have reached the target 
            player_q.0.translation = player_q.1.target; 
        } else { 
            // move the target
            let movement = direction.normalize() * step; 
            player_q.0.translation += movement;
         }
    }
}

pub fn set_player_sprite(mut player_q: Single<(&mut Sprite, &mut Player)>,) {
    if player_q.1.orientation != player_q.1.old_orientation {
         let index = match player_q.1.orientation {
            Orientation::Up => 1,
            Orientation::Down => 3,
            Orientation::Left => 7,
            Orientation::Right => 4,
        };
        player_q.0.texture_atlas.as_mut().unwrap().index = index;
        player_q.1.old_orientation = player_q.1.orientation;
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
         
            // Set player orientation
            let target = Vec2::new(player.target.x, player.target.y);
            let direction = world_pos - target;

            log::info!("direction {:?}", direction);

            if direction.x.abs() > direction.y.abs() { 
                if direction.x > 0. { 
                    player.orientation = Orientation::Right; 
                } else { 
                    player.orientation = Orientation::Left; 
                }
            } else { 
                if direction.y > 0. { 
                    player.orientation = Orientation::Up; 
                } else { 
                    player.orientation = Orientation::Down; 
                }
                log::info!("Orientation set to {:?}", player.orientation);
            }

            // Set target position
            player.target = Vec3::new(world_pos.x, world_pos.y, 0.0);
        } 
    }
}