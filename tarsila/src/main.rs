use egui_macroquad::macroquad::{self, prelude::*};

mod bg;
mod error;
mod graphics;
mod gui;
mod input;
mod mouse;
mod project;
mod resource;
mod ui_state;
mod util;
mod wrapped_image;

const VERSION: &str = env!("CARGO_PKG_VERSION");

use error::Result;
use resource::Resources;
use ui_state::{Effect, UiEvent, UiState, WINDOW_H, WINDOW_W};
use util::*;

fn window_conf() -> Conf {
    Conf {
        window_title: format!("Tarsila {VERSION}: Pixel Art and 2D Sprite Editor"),
        window_width: WINDOW_W,
        window_height: WINDOW_H,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = UiState::default();
    let mut frame = 0;

    loop {
        if let Err(e) = state.update(frame) {
            eprintln!("ERROR: {e}");
        }

        if let Err(e) = state.draw() {
            eprintln!("ERROR: {e}");
        }

        next_frame().await;

        if state.must_exit() {
            break;
        }

        frame += 1;
    }
}
