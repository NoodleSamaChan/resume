use egui::Ui;
use snake::{
    display, go_display, return_in_time, snake_generator, Cli, Difficulty, Direction, TimeCycle,
    World,
};
use web_time::{Duration, Instant};
use window_rs::WindowBuffer;

use crate::{
    common::{colour_changer, Game},
    draw_window_buffer, InputWrapper,
};

pub struct Snake {
    buffer: WindowBuffer,
    config: World,
    cli: Cli,
    snake_instant: Instant,
    options: SnakeOptions,
    points_to_reach: usize,
}

pub struct SnakeOptions {
    snake_speed: usize,
    two_player: bool,
    bad_berries: bool,
    snake_size: usize,
    ghost_mode: bool,
    first_snake_colour: u32,
    first_snake_head_colour: u32,
    second_snake_colour: u32,
    second_snake_head_colour: u32,
    food_colour: u32,
    bad_berry_colour: u32,
    points_to_reach: usize,
}

impl Default for Snake {
    fn default() -> Self {
        let options = SnakeOptions {
            snake_speed: 60,
            two_player: false,
            bad_berries: false,
            snake_size: 3,
            ghost_mode: true,
            first_snake_colour: 0x0033CCFF,
            first_snake_head_colour: 0x00CC66FF,
            second_snake_colour: 0x00CC33FF,
            second_snake_head_colour: 0x00FFCC00,
            food_colour: 0x0066CC33,
            bad_berry_colour: 0x00FF0000,
            points_to_reach: 15,
        };
        let cli = Cli {
            width: 80,
            height: 50*(16/9),
            snake_size_start: 3,
            file_path: None,
            snake_speed: 60,
            speed_increase: Difficulty::Medium,
            bad_berries: false,
            ghost_mode: true,
            two_players_mode: false,
        };
        let buffer: WindowBuffer = WindowBuffer::new(cli.width, cli.height);
        let config = create_world(cli.snake_speed);
        Self {
            buffer,
            config,
            cli,
            snake_instant: Instant::now(),
            options,
            points_to_reach: 15,
        }
    }
}

impl Snake {
    pub fn new_snake_w_options(&mut self) -> Self {
        let mut base_snake = Snake::default();

        base_snake.cli.bad_berries = self.options.bad_berries;
        base_snake.cli.snake_size_start = self.options.snake_size;
        base_snake.cli.ghost_mode = self.options.ghost_mode;
        base_snake.cli.snake_speed = self.options.snake_speed;
        base_snake.cli.two_players_mode = self.options.two_player;
        base_snake.config.first_snake_colour = self.options.first_snake_colour;
        base_snake.config.second_snake_colour = self.options.second_snake_colour;
        base_snake.config.food_colour = self.options.food_colour;
        base_snake.config.bad_berries_colour = self.options.bad_berry_colour;
        base_snake.config.first_snake_head_colour = self.options.first_snake_head_colour;
        base_snake.config.second_snake_head_colour = self.options.second_snake_head_colour;
        base_snake.points_to_reach = self.options.points_to_reach;
        Self {
            buffer: base_snake.buffer,
            config: base_snake.config,
            cli: base_snake.cli,
            snake_instant: base_snake.snake_instant,
            options: base_snake.options,
            points_to_reach: base_snake.points_to_reach,
        }
    }
    pub fn configuration(&mut self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label("Reset snake board:");
            if ui.add(egui::Button::new("Reset")).clicked() {
                *self = Snake::default();
            };

            ui.separator();

            ui.label("Snake starting size:");
            ui.add(egui::Slider::new(&mut self.options.snake_size, 0..=10).suffix("pixels"));

            ui.separator();

            ui.label("Number of points to reach:");
            ui.add(egui::Slider::new(&mut self.options.points_to_reach, 0..=50).suffix("points"));

            ui.separator();

            ui.label("Snake speed:");
            ui.add(egui::DragValue::new(&mut self.options.snake_speed).speed(1));

            ui.separator();

            ui.label("Bad berries:");
            ui.add(egui::Checkbox::new(
                &mut self.options.bad_berries,
                "Checked",
            ));

            ui.separator();

            ui.label("Ghost mode:");
            ui.add(egui::Checkbox::new(&mut self.options.ghost_mode, "Checked"));

            ui.separator();

            ui.label("2 players:");
            ui.add(egui::Checkbox::new(&mut self.options.two_player, "Checked"));

            ui.separator();

            ui.label("Colour of the head of snake player 1:");
            let rgba_first_snake: u32 = self.config.first_snake_head_colour;
            let first_snake_head_colour = colour_changer(rgba_first_snake, ui);
            self.config.first_snake_head_colour = first_snake_head_colour;
            self.options.first_snake_head_colour = self.config.first_snake_head_colour;

            ui.separator();

            ui.label("Colour snake player 1:");
            let rgba_first_snake: u32 = self.config.first_snake_colour;
            let first_snake_colour = colour_changer(rgba_first_snake, ui);
            self.config.first_snake_colour = first_snake_colour;
            self.options.first_snake_colour = self.config.first_snake_colour;

            ui.separator();

            ui.label("Colour of the head of snake player 2:");
            let rgba_second_snake: u32 = self.config.second_snake_head_colour;
            let second_snake_head_colour = colour_changer(rgba_second_snake, ui);
            self.config.second_snake_head_colour = second_snake_head_colour;
            self.options.second_snake_head_colour = self.config.second_snake_head_colour;

            ui.separator();

            ui.label("Colour snake player 2:");
            let rgba_second_snake: u32 = self.config.second_snake_colour;
            let second_snake_colour = colour_changer(rgba_second_snake, ui);
            self.config.second_snake_colour = second_snake_colour;
            self.options.second_snake_colour = self.config.second_snake_colour;

            ui.separator();

            ui.label("Colour berry:");
            let rgba_berry: u32 = self.config.food_colour;
            let berry_colour = colour_changer(rgba_berry, ui);
            self.config.food_colour = berry_colour;
            self.options.food_colour = self.config.food_colour;

            ui.separator();

            ui.label("Colour bad berry:");
            let rgba_bad_berry: u32 = self.config.bad_berries_colour;
            let bad_berry_colour = colour_changer(rgba_bad_berry, ui);
            self.config.bad_berries_colour = bad_berry_colour;
            self.options.bad_berry_colour = self.config.bad_berries_colour;

            ui.separator();

            ui.label("Player 1 points:");
            let mut points = self.config.score as f32;
            let max = (self.points_to_reach as f32) * 10.0;
            ui.add(egui::Slider::new(&mut points, 0.0..=max).suffix("points"));

            ui.separator();

            ui.label("Player 2 points:");
            let mut points = self.config.second_score as f32;
            let max = (self.points_to_reach as f32) * 10.0;
            ui.add(egui::Slider::new(&mut points, 0.0..=max).suffix("points"));

            ui.separator();

            ui.label("Create snake board with all your options:");
            if ui.add(egui::Button::new("Create")).clicked() {
                *self = self.new_snake_w_options();
            };
        })
        .response
    }

    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update(ctx);
        egui::SidePanel::right("Configuration").show(ctx, |ui| self.configuration(ui));
        egui::CentralPanel::default().show(ctx, |ui| self.draw(ctx, ui));
    }
}

impl Game for Snake {
    fn name() -> &'static str {
        "Snake"
    }
    fn github() -> &'static str {
        "https://github.com/NoodleSamaChan/snake"
    }

    fn update(&mut self, ctx: &egui::Context) {
        if self.config.food == (0, 0) {
            self.config.food_generator(&self.buffer, &self.cli)
        };

        if self.config.snake.is_empty() {
            snake_generator(&mut self.config, &self.buffer, &self.cli);
        };
        ctx.input(|i| {
            let _ = self.config.handle_user_input(
                &InputWrapper {
                    input: i,
                    cell_size: None,
                    mouse_pos: None,
                },
                &self.cli,
                &self.buffer,
            );
        });

        if (self.points_to_reach == self.config.score / 10)
            || (self.points_to_reach == self.config.second_score / 10)
        {
            self.config.finished = true;
        }

        if self.config.time_cycle == TimeCycle::Forward {
            if !self.config.finished {
                let elapsed_time = Duration::from_millis(self.cli.snake_speed as u64);

                if self.snake_instant.elapsed() >= elapsed_time {
                    self.config.update(&mut self.buffer, &self.cli);
                    self.snake_instant = Instant::now();
                }
                display(&self.config, &mut self.buffer, &self.cli);
            } else {
                go_display(&mut self.config, &mut self.buffer, &self.cli);
            }
        } else if self.config.time_cycle == TimeCycle::Backward {
            let elapsed_time = Duration::from_millis(100);

            if self.snake_instant.elapsed() >= elapsed_time {
                return_in_time(&mut self.config, &self.cli);
                self.snake_instant = Instant::now();
            }
            display(&self.config, &mut self.buffer, &self.cli);
            self.config.time_cycle = TimeCycle::Pause;
        }
    }

    fn draw(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ctx.request_repaint();
        draw_window_buffer(ui, &self.buffer);
    }

    fn resize(&mut self, ui: &mut egui::Ui) {
        let size = 30.0;

        let max_width = (ui.available_width() / size) as usize;
        let max_height = (ui.available_height() / size) as usize;

        self.buffer = WindowBuffer::new(max_width, max_height);
        self.config = create_world(self.cli.snake_speed);
    }
}

fn create_world(snake_speed: usize) -> World {
    World::new(
        Direction::Still,
        vec![Direction::Still],
        Vec::new(),
        (0, 0),
        false,
        Instant::now(),
        0,
        snake_speed,
        0,
        0,
        None,
        Vec::new(),
        TimeCycle::Forward,
        Some(Vec::new()),
        vec![Direction::Still],
        Some(Vec::new()),
        Direction::Still,
        0,
        0x0033CCFF,
        0x00CC66FF,
        0x00CC33FF,
        0x00FFCC00,
        0x0066CC33,
        0x00FF0000,
    )
}
