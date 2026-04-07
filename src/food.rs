use crate::{ant::Food, *};
use bevy::prelude::*;

pub struct FoodPlugin;

#[derive(Component)]
pub struct FoodLabel;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, manual_food_spawn_system);
    }
}

fn manual_food_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if (keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight))
        && mouse_button_input.just_pressed(MouseButton::Left)
    {
        if let (Ok(window), Ok((camera, camera_transform))) =
            (windows.get_single(), camera_query.get_single())
        {
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
            {
                spawn_food_entity(
                    &mut commands,
                    &asset_server,
                    world_position.x,
                    world_position.y,
                );
            }
        }
    }
}

fn spawn_food_entity(commands: &mut Commands, asset_server: &Res<AssetServer>, x: f32, y: f32) {
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load(SPRITE_FOOD),
                sprite: Sprite {
                    color: Color::rgb(1.5, 1.5, 1.5),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 2.0)
                    .with_scale(Vec3::splat(FOOD_SPRITE_SCALE)),
                ..Default::default()
            },
            Food {
                units: FOOD_UNITS_PER_ENTITY,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        FOOD_UNITS_PER_ENTITY.to_string(),
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 20.0, 1.0),
                    ..default()
                },
                FoodLabel,
            ));
        });
}
