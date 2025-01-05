pub mod bindings;
pub mod manager;
pub mod mapper;

pub use bindings::KeyBindings;
pub use mapper::InputMapper;

use egui_macroquad::macroquad::prelude as mq;
use lapix::Point;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MouseButton(mq::MouseButton);
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyboardKey(mq::KeyCode);
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyboardModifier {
    Shift,
    Control,
    Alt,
    Super,
}

impl From<mq::KeyCode> for KeyboardKey {
    fn from(value: mq::KeyCode) -> Self {
        Self(value)
    }
}

impl From<mq::MouseButton> for MouseButton {
    fn from(value: mq::MouseButton) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InputEvent {
    KeyPress(KeyboardKey),
    KeyDown(KeyboardKey),
    KeyRelease(KeyboardKey),
    KeyModifier(KeyboardModifier),
    MouseButtonPress(MouseButton),
    MouseButtonDown(MouseButton),
    MouseButtonRelease(MouseButton),
    MouseScrollUp,
    MouseScrollDown,
    MouseRealMove(Point<i32>),
    MouseCanvasMove(Point<i32>),
}
