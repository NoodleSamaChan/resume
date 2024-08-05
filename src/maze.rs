use egui::{RichText, Ui};
use maze::start_end_generator;
use maze::{display, MazeConfig, Player};
use rand::SeedableRng;
use web_time::{Duration, Instant};
use window_rs::WindowBuffer;

use crate::common::Game;
use crate::{common::colour_changer, draw_window_buffer, InputWrapper};

pub struct Maze {
    buffer: WindowBuffer,
    seed: u64,
    pub config: MazeConfig,
    player: Player,
    start_point: (usize, usize),
    update_time_wait: Instant,
}

impl Default for Maze {
    fn default() -> Self {
        let mut buffer: WindowBuffer = WindowBuffer::new(30 + 30 * (16/10), 40 );

        let mut buff = [0; 8];
        getrandom::getrandom(&mut buff).unwrap();
        let seed: u64 = u64::from_be_bytes(buff);
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let config = MazeConfig::default();
        config.generate(&mut buffer, &mut rng);
        let mut player = Player::new(
            (0, 0),
            (0, 0),
            maze::Direction::Still,
            (0, 0),
            config.clone(),
            false,
        );

        let start_point = start_end_generator(&mut buffer, &mut rng, &mut player);
        Self {
            seed,
            config,
            player,
            start_point,
            buffer,
            update_time_wait: Instant::now(),
        }
    }
}

impl Maze {
    pub fn configuration(&mut self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label("Reset maze:");
            if ui.add(egui::Button::new("Reset")).clicked() {
                *self = Maze::default();
            };

            ui.label("Wall:");
            let rgba_wall: u32 = self.config.wall_color;
            let wall_color = colour_changer(rgba_wall, ui);
            self.config.wall_color = wall_color;

            ui.separator();

            ui.label("Path:");
            let rgba_path = self.config.path_color;
            let mut path_color = colour_changer(rgba_path, ui);
            if path_color == wall_color && wall_color != u32::MAX {
                path_color += 1;
            } else if path_color == wall_color && wall_color == u32::MAX {
                path_color -= 1;
            }
            self.config.path_color = path_color;

            ui.separator();

            ui.label("Colour player:");
            let rgba_player: u32 = self.player.player_color;
            let player_color = colour_changer(rgba_player, ui);
            self.player.player_color = player_color;

            ui.separator();

            ui.label("Colour ending:");
            let rgba_ending: u32 = self.player.finish_color;
            let mut ending_color = colour_changer(rgba_ending, ui);
            if ending_color == wall_color && wall_color != u32::MAX {
                ending_color += 1;
            } else if ending_color == wall_color && wall_color == u32::MAX {
                ending_color -= 1;
            }
            self.player.finish_color = ending_color;

            ui.separator();

            let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
            self.config.generate(&mut self.buffer, &mut rng);
            self.player.maze_config = self.config.clone();
            display(&self.player, &mut self.buffer);

            ui.label(RichText::new("How to play maze: ").strong());
            ui.label("Use the up, down, left, and right arrow to move your character around around!");

            ui.separator();
        })
        .response
    }

    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update(ctx);
        egui::SidePanel::right("Configuration").show(ctx, |ui| self.configuration(ui));
        egui::CentralPanel::default().show(ctx, |ui| self.draw(ctx, ui));
    }
}

impl Game for Maze {
    fn name() -> &'static str {
        "Maze"
    }

    fn github() -> &'static str {
        "https://github.com/NoodleSamaChan/naze"
    }

    fn update(&mut self, ctx: &egui::Context) {
        let elapsed_time = Duration::from_millis(1_u64);
        ctx.input(|i| {
            let _ = self.player.handle_user_input(
                &InputWrapper {
                    input: i,
                    cell_size: None,
                    mouse_pos: None,
                },
                &self.start_point,
            );
        });

        if self.update_time_wait.elapsed() >= elapsed_time {
            display(&self.player, &mut self.buffer);
            self.player.direction(&self.buffer);
            self.update_time_wait = Instant::now();
        }
    }

    fn draw(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        draw_window_buffer(ui, &self.buffer);
    }

    fn resize(&mut self, ui: &mut egui::Ui) {
        let size = 30.0;

        let max_width = (ui.available_width() / size) as usize;
        let max_height = (ui.available_height() / size) as usize;

        self.buffer = WindowBuffer::new(max_width, max_height);
        self.config = MazeConfig::default();
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
        self.config.generate(&mut self.buffer, &mut rng);
        let config = self.config.clone();
        self.player = create_world(config);
        start_end_generator(&mut self.buffer, &mut rng, &mut self.player);
    }
}

fn create_world(config: MazeConfig) -> Player {
    Player::new(
        (0, 0),
        (0, 0),
        maze::Direction::Still,
        (0, 0),
        config.clone(),
        false,
    )
}
