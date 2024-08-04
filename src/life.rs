use egui::{Response, Ui};
use game_of_life::{Cli, World};
use web_time::{Duration, Instant};
use window_rs::WindowBuffer;

use crate::{
    common::{colour_changer, Game},
    draw_window_buffer, InputWrapper,
};

pub struct Life {
    cli: Cli,
    config: World,
    time_check: Instant,
    cell_size: Option<f32>,
    mouse_pos: Option<Response>,
}

impl Default for Life {
    fn default() -> Self {
        let cli = Cli {
            width: 100+100*(3/2),
            height: 100,
            file_path: None,
        };
        let config = World::new(
            WindowBuffer::new(cli.width, cli.height),
            0,
            Instant::now(),
            2,
            0x0066CC33,
        );
        let time_check = Instant::now();

        Self {
            cli,
            config,
            time_check,
            cell_size: None,
            mouse_pos:  None,
        }
    }
}

impl Life {
    pub fn configuration(&mut self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label("Reset game of life board:");
            if ui.add(egui::Button::new("Reset")).clicked() {
                *self = Life::default();
            };

            ui.separator();

            ui.label("Colour cells:");
            let cell_colour: u32 = self.config.colour_cell;
            let berry_colour = colour_changer(cell_colour, ui);
            self.config.colour_cell = berry_colour;

            ui.separator();

            ui.label("Speed of the game:");
            ui.add(egui::Slider::new(&mut self.config.speed, 0..=50));
        })
        .response
    }

    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Configuration").show(ctx, |ui| self.configuration(ui));
        self.update(ctx);
        egui::CentralPanel::default().show(ctx, |ui| self.draw(ctx, ui));
    }
}

impl Game for Life {
    fn name() -> &'static str {
        "Game of life"
    }

    fn github() -> &'static str {
        "https://github.com/NoodleSamaChan/rust_project/tree/main/game_of_life"
    }

    fn update(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            let _ = self.config.handle_user_input(
                &InputWrapper {
                    input: i,
                    cell_size: self.cell_size,
                    mouse_pos: self.mouse_pos.clone(),

                },
                &self.cli,
            );
        });

        let two_seconds = Duration::from_secs(self.config.speed());
        if self.time_check.elapsed() >= two_seconds {
            self.config.update();
            self.time_check = Instant::now();
        }
    }

    fn draw(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ctx.request_repaint();
        let (cell_size, mouse_pos) = draw_window_buffer(ui, &self.config.window_buffer);
        self.cell_size = Some(cell_size);
        self.mouse_pos = Some(mouse_pos);
        //println!("cell size is {:#?} and mouse pos is {:#?}", self.cell_size, self.mouse_pos);
    }

    fn resize(&mut self, ui: &mut egui::Ui) {
        let size = 30.0;

        let max_width = (ui.available_width() / size) as usize;
        let max_height = (ui.available_height() / size) as usize;

        self.config = create_world(max_width, max_height);
    }
}

fn create_world(width: usize, height: usize) -> World {
    World::new(
        WindowBuffer::new(width, height),
        0,
        Instant::now(),
        2,
        0x0066CC33,
    )
}
