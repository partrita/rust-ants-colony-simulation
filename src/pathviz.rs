use crate::{
    ant::{Ant, AntTask, CurrentTask},
    grid::{add_map_to_grid_img, DecayGrid},
    gui::SimSettings,
    utils::window_to_grid,
    *,
};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    time::common_conditions::on_timer,
};
use std::{collections::HashMap, time::Duration};

pub struct PathVizPlugin;

#[derive(Resource)]
pub struct PathVizGrid {
    pub dg_path: DecayGrid,
}

#[derive(Component)]
struct PathVizImageRender;

impl Plugin for PathVizPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .insert_resource(PathVizGrid::new())
            .add_systems(Update, update_grid_values)
            .add_systems(
                Update,
                update_viz_grid_visibility.run_if(on_timer(Duration::from_secs_f32(1.0))),
            )
            .add_systems(
                Update,
                update_path_viz_image.run_if(on_timer(Duration::from_secs_f32(0.1))),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0)
                .with_scale(Vec3::splat(PH_UNIT_GRID_SIZE as f32)),
            ..Default::default()
        },
        PathVizImageRender,
    ));
}

fn update_viz_grid_visibility(
    sim_settings: Res<SimSettings>,
    mut query: Query<&mut Visibility, With<PathVizImageRender>>,
) {
    let mut img_visibility = query.single_mut();
    if sim_settings.is_show_ants_path {
        *img_visibility = Visibility::Visible;
    } else {
        *img_visibility = Visibility::Hidden;
    }
}

fn update_grid_values(
    ant_query: Query<(&Transform, &CurrentTask), With<Ant>>,
    mut viz_grid: ResMut<PathVizGrid>,
) {
    for (transform, current_task) in ant_query.iter() {
        let x = transform.translation.x as i32;
        let y = transform.translation.y as i32;
        let key = window_to_grid(x, y);

        // 집으로 돌아가는 개미(먹이를 찾은 개미)의 경로만 더 진하게 표시
        if let AntTask::FindHome = current_task.0 {
            viz_grid
                .dg_path
                .add_value(&key, VIZ_COLOR_STRENGTH * 2.0, 5.0);
        } else {
            viz_grid.dg_path.add_value(&key, VIZ_COLOR_STRENGTH, 5.0);
        }
    }

    viz_grid.dg_path.decay_values(VIZ_DECAY_RATE);
    viz_grid.dg_path.drop_zero_values();
}

fn update_path_viz_image(
    mut textures: ResMut<Assets<Image>>,
    viz_grid: Res<PathVizGrid>,
    mut query: Query<&mut Handle<Image>, With<PathVizImageRender>>,
) {
    let mut img_handle = query.single_mut();
    let (w, h) = (
        W as usize / PH_UNIT_GRID_SIZE,
        H as usize / PH_UNIT_GRID_SIZE,
    );

    let mut bytes = vec![0; w * h * 4];
    add_map_to_grid_img(
        viz_grid.dg_path.get_values(),
        &VIZ_COLOR_TO_FOOD, // 한 가지 색상으로 통합
        &mut bytes,
        false,
    );

    let path_img = Image::new(
        Extent3d {
            width: w as u32,
            height: h as u32,
            ..Default::default()
        },
        TextureDimension::D2,
        bytes,
        TextureFormat::Rgba8Unorm,
    );
    *img_handle = textures.add(path_img);
}

impl PathVizGrid {
    fn new() -> Self {
        Self {
            dg_path: DecayGrid::new(HashMap::new(), VIZ_MAX_COLOR_STRENGTH),
        }
    }
}
