use crate::wrapped_image::WrappedImage;
use lapix_core::{Canvas, CanvasEffect, Event, State, Tool};
use lapix_core::primitives::*;
use macroquad::prelude::*;
use std::default::Default;

pub const WINDOW_W: i32 = 1000;
pub const WINDOW_H: i32 = 600;
pub const CANVAS_W: u16 = 64;
pub const CANVAS_H: u16 = 64;
const CANVAS_SCALE: f32 = 8.;
const LEFT_TOOLBAR_W: u16 = 300;

// Center on the space after the toolbar
const CANVAS_X: f32 = LEFT_TOOLBAR_W as f32 + ((WINDOW_W as u16 - LEFT_TOOLBAR_W) / 2) as f32
    - (CANVAS_W as f32 * CANVAS_SCALE / 2.);
const CANVAS_Y: f32 = (WINDOW_H / 2) as f32 - (CANVAS_H as f32 * CANVAS_SCALE / 2.);

fn rgba_to_rgb_u8(color: [u8; 4]) -> [u8; 3] {
    [color[0], color[1], color[2]]
}

pub struct UiState {
    inner: State<WrappedImage>,
    camera: Position<f32>,
    canvas_pos: Position<f32>,
    zoom: f32,
    drawing: Texture2D,
    canvas_w_str: String,
    canvas_h_str: String,
    brush: [u8; 3],
}

impl Default for UiState {
    fn default() -> Self {
        let state = State::<WrappedImage>::new(CANVAS_W, CANVAS_H);
        let drawing = Texture2D::from_image(&state.canvas().inner().0);
        drawing.set_filter(FilterMode::Nearest);
        let canvas_w_str = state.canvas().width().to_string();
        let canvas_h_str = state.canvas().height().to_string();

        Self {
            inner: state,
            camera: (0., 0.).into(),
            canvas_pos: (CANVAS_X, CANVAS_Y).into(),
            zoom: 8.,
            drawing,
            canvas_w_str,
            canvas_h_str,
            brush: [0, 0, 0],
        }
    }
}

impl UiState {
    pub fn camera(&self) -> Position<f32> {
        self.camera
    }
    pub fn canvas(&self) -> &Canvas<WrappedImage> {
        &self.inner.canvas()
    }
    pub fn execute(&mut self, event: Event<WrappedImage>) {
        let effect = event.canvas_effect();
        self.inner.execute(event);

        match effect {
            CanvasEffect::Update => self.drawing.update(&self.canvas().inner().0),
            CanvasEffect::New => {
                self.drawing = Texture2D::from_image(&self.canvas().inner().0);
                self.drawing.set_filter(FilterMode::Nearest);
            }
            CanvasEffect::None => (),
        }
    }
    pub fn canvas_w_str(&mut self) -> &mut String {
        &mut self.canvas_w_str
    }
    pub fn canvas_h_str(&mut self) -> &mut String {
        &mut self.canvas_h_str
    }
    pub fn brush(&mut self) -> &mut [u8; 3] {
        self.brush = rgba_to_rgb_u8(self.main_color());

        &mut self.brush
    }
    pub fn canvas_pos(&self) -> Position<f32> {
        self.canvas_pos
    }
    pub fn canvas_size(&self) -> Size<f32> {
        (
            self.inner.canvas().width() as f32,
            self.inner.canvas().height() as f32,
        )
            .into()
    }
    pub fn canvas_actual_size(&self) -> Size<f32> {
        (
            self.inner.canvas().width() as f32 * self.zoom,
            self.inner.canvas().height() as f32 * self.zoom,
        )
            .into()
    }
    pub fn main_color(&self) -> [u8; 4] {
        self.inner.main_color()
    }
    pub fn selected_tool(&self) -> Tool {
        self.inner.selected_tool()
    }
    pub fn zoom(&self) -> f32 {
        self.zoom
    }
    pub fn zoom_in(&mut self) {
        self.zoom *= 2.;
    }
    pub fn zoom_out(&mut self) {
        self.zoom /= 2.;
    }
    pub fn move_camera(&mut self, direction: Direction) {
        let speed = 0.5 * self.zoom;

        if !self.is_camera_off(direction) {
            match direction {
                Direction::Up => self.camera.y -= speed,
                Direction::Down => self.camera.y += speed,
                Direction::Left => self.camera.x -= speed,
                Direction::Right => self.camera.x += speed,
            }
        }
    }
    fn is_camera_off(&self, direction: Direction) -> bool {
        let buffer = 20.;
        let canvas_size = self.canvas_actual_size();
        let canvas_pos = self.canvas_pos;
        let camera = self.camera;
        let win_w = WINDOW_W as f32;
        let win_h = WINDOW_H as f32;

        match direction {
            Direction::Up => canvas_pos.y - camera.y > win_h as f32 - buffer,
            Direction::Down => camera.y > canvas_pos.y + canvas_size.y - buffer,
            Direction::Left => canvas_pos.x - camera.x > win_w as f32 - buffer,
            Direction::Right => camera.x > canvas_pos.x + canvas_size.x - buffer,
        }
    }
    pub fn draw_canvas_bg(&self) {
        let scale = self.zoom();

        let x = self.canvas_pos().x - self.camera().x;
        let y = self.canvas_pos().y - self.camera().y;
        let w = self.drawing.width() * scale;
        let h = self.drawing.height() * scale;

        let side = 4. * scale;

        // TODO: optimize this by storing a rendered texture that contains all
        // BG rectangles
        let bg1 = Color::new(0.875, 0.875, 0.875, 1.);
        let bg2 = Color::new(0.75, 0.75, 0.75, 1.);
        for i in 0..(w / side + 1.) as usize {
            for j in 0..(h / side + 1.) as usize {
                let cur_w = i as f32 * side;
                let cur_h = j as f32 * side;
                let next_w = (i + 1) as f32 * side;
                let next_h = (j + 1) as f32 * side;
                let x = x + i as f32 * side;
                let y = y + j as f32 * side;
                let w = if next_w <= w { side } else { w - cur_w };
                let h = if next_h <= h { side } else { h - cur_h };
                let color = if (i + j) % 2 == 0 { bg1 } else { bg2 };
                draw_rectangle(x, y, w, h, color);
            }
        }
    }
    pub fn draw_canvas(&self) {
        let texture = self.drawing;
        let w = texture.width();
        let h = texture.height();

        let x = self.canvas_pos().x - self.camera().x;
        let y = self.canvas_pos().y - self.camera().y;
        let scale = self.zoom();

        let params = DrawTextureParams {
            dest_size: Some(Vec2 {
                x: w * scale,
                y: h * scale,
            }),
            ..Default::default()
        };

        draw_texture_ex(texture, x, y, WHITE, params);
    }
    pub fn screen_to_canvas(&self, x: f32, y: f32) -> (i16, i16) {
        let canvas_x = self.canvas_pos().x - self.camera().x;
        let canvas_y = self.canvas_pos().y - self.camera().y;
        let scale = self.zoom();

        (
            ((x - canvas_x) / scale) as i16,
            ((y - canvas_y) / scale) as i16,
        )
    }
}
