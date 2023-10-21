use crate::bubble_graph::BubbleId;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CursorControl {
    pub input_mode: InputMode,
    /// Selected bubble
    pub selected: Option<BubbleId>,
}

#[derive(Default, Debug, Clone)]
pub enum InputMode {
    /// Pan view & select bubbles
    #[default]
    Travel,
    /// Edit the focused bubble
    Edit(BubbleId),
}

impl InputMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputMode::Travel => "travel",
            InputMode::Edit(_) => "edit",
        }
    }
}
