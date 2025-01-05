use egui_macroquad::macroquad::prelude::*;
use lapix::{Position, Size, Tool};
use {
    crate::{Effect, UiEvent, UiState},
    egui_macroquad::{egui, EguiMqInteg},
};

mod layers;
mod menu;
mod palette;
mod preview;
mod status;
mod toolbar;

use layers::LayersPanel;
use menu::MenuBar;
use palette::Palette;
use preview::Preview;
use status::StatusBar;
use toolbar::Toolbar;

#[derive(Debug, Clone)]
pub struct GuiSyncParams {
    pub main_color: [u8; 4],
    pub num_layers: usize,
    pub active_layer: usize,
    pub layers_vis: Vec<bool>,
    pub layers_alpha: Vec<u8>,
    pub palette: Vec<[u8; 4]>,
    pub mouse_canvas: Position<i32>,
    pub is_on_canvas: bool,
    pub selected_tool: Tool,
    pub visible_pixel_on_mouse: Option<[u8; 4]>,
    pub canvas_size: Size<i32>,
    pub spritesheet: Size<u8>,
    pub zoom: f32,
    pub fps: f32,
}

pub struct Gui {
    toolbar: Toolbar,
    layers_panel: LayersPanel,
    preview: Preview,
    palette: Palette,
    status_bar: StatusBar,
    menu: MenuBar,
    mouse_on_canvas: bool,
    selected_tool: Tool,
    pub egui_mq: EguiMqInteg,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            toolbar: Toolbar::new(),
            layers_panel: LayersPanel::new(),
            preview: Preview::new(),
            palette: Palette::new(),
            status_bar: StatusBar::new(),
            menu: MenuBar::new(),
            mouse_on_canvas: false,
            selected_tool: Tool::Brush,
            egui_mq: EguiMqInteg::new(),
        }
    }

    pub fn sync(&mut self, params: GuiSyncParams) {
        self.mouse_on_canvas = params.is_on_canvas;

        self.toolbar.sync(params.main_color);
        self.selected_tool = params.selected_tool;
        self.layers_panel.sync(
            params.num_layers,
            params.active_layer,
            params.layers_vis.clone(),
            params.layers_alpha.clone(),
        );
        self.preview.sync(
            params.spritesheet,
            params.canvas_size,
            params.layers_vis.clone(),
            params.layers_alpha.clone(),
        );
        self.palette.sync(params.palette.clone());
        self.menu.sync(params.canvas_size, params.spritesheet);
        self.status_bar.sync(params);
    }

    pub fn update(&mut self) -> Vec<Effect> {
        let mut events = Vec::new();

        let widget_color = egui::Color32::from_rgb(150, 150, 150);
        let widget_weak_color = egui::Color32::from_rgb(150, 150, 150);
        let bg_color = egui::Color32::from_rgb(175, 175, 175);
        let bg_strong_color = egui::Color32::from_rgb(230, 230, 230);
        let bg_weak_color = egui::Color32::from_rgb(150, 150, 150);
        let text_color = Some(egui::Color32::from_rgb(0, 0, 0));
        let mut egui_mq = std::mem::take(&mut self.egui_mq);
        egui_mq.ui(|_backend, egui_ctx| {
            let mut visuals = egui_ctx.style().visuals.clone();
            visuals.dark_mode = false;
            visuals.menu_rounding = 2.0.into();
            visuals.window_rounding = 2.0.into();
            visuals.widgets.noninteractive.bg_fill = bg_color;
            visuals.widgets.noninteractive.weak_bg_fill = bg_weak_color;
            visuals.widgets.active.bg_fill = widget_color;
            visuals.widgets.active.weak_bg_fill = widget_weak_color;
            visuals.widgets.inactive.bg_fill = widget_color;
            visuals.widgets.inactive.weak_bg_fill = widget_weak_color;
            visuals.widgets.hovered.bg_fill = widget_color;
            visuals.widgets.hovered.weak_bg_fill = widget_weak_color;
            visuals.faint_bg_color = bg_weak_color;
            visuals.extreme_bg_color = bg_strong_color;
            visuals.panel_fill = bg_color;
            visuals.window_fill = bg_color;
            visuals.override_text_color = text_color;
            egui_ctx.set_visuals(visuals);

            let mut palette_events = self.palette.update(egui_ctx);
            events.append(&mut palette_events);

            let mut toolbar_events = self.toolbar.update(egui_ctx, self.selected_tool);
            events.append(&mut toolbar_events);

            let mut layers_events = self.layers_panel.update(egui_ctx);
            events.append(&mut layers_events);

            let mut menu_events = self.menu.update(egui_ctx);
            events.append(&mut menu_events);

            self.preview.update(egui_ctx);
            self.status_bar.update(egui_ctx);

            let mut canvas_panel_events = self.update_canvas_panel(egui_ctx);
            events.append(&mut canvas_panel_events);

            if self.mouse_on_canvas {
                egui_ctx.output_mut(|o| o.cursor_icon = egui::CursorIcon::None);
            }
        });
        std::mem::swap(&mut self.egui_mq, &mut egui_mq);

        events
    }

    pub fn draw_preview(&self, state: &UiState) {
        self.preview.draw(state);
    }

    fn update_canvas_panel(&mut self, egui_ctx: &egui::Context) -> Vec<Effect> {
        let mut events = Vec::new();

        if egui_ctx.is_pointer_over_area() {
            events.push(Effect::UiEvent(UiEvent::MouseOverGui));
        }

        events
    }
}
