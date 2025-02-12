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


/// Scrollbar Handle Marker
#[derive(Component)]
pub struct ScrollbarHandle;



#[derive(Component)]
pub struct ScrollableContent {
   pub max_scroll: f32, // Max scrollable height
   pub current_scroll: f32, // Current scroll offset
}

#[derive(Component)]
pub struct ScrollbarDragging(bool);
