import re

with open("src/ant.rs", "r") as f:
    content = f.read()

# Make the changes more reliably using precise string replacement
old_periodic = """fn periodic_direction_update(
    mut ant_query: Query<(&mut Acceleration, &Transform, &CurrentTask, &Velocity), With<Ant>>,
    mut pheromones: ResMut<Pheromones>,
    mut stats: ResMut<SimStatistics>,
    scan_radius: Res<AntScanRadius>,
) {
    (stats.food_cache_size, stats.home_cache_size) = pheromones.clear_cache();

    for (mut acceleration, transform, current_task, velocity) in ant_query.iter_mut() {"""

new_periodic = """fn periodic_direction_update(
    mut ant_query: Query<(&mut Acceleration, &Transform, &CurrentTask, &Velocity), With<Ant>>,
    mut pheromones: ResMut<Pheromones>,
    mut stats: ResMut<SimStatistics>,
    scan_radius: Res<AntScanRadius>,
) {
    (stats.food_cache_size, stats.home_cache_size) = pheromones.clear_cache();

    // BOLT OPTIMIZATION: Hoist constants out of the ant loop
    let home_pos = vec3(HOME_LOCATION.0, HOME_LOCATION.1, 0.0);
    let food_pos = vec3(FOOD_LOCATION.0, FOOD_LOCATION.1, 0.0);
    let pull_radius_sq = ANT_TARGET_AUTO_PULL_RADIUS * ANT_TARGET_AUTO_PULL_RADIUS;
    let home_target = Some(vec2(HOME_LOCATION.0, HOME_LOCATION.1));
    let food_target = Some(vec2(FOOD_LOCATION.0, FOOD_LOCATION.1));
    let mut rng = rand::thread_rng();

    for (mut acceleration, transform, current_task, velocity) in ant_query.iter_mut() {"""

if old_periodic in content:
    content = content.replace(old_periodic, new_periodic)

old_periodic_body1 = """                let dist_to_food = transform.translation.distance_squared(vec3(
                    FOOD_LOCATION.0,
                    FOOD_LOCATION.1,
                    0.0,
                ));
                if dist_to_food <= ANT_TARGET_AUTO_PULL_RADIUS * ANT_TARGET_AUTO_PULL_RADIUS {
                    target = Some(vec2(FOOD_LOCATION.0, FOOD_LOCATION.1));
                }"""
new_periodic_body1 = """                let dist_to_food = transform.translation.distance_squared(food_pos);
                if dist_to_food <= pull_radius_sq {
                    target = food_target;
                }"""
if old_periodic_body1 in content:
    content = content.replace(old_periodic_body1, new_periodic_body1)

old_periodic_body2 = """                let dist_to_home = transform.translation.distance_squared(vec3(
                    HOME_LOCATION.0,
                    HOME_LOCATION.1,
                    0.0,
                ));
                if dist_to_home <= ANT_TARGET_AUTO_PULL_RADIUS * ANT_TARGET_AUTO_PULL_RADIUS {
                    target = Some(vec2(HOME_LOCATION.0, HOME_LOCATION.1));
                }"""
new_periodic_body2 = """                let dist_to_home = transform.translation.distance_squared(home_pos);
                if dist_to_home <= pull_radius_sq {
                    target = home_target;
                }"""
if old_periodic_body2 in content:
    content = content.replace(old_periodic_body2, new_periodic_body2)

old_periodic_body3 = """        let mut rng = rand::thread_rng();
        acceleration.0 += steering_force * rng.gen_range(0.4..=ANT_STEERING_FORCE_FACTOR);"""
new_periodic_body3 = """        acceleration.0 += steering_force * rng.gen_range(0.4..=ANT_STEERING_FORCE_FACTOR);"""
if old_periodic_body3 in content:
    content = content.replace(old_periodic_body3, new_periodic_body3)

old_check = """fn check_home_food_collisions(
    mut ant_query: Query<
        (
            &Transform,
            &mut Sprite,
            &mut Velocity,
            &mut CurrentTask,
            &mut PhStrength,
            &mut Handle<Image>,
        ),
        With<Ant>,
    >,
    asset_server: Res<AssetServer>,
) {
    for (transform, mut sprite, mut velocity, mut ant_task, mut ph_strength, mut image_handle) in
        ant_query.iter_mut()
    {"""
new_check = """fn check_home_food_collisions(
    mut ant_query: Query<
        (
            &Transform,
            &mut Sprite,
            &mut Velocity,
            &mut CurrentTask,
            &mut PhStrength,
            &mut Handle<Image>,
        ),
        With<Ant>,
    >,
    asset_server: Res<AssetServer>,
) {
    // BOLT OPTIMIZATION: Hoist constants out of the ant loop
    let home_pos = vec3(HOME_LOCATION.0, HOME_LOCATION.1, 0.0);
    let food_pos = vec3(FOOD_LOCATION.0, FOOD_LOCATION.1, 0.0);
    let home_radius_sq = HOME_RADIUS * HOME_RADIUS;
    let food_pickup_radius_sq = FOOD_PICKUP_RADIUS * FOOD_PICKUP_RADIUS;

    for (transform, mut sprite, mut velocity, mut ant_task, mut ph_strength, mut image_handle) in
        ant_query.iter_mut()
    {"""
if old_check in content:
    content = content.replace(old_check, new_check)

old_check_body1 = """        // Home collision
        let dist_to_home =
            transform
                .translation
                .distance_squared(vec3(HOME_LOCATION.0, HOME_LOCATION.1, 0.0));
        if dist_to_home < HOME_RADIUS * HOME_RADIUS {"""
new_check_body1 = """        // Home collision
        let dist_to_home = transform.translation.distance_squared(home_pos);
        if dist_to_home < home_radius_sq {"""
if old_check_body1 in content:
    content = content.replace(old_check_body1, new_check_body1)

old_check_body2 = """        // Food Collision
        let dist_to_food =
            transform
                .translation
                .distance_squared(vec3(FOOD_LOCATION.0, FOOD_LOCATION.1, 0.0));
        if dist_to_food < FOOD_PICKUP_RADIUS * FOOD_PICKUP_RADIUS {"""
new_check_body2 = """        // Food Collision
        let dist_to_food = transform.translation.distance_squared(food_pos);
        if dist_to_food < food_pickup_radius_sq {"""
if old_check_body2 in content:
    content = content.replace(old_check_body2, new_check_body2)

old_wall = """fn check_wall_collision(
    mut ant_query: Query<(&Transform, &Velocity, &mut Acceleration), With<Ant>>,
) {
    for (transform, velocity, mut acceleration) in ant_query.iter_mut() {
        // wall rebound
        let border = 20.0;
        let top_left = (-W / 2.0, H / 2.0);
        let bottom_right = (W / 2.0, -H / 2.0);"""
new_wall = """fn check_wall_collision(
    mut ant_query: Query<(&Transform, &Velocity, &mut Acceleration), With<Ant>>,
) {
    // BOLT OPTIMIZATION: Hoist constants and thread_rng out of the ant loop
    let border = 20.0;
    let top_left = (-W / 2.0, H / 2.0);
    let bottom_right = (W / 2.0, -H / 2.0);
    let mut rng = thread_rng();

    for (transform, velocity, mut acceleration) in ant_query.iter_mut() {
        // wall rebound"""
if old_wall in content:
    content = content.replace(old_wall, new_wall)

old_wall_body = """        if x_bound || y_bound {
            let mut rng = thread_rng();
            let target = vec2(rng.gen_range(-200.0..200.0), rng.gen_range(-200.0..200.0));
            acceleration.0 +=
                get_steering_force(target, transform.translation.truncate(), velocity.0);
        }"""
new_wall_body = """        if x_bound || y_bound {
            let target = vec2(rng.gen_range(-200.0..200.0), rng.gen_range(-200.0..200.0));
            acceleration.0 +=
                get_steering_force(target, transform.translation.truncate(), velocity.0);
        }"""
if old_wall_body in content:
    content = content.replace(old_wall_body, new_wall_body)

with open("src/ant.rs", "w") as f:
    f.write(content)

print("done patching2")
