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
            prev_orientation: Orientation::Right,
            frame_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            is_first_frame: true,
        },
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            ..default()
        }
    ));
}

pub fn move_player(time: Res<Time>, player_q: Single<(&mut Transform, &mut Sprite, &mut Player)>) {
    let (mut transform, mut sprite, mut player) = player_q.into_inner();
    let direction = player.target - transform.translation; 
    let distance = direction.length(); 
    if distance > 1.0 { 
        let step =  player.speed * time.delta_secs(); 
        if step >= distance { 
            // the player have reached the target 
            transform.translation = player.target; 
        } else { 
            player.frame_timer.tick(time.delta());

            if player.frame_timer.just_finished() {
                let mut index = sprite.texture_atlas.as_ref().unwrap().index;
                if player.is_first_frame {
                    index += 1;
                } else {
                    index -= 1;
                }

                sprite.texture_atlas.as_mut().unwrap().index = index;

                player.is_first_frame = !player.is_first_frame;
            }
            // move the target
            let movement = direction.normalize() * step; 
            transform.translation += movement;
         }
    }
}

pub fn set_player_sprite(player_q: Single<(&mut Sprite, &mut Player)>,) {
    let (mut sprite, mut player) = player_q.into_inner();
    if player.orientation != player.prev_orientation {
        let index = match player.orientation {
            Orientation::Up => 0,
            Orientation::Down => 2,
            Orientation::Left => 6,
            Orientation::Right => 4,
        };
        sprite.texture_atlas.as_mut().unwrap().index = index;
        player.prev_orientation = player.orientation;
        player.is_first_frame = true;
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