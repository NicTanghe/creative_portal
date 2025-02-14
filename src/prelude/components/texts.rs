use bevy::prelude::Component;

//these just function as markers so that the thing knows ooh yes i got to work on this
#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ColorText;

#[derive(Component)]
pub struct DebugText;

#[derive(Component)]
pub struct ScrollableText; // Marker for the scrolling text container


#[derive(Component)]
pub struct AmISelectedLine{
    selected: bool,
}

// this one is to tag the entity so that we can match it the line that`s set as a compnent on the
// Chukers 
#[derive(Component)]
pub struct LinePosition{
    pub chunk_idx: usize,
    pub line_idx: usize,
}

#[derive(Component)]
pub struct ScrollableContent {
   pub max_scroll: f32, // Max scrollable height
   pub current_scroll: f32, // Current scroll offset
}

/// Scrollbar Handle Marker
#[derive(Component)]
pub struct ScrollbarHandle;


#[derive(Component)]
pub struct ScrollableContainer {  // Range: 0.0 (top) to 1.0 (bottom)
  pub  content_height: f32,
}


#[derive(Component, Default)]
struct ScrollPosition {
    offset_y: f32,
}

//#[derive(Resource, Default)]
//struct DraggingHandle {
//    active: bool,
//    last_cursor_y: f32,
//}

#[derive(Component, Default)]
pub struct CursorIndicator; 
