use crate::{common::{colour_changer, Game}, draw_window_buffer, InputWrapper};
use egui::Ui;
use pong::{creation_pongs, display, Cli, Difficulty, World};
use rand::SeedableRng;
use web_time::Instant;
use window_rs::WindowBuffer;

pub struct Pong {
    buffer: WindowBuffer,
    config: World,
    instant_ball: Instant,
    instant_pong: Instant,
    cli: Cli,
}

impl Default for Pong {
    fn default() -> Self {
        let cli = Cli {
            ball_speed: 20,
            pong_speed: 60,
            difficulty: Difficulty::Medium,
            number_of_points_to_reach: 10,
        };
        let buffer: WindowBuffer = WindowBuffer::new(50, 33);
        let mut buff = [0; 8];
        getrandom::getrandom(&mut buff).unwrap();
        let seed: u64 = u64::from_be_bytes(buff);
        let config = World::new(
            Vec::new(),
            Vec::new(),
            0,
            0,
            pong::Direction::Still,
            pong::Direction::Still,
            Some((buffer.width() / 2, buffer.height() / 2)),
            pong::BallDirection::Still,
            false,
            Instant::now(),
            0,
            cli.pong_speed,
            cli.ball_speed,
            rand::rngs::StdRng::seed_from_u64(seed),
            0x00FF0000,
            0xFF00FF00,
            0xFFFFFF00,
        );

        Self {
            buffer,
            config,
            instant_ball: Instant::now(),
            instant_pong: Instant::now(),
            cli,
        }
    }
}

impl Pong {
    pub fn configuration(&mut self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label("Reset pong board:");
            if ui.add(egui::Button::new("Reset")).clicked() {
                *self = Pong::default();
            };

            ui.separator();

            ui.label("Ball speed:");
            ui.add(egui::DragValue::new(&mut self.cli.ball_speed).speed(1));

            ui.separator();

            ui.label("Pong speed:");
            ui.add(egui::DragValue::new(&mut self.cli.pong_speed).speed(1));

            ui.separator();

            ui.label("Colour player 1:");
            let rgba_player: u32 = self.config.player_1_colour;
            let player_color = colour_changer(rgba_player, ui);
            self.config.player_1_colour = player_color;

            ui.separator();

            ui.label("Colour player 2:");
            let rgba_player: u32 = self.config.player_2_colour;
            let player_color = colour_changer(rgba_player, ui);
            self.config.player_2_colour = player_color;

            ui.separator();

            ui.label("Colour ball:");
            let rgba_ball: u32 = self.config.ball_colour;
            let ball_color = colour_changer(rgba_ball, ui);
            self.config.ball_colour = ball_color;

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

impl Game for Pong {
    fn name() -> &'static str {
        "Pong"
    }

    fn github() -> &'static str {
        "https://github.com/NoodleSamaChan/pong"
    }

    fn update(&mut self, ctx: &egui::Context) {
        if self.config.player_1_pong.is_empty() {
            creation_pongs(&mut self.config, &self.buffer)
        }

        ctx.input(|i| {
            let _ = self
                .config
                .handle_user_input(&InputWrapper { input: i }, &self.buffer);
        });

        self.config.update(
            &mut self.buffer,
            &self.cli,
            &mut self.instant_pong,
            &mut self.instant_ball,
        );
        display(&self.config, &mut self.buffer);
    }

    fn draw(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ctx.request_repaint();
        draw_window_buffer(ui, &self.buffer)
    }
}
