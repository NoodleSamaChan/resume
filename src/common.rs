use egui::{
    color_picker::{color_edit_button_rgba, Alpha},
    Color32, InputState, Key, Pos2, Rgba, Rounding, Sense, Ui, Vec2,
};
use graphic::Graphic;
use window_rs::WindowBuffer;

/// This trait define the method that everygame should provides.
pub trait Game: Default {
    /// The name of the game
    fn name() -> &'static str;
    /// The link to the github repositiory
    fn github() -> &'static str;

    /// Update all the internal state of the game.
    /// Will be called on every frame the game is being played.
    fn update(&mut self, ctx: &egui::Context);

    /// Redraw the game in the specified ui.
    fn draw(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);

    fn resize(&mut self, ui: &mut egui::Ui);
}

pub struct InputWrapper<'a> {
    pub input: &'a InputState,
    pub cell_size: Option<f32>,
}

impl Graphic for InputWrapper<'_> {
    fn new(_name: &str, _width: usize, _height: usize) -> Self {
        todo!()
    }

    fn is_open(&self) -> bool {
        true
    }

    fn is_key_down(&self, key: graphic::Key) -> bool {
        match key {
            graphic::Key::Up => self.input.key_down(Key::ArrowUp),
            graphic::Key::Down => self.input.key_down(Key::ArrowDown),
            graphic::Key::Left => self.input.key_down(Key::ArrowLeft),
            graphic::Key::Right => self.input.key_down(Key::ArrowRight),
            graphic::Key::Escape => self.input.key_down(Key::Escape),
            graphic::Key::Quit => self.input.key_down(Key::Delete),
            graphic::Key::Space => self.input.key_down(Key::Space),
            graphic::Key::UpPlayer1 => self.input.key_down(Key::E),
            graphic::Key::DownPlayer1 => self.input.key_down(Key::D),
            graphic::Key::UpPlayer2 => self.input.key_down(Key::O),
            graphic::Key::DownPlayer2 => self.input.key_down(Key::L),
            graphic::Key::LeftPlayer2 => self.input.key_down(Key::K),
            graphic::Key::RightPlayer2 => self.input.key_down(Key::M),
            graphic::Key::Launch => self.input.key_down(Key::W),
            graphic::Key::Forward => self.input.key_down(Key::F),
            graphic::Key::Backward => self.input.key_down(Key::R),
            graphic::Key::Save => self.input.key_down(Key::S),
        }
    }

    fn update_with_buffer(&mut self, _windows: &WindowBuffer) {
        todo!()
    }

    fn is_key_pressed(&self, key: graphic::Key) -> bool {
        match key {
            graphic::Key::Up => self.input.key_pressed(Key::ArrowUp),
            graphic::Key::Down => self.input.key_pressed(Key::ArrowDown),
            graphic::Key::Left => self.input.key_pressed(Key::ArrowLeft),
            graphic::Key::Right => self.input.key_pressed(Key::ArrowRight),
            graphic::Key::Escape => self.input.key_pressed(Key::Escape),
            graphic::Key::Quit => self.input.key_pressed(Key::Delete),
            graphic::Key::Space => self.input.key_pressed(Key::Space),
            graphic::Key::UpPlayer1 => self.input.key_pressed(Key::E),
            graphic::Key::DownPlayer1 => self.input.key_pressed(Key::D),
            graphic::Key::UpPlayer2 => self.input.key_pressed(Key::O),
            graphic::Key::DownPlayer2 => self.input.key_pressed(Key::L),
            graphic::Key::LeftPlayer2 => self.input.key_pressed(Key::K),
            graphic::Key::RightPlayer2 => self.input.key_pressed(Key::M),
            graphic::Key::Launch => self.input.key_pressed(Key::W),
            graphic::Key::Forward => self.input.key_pressed(Key::F),
            graphic::Key::Backward => self.input.key_pressed(Key::R),
            graphic::Key::Save => self.input.key_pressed(Key::S),
        }
    }

    fn get_keys_released(&self) -> Vec<graphic::Key> {
        let mut ret = vec![];
        if self.input.key_released(Key::ArrowUp) {
            ret.push(graphic::Key::Up)
        }
        if self.input.key_released(Key::ArrowDown) {
            ret.push(graphic::Key::Down)
        }
        if self.input.key_released(Key::ArrowLeft) {
            ret.push(graphic::Key::Left)
        }
        if self.input.key_released(Key::ArrowRight) {
            ret.push(graphic::Key::Right)
        }
        if self.input.key_released(Key::Escape) {
            ret.push(graphic::Key::Escape)
        }
        if self.input.key_released(Key::Delete) {
            ret.push(graphic::Key::Quit)
        }
        if self.input.key_released(Key::Space) {
            ret.push(graphic::Key::Space)
        }
        if self.input.key_released(Key::E) {
            ret.push(graphic::Key::UpPlayer1)
        }
        if self.input.key_released(Key::D) {
            ret.push(graphic::Key::DownPlayer1)
        }
        if self.input.key_released(Key::O) {
            ret.push(graphic::Key::UpPlayer2)
        }
        if self.input.key_released(Key::L) {
            ret.push(graphic::Key::DownPlayer2)
        }
        if self.input.key_released(Key::K) {
            ret.push(graphic::Key::LeftPlayer2)
        }
        if self.input.key_released(Key::M) {
            ret.push(graphic::Key::RightPlayer2)
        }
        if self.input.key_released(Key::W) {
            ret.push(graphic::Key::Launch)
        }
        if self.input.key_released(Key::F) {
            ret.push(graphic::Key::Forward)
        }
        if self.input.key_released(Key::R) {
            ret.push(graphic::Key::Backward)
        }
        if self.input.key_released(Key::S) {
            ret.push(graphic::Key::Save)
        }
        ret
    }

    fn get_mouse_pos(&self, mouse: graphic::Mouse) -> Option<(f32, f32)> {
        let Some(cell_size) = self.cell_size else {
            return None;
        };
        let ret = self.input.pointer.interact_pointer_pos()?;
        let retured_pos = (ret[0] / cell_size, ret[1] / cell_size);
        println!("mouse position: {:#?}", retured_pos);
        let clicked = match mouse {
            graphic::Mouse::Left => self.input.pointer.any_pressed(),
            graphic::Mouse::Right => self.input.pointer.any_pressed(),
            graphic::Mouse::Discard => self.input.pointer.any_pressed(),
        };
        println!("first click is {}", clicked);
        if clicked {
            Some(retured_pos)
        } else {
            None
        }
    }

    fn get_mouse_down(&self, mouse: graphic::Mouse) -> bool {
        println!("mouse clicked");
        match mouse {
            graphic::Mouse::Left => self.input.pointer.any_pressed(),
            graphic::Mouse::Right => self.input.pointer.any_pressed(),
            graphic::Mouse::Discard => self.input.pointer.any_pressed(),
        }
    }
}

pub fn draw_window_buffer(ui: &mut egui::Ui, window: &WindowBuffer) -> f32 {
    let max_width = ui.available_width();
    let max_height = ui.available_height();
    let size = (max_width / window.width() as f32).min(max_height / window.height() as f32);
    egui::ScrollArea::both().show(ui, |ui| {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(max_width, ui.available_height()), Sense::hover());

        let base_position = response.rect.left_top().to_vec2();
        for x in 0..window.width() {
            for y in 0..window.height() {
                let color = window[(x, y)];

                let rect = egui::Rect {
                    min: Pos2 {
                        x: x as f32 * size,
                        y: y as f32 * size,
                    },
                    max: Pos2 {
                        x: (x + 1) as f32 * size,
                        y: (y + 1) as f32 * size,
                    },
                };

                let rect = rect.translate(base_position);

                let [r, g, b, a] = color.to_ne_bytes();
                painter.rect_filled(
                    rect,
                    Rounding::ZERO,
                    Color32::from_rgba_premultiplied(r, g, b, a),
                );
            }
        }
    });
    size
}

pub fn colour_changer(rgba_colour_to_change: u32, ui: &mut Ui) -> u32 {
    let [r, g, b, a] = rgba_colour_to_change.to_le_bytes();
    let mut colour_player = Rgba::from_srgba_premultiplied(r, g, b, a);
    color_edit_button_rgba(ui, &mut colour_player, Alpha::Opaque);
    let convert_color = Rgba::to_srgba_unmultiplied(&colour_player);
    u32::from_le_bytes(convert_color)
}
