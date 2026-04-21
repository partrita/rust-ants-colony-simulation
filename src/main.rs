// 🛡️ Security Enhancement: Prevent memory safety vulnerabilities by forbidding unsafe code.
#![forbid(unsafe_code)]
// 🛡️ Security Enhancement: Prevent DoS by forbidding panics.
#![forbid(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]
use ants::{
    ant::{Ant, AntFollowCameraPos, AntPlugin, Food},
    food::FoodPlugin,
    gui::{GuiPlugin, SimSettings},
    pathviz::{PathVizGrid, PathVizPlugin},
    pheromone::{PheromonePlugin, Pheromones},
    ResetSimulationEvent, *,
};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::vec3,
    prelude::*,
};
use bevy_pancam::{PanCam, PanCamPlugin};

#[derive(Component)]
struct FollowCamera;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (W, H).into(),
                        title: "Ants".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_event::<ResetSimulationEvent>()
        // External plugins & systems
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(PanCamPlugin)
        // Default Resources
        .insert_resource(ClearColor(Color::rgba_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
        )))
        .insert_resource(Msaa::Off)
        // Systems
        .add_systems(Startup, setup)
        .add_systems(Update, fix_window_size)
        .add_systems(Update, spawn_camera)
        .add_systems(Update, ant_follow_camera)
        .add_systems(Update, handle_reset_event)
        // Internal Plugins
        .add_plugins(AntPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(PheromonePlugin)
        .add_plugins(PathVizPlugin)
        .add_plugins(GuiPlugin)
        .run();
}

fn handle_reset_event(
    mut commands: Commands,
    mut reset_events: EventReader<ResetSimulationEvent>,
    ant_query: Query<Entity, With<Ant>>,
    food_query: Query<Entity, With<Food>>,
    mut pheromones: ResMut<Pheromones>,
    mut path_viz: ResMut<PathVizGrid>,
    asset_server: Res<AssetServer>,
) {
    for _ in reset_events.iter() {
        // 1. 모든 개미 제거
        for entity in ant_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        // 2. 모든 먹이 제거
        for entity in food_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        // 3. 페로몬 및 경로 시각화 초기화
        pheromones.reset();
        path_viz.dg_path.clear_values();

        // 4. 초기 개미(10마리) 다시 생성
        let home_pos = Vec2::new(HOME_LOCATION.0, HOME_LOCATION.1);
        for _ in 0..10 {
            ants::ant::spawn_ant(&mut commands, &asset_server, home_pos);
        }
    }
}

fn fix_window_size(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        let width = window.physical_width();
        let height = window.physical_height();
        if width > 16384 || height > 16384 {
            window
                .resolution
                .set_physical_resolution(W as u32, H as u32);
        }
    }
}

fn ant_follow_camera(
    ant_pos: Res<AntFollowCameraPos>,
    sim_settings: Res<SimSettings>,
    mut camera_query: Query<&mut Transform, With<FollowCamera>>,
) {
    if !sim_settings.is_camera_follow {
        return;
    }

    if let Ok(mut transform) = camera_query.get_single_mut() {
        transform.translation = vec3(ant_pos.0.x, ant_pos.0.y, ANT_Z_INDEX);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load(SPRITE_ANT_COLONY),
        sprite: Sprite {
            color: Color::rgb(1.5, 1.5, 1.5),
            ..default()
        },
        transform: Transform::from_xyz(HOME_LOCATION.0, HOME_LOCATION.1, 2.0)
            .with_scale(Vec3::splat(HOME_SPRITE_SCALE)),
        ..Default::default()
    });
}

fn spawn_camera(
    mut commands: Commands,
    windows: Query<&Window>,
    mut frame_count: Local<u32>,
    mut is_spawned: Local<bool>,
) {
    if *is_spawned {
        return;
    }
    if let Ok(window) = windows.get_single() {
        let width = window.physical_width();
        let height = window.physical_height();
        if width > 0 && width < 16384 && height > 0 && height < 16384 {
            info!(
                "Spawning camera now that window is ready (size: {}x{})",
                width, height
            );
            commands
                .spawn((
                    Camera2dBundle {
                        camera: Camera {
                            hdr: true,
                            ..default()
                        },
                        tonemapping: Tonemapping::TonyMcMapface,
                        ..default()
                    },
                    BloomSettings::default(),
                    FollowCamera,
                ))
                .insert(PanCam::default());
            *is_spawned = true;
        }
    }
    *frame_count += 1;
}
