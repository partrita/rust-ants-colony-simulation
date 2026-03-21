use crate::{
    grid::{add_map_to_grid_img, WorldGrid},
    gui::{SimSettings, SimStatistics},
    *,
};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    time::common_conditions::on_timer,
};
use std::{collections::HashMap, time::Duration};

pub struct PheromonePlugin;

#[derive(Resource)]
pub struct Pheromones {
    pub to_food: WorldGrid,
}

#[derive(Component)]
struct PheromoneImageRender;

impl Plugin for PheromonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .insert_resource(Pheromones::new())
            .add_systems(
                Update,
                pheromone_decay.run_if(on_timer(Duration::from_secs_f32(PH_DECAY_INTERVAL))),
            )
            .add_systems(
                Update,
                update_kd_tree.run_if(on_timer(Duration::from_secs_f32(
                    PH_KD_TREE_UPDATE_INTERVAL,
                ))),
            )
            .add_systems(
                Update,
                update_sim_stats.run_if(on_timer(Duration::from_secs_f32(
                    PH_KD_TREE_UPDATE_INTERVAL,
                ))),
            )
            .add_systems(
                Update,
                clean_zero_signals.run_if(on_timer(Duration::from_secs_f32(2.0))),
            )
            .add_systems(
                Update,
                pheromone_image_update.run_if(on_timer(Duration::from_secs_f32(PH_IMG_UPDATE_SEC))),
            );
    }
}

fn pheromone_decay(mut pheromones: ResMut<Pheromones>, settings: Res<SimSettings>) {
    pheromones.to_food.decay_signals_custom(settings.ph_decay_rate);
}

fn update_sim_stats(pheromones: Res<Pheromones>, mut stats: ResMut<SimStatistics>) {
    stats.ph_food_size = pheromones.to_food.get_signals_size() as u32;
}

fn update_kd_tree(mut pheromones: ResMut<Pheromones>) {
    pheromones.to_food.update_tree();
}

fn clean_zero_signals(mut pheromones: ResMut<Pheromones>) {
    pheromones.to_food.drop_zero_signals();
}

fn pheromone_image_update(
    mut textures: ResMut<Assets<Image>>,
    sim_settings: Res<SimSettings>,
    pheromone: Res<Pheromones>,
    mut image_handle_query: Query<&mut Handle<Image>, With<PheromoneImageRender>>,
) {
    let mut img_handle = image_handle_query.single_mut();
    let (w, h) = (
        W as usize / PH_UNIT_GRID_SIZE,
        H as usize / PH_UNIT_GRID_SIZE,
    );
    let mut bytes = vec![0; w * h * 4];

    if sim_settings.is_show_food_ph {
        add_map_to_grid_img(
            pheromone.to_food.get_signals(),
            &pheromone.to_food.color,
            &mut bytes,
            true,
        );
    }

    let pheromone_map = Image::new(
        Extent3d {
            width: w as u32,
            height: h as u32,
            ..Default::default()
        },
        TextureDimension::D2,
        bytes,
        TextureFormat::Rgba8Unorm,
    );
    *img_handle = textures.add(pheromone_map);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::splat(PH_UNIT_GRID_SIZE as f32)),
            ..Default::default()
        },
        PheromoneImageRender,
    ));
}

impl Pheromones {
    fn new() -> Self {
        Self {
            to_food: WorldGrid::new(PH_COLOR_TO_FOOD, HashMap::new()),
        }
    }

    pub fn reset(&mut self) {
        self.to_food.clear_all_signals();
    }

    pub fn clear_cache(&mut self) -> (u32, u32) {
        (
            self.to_food.clear_steer_cache(),
            0
        )
    }
}
