use crate::{ant::Ant, pheromone::Pheromones, *};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct GuiPlugin;

#[derive(Resource)]
pub struct SimSettings {
    pub is_show_food_ph: bool,
    pub is_show_ants: bool,
    pub is_camera_follow: bool,
    pub is_show_menu: bool,
    pub is_show_ants_path: bool,
    pub max_ants: u32,
    pub scout_ratio: f32,
    pub ph_decay_rate: f32,
}

#[derive(Default, Resource)]
pub struct SimStatistics {
    pub ph_food_size: u32,
    pub scan_radius: f32,
    pub num_ants: usize,
    pub food_cache_size: u32,
    pub elapsed_time: f32,
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SimSettings::default())
            .insert_resource(SimStatistics::default())
            .add_systems(Update, settings_dialog)
            .add_systems(Update, top_panel)
            .add_systems(Update, settings_toggle)
            .add_systems(Update, update_time)
            .add_plugins(EguiPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup() {}

fn update_time(time: Res<Time>, mut stats: ResMut<SimStatistics>) {
    stats.elapsed_time += time.delta_seconds();
}

fn top_panel(mut contexts: EguiContexts, stats: Res<SimStatistics>, windows: Query<&Window>) {
    if let Ok(window) = windows.get_single() {
        if window.physical_width() > 16384 {
            return;
        }
    }
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Ants: {}", stats.num_ants));
            ui.separator();
            ui.label(format!("Time: {:.1}s", stats.elapsed_time));
        });
    });
}

fn settings_toggle(
    mut settings: ResMut<SimSettings>,
    ant_query: Query<&mut Visibility, With<Ant>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        settings.is_show_menu = !settings.is_show_menu;
    }
    if keys.just_pressed(KeyCode::F) {
        settings.is_show_food_ph = !settings.is_show_food_ph;
    }
    if keys.just_pressed(KeyCode::P) {
        settings.is_show_ants_path = !settings.is_show_ants_path;
    }
    if keys.just_pressed(KeyCode::A) {
        settings.is_show_ants = !settings.is_show_ants;
        toggle_ant_visibility(ant_query, settings.is_show_ants);
    }
}

fn settings_dialog(
    mut contexts: EguiContexts,
    mut settings: ResMut<SimSettings>,
    stats: Res<SimStatistics>,
    ant_query: Query<&mut Visibility, With<Ant>>,
    windows: Query<&Window>,
    mut pheromones: ResMut<Pheromones>,
    mut reset_event: EventWriter<crate::ResetSimulationEvent>,
) {
    if !settings.is_show_menu {
        return;
    }

    if let Ok(window) = windows.get_single() {
        if window.physical_width() > 16384 {
            return;
        }
    }

    let ctx = contexts.ctx_mut();

    egui::Window::new("no-title")
        .title_bar(false)
        .default_pos(egui::pos2(10.0, 40.0)) // 왼쪽 상단으로 이동 (상단 바 아래)
        .show(ctx, |ui| {
            egui::CollapsingHeader::new("Stats")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label(format!("Pheromones: {:?}", stats.ph_food_size));
                    ui.label(format!("Cache: {:?}", stats.food_cache_size));
                    ui.label(format!("Scan radius: {:?}", stats.scan_radius.round()));
                    ui.label(format!("Num ants: {:?}", stats.num_ants));
                });
            egui::CollapsingHeader::new("Settings")
                .default_open(true)
                .show(ui, |ui| {
                    ui.checkbox(&mut settings.is_show_food_ph, "Show Pheromones")
                        .on_hover_text("Shortcut: F");
                    ui.checkbox(&mut settings.is_show_ants_path, "Paths")
                        .on_hover_text("Shortcut: P");
                    ui.checkbox(&mut settings.is_camera_follow, "Camera follow");
                    if ui
                        .checkbox(&mut settings.is_show_ants, "Ants")
                        .on_hover_text("Shortcut: A")
                        .clicked()
                    {
                        toggle_ant_visibility(ant_query, settings.is_show_ants);
                    };
                    ui.separator();
                    ui.label("Max Ants")
                        .on_hover_text("Set the population ceiling for your colony");
                    ui.add(egui::Slider::new(&mut settings.max_ants, 1..=5000));
                    ui.label("Scout Ratio (%)").on_hover_text(
                        "Adjust how many ants ignore pheromones to explore new territories",
                    );
                    ui.add(egui::Slider::new(&mut settings.scout_ratio, 0.0..=10.0));
                    ui.label("Ph Decay Rate")
                        .on_hover_text("Control how fast pheromone trails evaporate");
                    ui.add(
                        egui::Slider::new(&mut settings.ph_decay_rate, 0.01..=3.0)
                            .logarithmic(true),
                    );

                    ui.separator();
                    if ui
                        .button("Reset Pheromones (Wind)")
                        .on_hover_text("Wipe all existing trails")
                        .clicked()
                    {
                        pheromones.reset();
                    }

                    if ui
                        .button("Full Reset Simulation")
                        .on_hover_text("Clear everything and start over with 10 ants")
                        .clicked()
                    {
                        reset_event.send(crate::ResetSimulationEvent);
                    }

                    ui.separator();
                    ui.label("Controls");
                    ui.label("Shift + L-Click: Place Food");
                });
        });
}

fn toggle_ant_visibility(mut ant_query: Query<&mut Visibility, With<Ant>>, is_visible: bool) {
    for mut ant in ant_query.iter_mut() {
        if is_visible {
            *ant = Visibility::Visible;
        } else {
            *ant = Visibility::Hidden;
        }
    }
}

impl Default for SimSettings {
    fn default() -> Self {
        Self {
            is_show_food_ph: true,
            is_show_ants: true,
            is_camera_follow: false,
            is_show_menu: false,
            is_show_ants_path: true,
            max_ants: 1000,
            scout_ratio: 1.0,
            ph_decay_rate: PH_DECAY_RATE,
        }
    }
}
