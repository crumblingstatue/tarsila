use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Drawing and manipulation tools available in the image editor
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tool {
    Brush,
    Eraser,
    Eyedropper,
    Bucket,
    Line,
    Selection,
    Move,
    Rectangle,
    Ellipse,
}

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let st = match self {
            Self::Brush => "brush",
            Self::Eraser => "eraser",
            Self::Eyedropper => "eyedropper",
            Self::Bucket => "bucket",
            Self::Line => "line",
            Self::Selection => "selection",
            Self::Move => "move",
            Self::Rectangle => "rectangle",
            Self::Ellipse => "ellipse",
        };

        f.write_str(st)
    }
}
