use crate::mouse::CursorType;
use lapix::Tool;

pub struct Resources;

impl Resources {
    pub fn cursor(cursor: CursorType) -> &'static [u8] {
        match cursor {
            CursorType::Tool(tool) => Self::tool_icon(tool),
            CursorType::Pan => include_bytes!("../res/cursor/pan.png"),
            CursorType::Cross => include_bytes!("../res/cursor/cross.png"),
        }
    }

    pub fn tool_icon(tool: Tool) -> &'static [u8] {
        match tool {
            Tool::Brush => include_bytes!("../res/icon/pencil.png"),
            Tool::Bucket => include_bytes!("../res/icon/bucket.png"),
            Tool::Eraser => include_bytes!("../res/icon/eraser.png"),
            Tool::Eyedropper => include_bytes!("../res/icon/eyedropper.png"),
            Tool::Line => include_bytes!("../res/icon/line.png"),
            Tool::Selection => include_bytes!("../res/icon/selection.png"),
            Tool::Move => include_bytes!("../res/icon/move.png"),
            Tool::Rectangle => include_bytes!("../res/icon/rectangle.png"),
            Tool::Ellipse => include_bytes!("../res/icon/ellipse.png"),
        }
    }
}
