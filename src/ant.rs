use crate::{
    gui::{SimSettings, SimStatistics},
    pheromone::Pheromones,
    utils::get_rand_unit_vec2,
    *,
};
use bevy::{
    math::{vec2, vec3},
    prelude::*,
    time::common_conditions::on_timer,
};
use rand::{thread_rng, Rng};
use std::{collections::HashSet, f32::consts::PI, time::Duration};

pub struct AntPlugin;

#[derive(PartialEq, Clone, Copy)]
pub enum AntTask {
    FindFood,
    PickingUp(f32),
    FindHome,
}

#[derive(Component)]
pub struct Ant {
    pub path_history: Vec<Vec2>,
    pub direction_timer: f32, // 개별 방향 업데이트 타이머
}

#[derive(Component)]
pub struct CurrentTask(pub AntTask);
#[derive(Component)]
struct Velocity(Vec2);
#[derive(Component)]
pub struct Acceleration(pub Vec2);
#[derive(Component)]
struct PhStrength(f32);
#[derive(Component)]
struct SearchTimer(f32);

#[derive(Component)]
pub struct Food {
    pub units: u32,
}

#[derive(Resource)]
pub struct AntScanRadius(f32);
#[derive(Resource)]
pub struct AntFollowCameraPos(pub Vec2);

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .insert_resource(AntScanRadius(INITIAL_ANT_PH_SCAN_RADIUS))
            .insert_resource(AntFollowCameraPos(Vec2::ZERO))
            .add_systems(
                Update,
                drop_pheromone.run_if(on_timer(Duration::from_secs_f32(ANT_PH_DROP_INTERVAL))),
            )
            .add_systems(
                Update,
                check_wall_collision.run_if(on_timer(Duration::from_secs_f32(0.1))),
            )
            .add_systems(
                Update,
                check_home_food_collisions,
            )
            .add_systems(Update, update_camera_follow_pos)
            .add_systems(
                Update,
                (
                    periodic_direction_update, // on_timer 제거 (매 프레임 체크)
                    record_path_system.run_if(on_timer(Duration::from_secs_f32(0.5))),
                    follow_path_home_system,
                    update_pickup_timer,
                ),
            )
            .add_systems(
                Update,
                update_stats.run_if(on_timer(Duration::from_secs_f32(1.0))),
            )
            .add_systems(
                Update,
                update_scan_radius.run_if(on_timer(Duration::from_secs_f32(1.0))),
            )
            .add_systems(
                Update,
                decay_ph_strength.run_if(on_timer(Duration::from_secs_f32(
                    ANT_PH_STRENGTH_DECAY_INTERVAL,
                ))),
            )
            .add_systems(Update, update_search_timer)
            .add_systems(Update, update_position.after(check_wall_collision));
    }
}

pub fn spawn_ant(commands: &mut Commands, asset_server: &Res<AssetServer>, pos: Vec2) {
    let mut rng = thread_rng();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(SPRITE_ANT),
            sprite: Sprite {
                color: Color::rgb(1.1, 1.1, 1.0),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, ANT_Z_INDEX)
                .with_scale(Vec3::splat(ANT_SPRITE_SCALE)),
            ..Default::default()
        },
        Ant {
            path_history: Vec::new(),
            // 생성 시 타이머를 랜덤하게 초기화하여 업데이트 시점을 분산시킴
            direction_timer: rng.gen_range(0.0..ANT_DIRECTION_UPDATE_INTERVAL),
        },
        CurrentTask(AntTask::FindFood),
        Velocity(get_rand_unit_vec2()),
        Acceleration(Vec2::ZERO),
        PhStrength(0.0),
        SearchTimer(0.0),
    ));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let home_pos = vec2(HOME_LOCATION.0, HOME_LOCATION.1);
    for _ in 0..NUM_ANTS {
        spawn_ant(&mut commands, &asset_server, home_pos);
    }
}

fn update_pickup_timer(
    time: Res<Time>,
    mut ant_query: Query<(&mut CurrentTask, &mut PhStrength, &mut Handle<Image>, &mut Sprite, &mut Velocity), With<Ant>>,
    asset_server: Res<AssetServer>,
) {
    let ant_with_food_handle = asset_server.load(SPRITE_ANT_WITH_FOOD);
    for (mut task, mut ph_strength, mut image_handle, mut sprite, mut velocity) in ant_query.iter_mut() {
        if let AntTask::PickingUp(ref mut remaining) = task.0 {
            *remaining -= time.delta_seconds();
            if *remaining <= 0.0 {
                task.0 = AntTask::FindHome;
                ph_strength.0 = ANT_INITIAL_PH_STRENGTH;
                *image_handle = ant_with_food_handle.clone();
                sprite.color = Color::rgb(1.0, 2.0, 1.0);
                velocity.0 = get_rand_unit_vec2();
            }
        }
    }
}

fn record_path_system(mut ant_query: Query<(&mut Ant, &Transform, &CurrentTask)>) {
    for (mut ant, transform, task) in ant_query.iter_mut() {
        if let AntTask::FindFood = task.0 {
            let pos = transform.translation.truncate();
            
            let mut loop_index = None;
            for (i, &past_pos) in ant.path_history.iter().enumerate() {
                if past_pos.distance_squared(pos) < 900.0 {
                    loop_index = Some(i);
                    break;
                }
            }

            if let Some(index) = loop_index {
                ant.path_history.truncate(index + 1);
            } else {
                if ant.path_history.is_empty() || ant.path_history.last().unwrap().distance_squared(pos) > 400.0 {
                    ant.path_history.push(pos);
                }
            }
        }
    }
}

fn follow_path_home_system(
    mut ant_query: Query<(&mut Ant, &mut Acceleration, &Transform, &CurrentTask, &Velocity)>,
) {
    for (mut ant, mut acceleration, transform, task, velocity) in ant_query.iter_mut() {
        if let AntTask::FindHome = task.0 {
            let current_pos = transform.translation.truncate();
            
            let target = if let Some(&last_pos) = ant.path_history.last() {
                if current_pos.distance_squared(last_pos) < 225.0 {
                    ant.path_history.pop();
                    if let Some(&next_pos) = ant.path_history.last() {
                        next_pos
                    } else {
                        vec2(HOME_LOCATION.0, HOME_LOCATION.1)
                    }
                } else {
                    last_pos
                }
            } else {
                vec2(HOME_LOCATION.0, HOME_LOCATION.1)
            };

            let steering_force = get_steering_force(target, current_pos, velocity.0);
            acceleration.0 += steering_force * 3.0;
        }
    }
}

fn update_search_timer(
    time: Res<Time>,
    mut ant_query: Query<(&mut SearchTimer, &mut CurrentTask, &mut PhStrength), With<Ant>>,
) {
    for (mut timer, mut task, mut ph_strength) in ant_query.iter_mut() {
        if let AntTask::FindFood = task.0 {
            timer.0 += time.delta_seconds();
            if timer.0 >= ANT_MAX_SEARCH_TIME {
                task.0 = AntTask::FindHome;
                ph_strength.0 = 0.0;
                timer.0 = 0.0;
            }
        } else if let AntTask::FindHome = task.0 {
            timer.0 = 0.0;
        }
    }
}

fn drop_pheromone(
    mut ant_query: Query<(&Transform, &CurrentTask, &PhStrength), With<Ant>>,
    mut pheromones: ResMut<Pheromones>,
) {
    for (transform, ant_task, ph_strength) in ant_query.iter_mut() {
        if let AntTask::FindHome = ant_task.0 {
            if ph_strength.0 > 0.1 {
                let x = transform.translation.x as i32;
                let y = transform.translation.y as i32;
                pheromones.to_food.emit_signal(&(x, y), ph_strength.0);
            }
        }
    }
}

fn update_scan_radius(mut scan_radius: ResMut<AntScanRadius>) {
    if scan_radius.0 > INITIAL_ANT_PH_SCAN_RADIUS * ANT_PH_SCAN_RADIUS_SCALE {
        return;
    }
    scan_radius.0 += ANT_PH_SCAN_RADIUS_INCREMENT;
}

fn update_camera_follow_pos(
    ant_query: Query<&Transform, With<Ant>>,
    mut follow_pos: ResMut<AntFollowCameraPos>,
) {
    if let Some(transform) = ant_query.iter().next() {
        follow_pos.0 = transform.translation.truncate();
    }
}

fn update_stats(
    mut stats: ResMut<SimStatistics>,
    scan_radius: Res<AntScanRadius>,
    ant_query: Query<With<Ant>>,
) {
    stats.scan_radius = scan_radius.0;
    stats.num_ants = ant_query.iter().len();
}

fn decay_ph_strength(mut ant_query: Query<&mut PhStrength, With<Ant>>) {
    for mut ph_strength in ant_query.iter_mut() {
        ph_strength.0 = f32::max(ph_strength.0 - ANT_PH_STRENGTH_DECAY_RATE, 0.0);
    }
}

fn get_steering_force(target: Vec2, current: Vec2, velocity: Vec2) -> Vec2 {
    let desired = (target - current).normalize() * ANT_SPEED;
    let steering = desired - velocity;
    steering * 0.8
}

fn periodic_direction_update(
    time: Res<Time>,
    mut ant_query: Query<(&mut Ant, &mut Acceleration, &Transform, &CurrentTask, &Velocity), Without<Food>>,
    food_query: Query<&Transform, (With<Food>, Without<Ant>)>,
    mut pheromones: ResMut<Pheromones>,
    mut stats: ResMut<SimStatistics>,
    sim_settings: Res<SimSettings>,
    scan_radius: Res<AntScanRadius>,
) {
    (stats.food_cache_size, _) = pheromones.clear_cache();
    let pull_radius_sq = ANT_TARGET_AUTO_PULL_RADIUS * ANT_TARGET_AUTO_PULL_RADIUS;
    let mut rng = thread_rng();

    for (mut ant, mut acceleration, transform, current_task, velocity) in ant_query.iter_mut() {
        if let AntTask::FindHome = current_task.0 {
            continue;
        }

        // 각 개미만의 타이머를 깎음
        ant.direction_timer -= time.delta_seconds();
        if ant.direction_timer > 0.0 {
            continue; // 자신의 시간이 안 되었으면 통과
        }
        // 타이머 리셋 (0.5초 근처에서 약간의 무작위성을 주어 더 분산시킴)
        ant.direction_timer = ANT_DIRECTION_UPDATE_INTERVAL + rng.gen_range(-0.1..0.1);

        let current_pos = transform.translation.truncate();
        let mut target = None;

        let mut min_dist_sq = pull_radius_sq;
        for food_transform in food_query.iter() {
            let dist_sq = current_pos.distance_squared(food_transform.translation.truncate());
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                target = Some(food_transform.translation.truncate());
            }
        }

        let follow_ph_chance = 1.0 - (sim_settings.scout_ratio / 100.0);
        if target.is_none() && rng.gen_bool(follow_ph_chance as f64) {
            target = pheromones.to_food.get_steer_target_filtered(&transform.translation, scan_radius.0, velocity.0);
        }

        if let Some(target_vec) = target {
            let steering_force = get_steering_force(target_vec, current_pos, velocity.0);
            acceleration.0 += steering_force * rng.gen_range(0.8..=1.5);
        } else {
            acceleration.0 += get_rand_unit_vec2() * 0.3;
        }
    }
}

fn check_home_food_collisions(
    mut commands: Commands,
    mut ant_query: Query<
        (
            &mut Ant,
            &mut Transform,
            &mut Sprite,
            &mut Velocity,
            &mut Acceleration,
            &mut CurrentTask,
            &mut PhStrength,
            &mut Handle<Image>,
        ),
        (With<Ant>, Without<Food>),
    >,
    mut food_query: Query<(Entity, &mut Transform, &mut Food, Option<&Children>), (With<Food>, Without<Ant>)>,
    mut label_query: Query<&mut Text, With<crate::food::FoodLabel>>,
    asset_server: Res<AssetServer>,
    sim_settings: Res<SimSettings>,
    mut consumed_this_frame: Local<HashSet<Entity>>,
) {
    consumed_this_frame.clear();
    let current_ant_count = ant_query.iter().count();
    let home_pos = vec3(HOME_LOCATION.0, HOME_LOCATION.1, 0.0);
    let home_radius_sq = HOME_RADIUS * HOME_RADIUS;
    let food_pickup_radius_sq = FOOD_PICKUP_RADIUS * FOOD_PICKUP_RADIUS;
    let ant_handle = asset_server.load(SPRITE_ANT);
    let mut rng = thread_rng();

    for (mut ant, mut transform, mut sprite, mut velocity, mut acceleration, mut ant_task, mut ph_strength, mut image_handle) in
        ant_query.iter_mut()
    {
        let dist_to_home = transform.translation.distance_squared(home_pos);
        if dist_to_home < home_radius_sq {
            if let AntTask::FindHome = ant_task.0 {
                if current_ant_count < sim_settings.max_ants as usize {
                    spawn_ant(&mut commands, &asset_server, home_pos.truncate());
                }
                
                ant_task.0 = AntTask::FindFood;
                ph_strength.0 = 0.0;
                ant.path_history.clear();
                *image_handle = ant_handle.clone();
                sprite.color = Color::rgb(1.0, 1.0, 2.5);
                velocity.0 *= -1.0;
                transform.rotation = Quat::from_rotation_z(velocity.0.y.atan2(velocity.0.x) + PI / 2.0);
            }
        }

        if let AntTask::FindFood = ant_task.0 {
            let mut target_food_entity = None;
            let mut nearest_food_dist_sq = f32::MAX;
            for (food_entity, food_transform, _food, _children) in food_query.iter() {
                if consumed_this_frame.contains(&food_entity) {
                    continue;
                }
                let dist_sq = transform.translation.distance_squared(food_transform.translation);
                if dist_sq < nearest_food_dist_sq {
                    nearest_food_dist_sq = dist_sq;
                    if dist_sq < food_pickup_radius_sq {
                        target_food_entity = Some(food_entity);
                    }
                }
            }

            if let Some(food_entity) = target_food_entity {
                if let Ok((_, mut food_transform, mut food, children)) = food_query.get_mut(food_entity) {
                    food.units -= 1;
                    if let Some(children) = children {
                        for &child in children.iter() {
                            if let Ok(mut text) = label_query.get_mut(child) {
                                text.sections[0].value = food.units.to_string();
                            }
                        }
                    }
                    let scale_factor = 0.5 + (food.units as f32 / FOOD_UNITS_PER_ENTITY as f32) * 0.5;
                    food_transform.scale = Vec3::splat(FOOD_SPRITE_SCALE * scale_factor);
                    if food.units == 0 {
                        consumed_this_frame.insert(food_entity);
                        commands.entity(food_entity).despawn_recursive();
                    }
                    
                    let delay = rng.gen_range(0.1..=2.0);
                    ant_task.0 = AntTask::PickingUp(delay);
                    velocity.0 = Vec2::ZERO;
                    acceleration.0 = Vec2::ZERO;
                }
            }
        }
    }
}

fn check_wall_collision(
    mut ant_query: Query<(&Transform, &Velocity, &mut Acceleration, &mut CurrentTask, &mut PhStrength), With<Ant>>,
) {
    let width_half = W / 2.0;
    let height_half = H / 2.0;
    let margin = 100.0;
    let home_pos = vec2(HOME_LOCATION.0, HOME_LOCATION.1);
    let mut rng = thread_rng();

    for (transform, velocity, mut acceleration, mut task, mut ph_strength) in ant_query.iter_mut() {
        let pos = transform.translation.truncate();
        if pos.x.abs() > width_half - margin || pos.y.abs() > height_half - margin {
            let mut target = vec2(rng.gen_range(-200.0..200.0), rng.gen_range(-200.0..200.0));
            if let AntTask::FindHome = task.0 {
                target = home_pos;
            }
            let desired = (target - pos).normalize();
            let steering = (desired - velocity.0) * 1.5;
            acceleration.0 += steering;
            if let AntTask::FindFood = task.0 {
                if pos.x.abs() > width_half - 20.0 || pos.y.abs() > height_half - 20.0 {
                    task.0 = AntTask::FindHome;
                    ph_strength.0 = 0.0;
                }
            }
        }
    }
}

fn update_position(
    mut ant_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), With<Ant>>,
) {
    for (mut transform, mut velocity, mut acceleration) in ant_query.iter_mut() {
        if acceleration.0 != Vec2::ZERO && !acceleration.0.is_nan() {
            velocity.0 = (velocity.0 + acceleration.0).normalize();
            acceleration.0 = Vec2::ZERO;
            transform.rotation = Quat::from_rotation_z(velocity.0.y.atan2(velocity.0.x) + PI / 2.0);
        }

        if velocity.0 == Vec2::ZERO { continue; }

        let new_translation = transform.translation + vec3(velocity.0.x, velocity.0.y, 0.0) * ANT_SPEED;
        if !new_translation.is_nan() {
            transform.translation = new_translation;
        }
    }
}
