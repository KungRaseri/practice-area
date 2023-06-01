use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::{AnimationIndices, AnimationTimer, Enemy, Player};
use crate::config::player_config;

pub fn initialize_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn initialize_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/tiny_heroes_base/Package/sprite_sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices: AnimationIndices = AnimationIndices { first: 0, last: 2 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        Player {},
    ));
}

pub fn initialize_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/tiny_heroes_base/Package/sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        3,
        1,
        None,
        Some(Vec2 {
            x: 16.0 * 33.,
            y: 16.0 * 28.,
        }),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 2 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        Enemy {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    player_transform.single_mut().translation += Vec3::new(
        (keyboard_input.pressed(KeyCode::D) as i32 - keyboard_input.pressed(KeyCode::A) as i32)
            as f32,
        (keyboard_input.pressed(KeyCode::W) as i32 - keyboard_input.pressed(KeyCode::S) as i32)
            as f32,
        0.,
    )
    .normalize_or_zero()
        * player_config::SPEED
        * time.delta_seconds();
}

pub fn enemy_movement(
    mut enemy_transform: Query<&mut Transform, With<Enemy>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player) = player_transform.get_single_mut() {}
}

pub fn handle_player_sprite_updates(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_sprite_query: Query<&mut TextureAtlasSprite, With<Player>>,
) {
    if let Ok(mut player_sprite) = player_sprite_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            player_sprite.flip_x = true;
        }

        if keyboard_input.pressed(KeyCode::D) {
            player_sprite.flip_x = false;
        }
    }
}

pub fn keep_entities_within_bounds(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_transform_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = player_config::SPRITE_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + player_config::SPRITE_SIZE;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}
