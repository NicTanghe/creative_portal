use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
};

use crate::prelude::{
    components::texts::*,
};


pub fn text_color_system(time: Res<Time>, mut query: Query<&mut TextColor, With<ColorText>>) {
    for mut text_color in &mut query {
        let seconds = time.elapsed_secs();
        text_color.0 = Color::srgb(
            seconds.sin() * 0.5 + 0.5,
            (seconds * 0.75).sin() * 0.5 + 0.5,
            (seconds * 0.5).sin() * 0.5 + 0.5,
        );
    }
}
//const FONT_SIZE: f32 = 20.;
const LINE_HEIGHT: f32 = 13.;

pub fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.2}");
            }
        }
    }
}

/// Updates the scroll position of scrollable nodes in response to mouse input
pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * LINE_HEIGHT,
                mouse_wheel_event.y * LINE_HEIGHT,
            ),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        //if keyboard_input.pressed(KeyCode::ControlLeft)
        //    || keyboard_input.pressed(KeyCode::ControlRight)
        //{
        //    std::mem::swap(&mut dx, &mut dy);
        //}

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}



use bevy::ecs::system::ParamSet;

// please ask for more info on this queries thing i dont fully understand it.

pub fn update_scrollbar_position(
    mut queries: ParamSet<(
        Query<(&ScrollableContainer, &ScrollPosition, &Node)>, // Read-only access
        Query<&mut Node, With<ScrollbarHandle>>, // Mutable access
    )>,
) {
    // Extract the scroll data first and store it in a variable
    let scroll_data = queries.p0().iter().next().map(|(scrollable, scroll_position, _)| {
        let max_scroll_height = scrollable.content_height;
        let scroll_percent = (scroll_position.offset_y / max_scroll_height).clamp(0.0, 1.0) * 100.0;
        scroll_percent
    });

    // Drop the first borrow before accessing `p1()`
    if let Some(scroll_percent) = scroll_data {
        for mut scrollbar_node in queries.p1().iter_mut() {
            println!("Updating scrollbar: mapped to {:.2}%", scroll_percent);

            scrollbar_node.top = Val::Percent(scroll_percent);

            println!("Scrollbar handle new top: {:?}", scrollbar_node.top);
        }
    } else {
        println!("Could not find a ScrollableContainer with a ScrollPosition.");
    }
}

pub fn handle_scrollbar_drag(
    mut commands: Commands,
    mut dragging: ResMut<DraggingHandle>,
    mut scroll_query: Query<(&mut ScrollPosition, &ScrollableContainer)>,
    mut handle_query: Query<(&mut Node, &GlobalTransform), With<ScrollbarHandle>>,
    mut cursor_events: EventReader<CursorMoved>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    
    // Get cursor position (needed for dragging calculations)
    if let Some(cursor_moved) = cursor_events.iter().last() {
        let cursor_y = cursor_moved.position.y;

        if dragging.active {
            // Only proceed if a handle is currently being dragged
            for (mut handle_node, handle_transform) in handle_query.iter_mut() {
                for (mut scroll_position, scrollable) in scroll_query.iter_mut() {
                    let max_scroll_height = scrollable.content_height;
                    let handle_top = handle_transform.translation().y;
                    let handle_height = handle_node.size().y;

                    // Calculate drag distance
                    let delta_y = cursor_y - dragging.last_cursor_y;
                    let new_handle_position = (handle_top + delta_y).clamp(0.0, window.height() - handle_height);

                    // Convert handle movement to scroll percentage
                    let scroll_percent = new_handle_position / (window.height() - handle_height);
                    scroll_position.offset_y = scroll_percent * max_scroll_height;

                    // Apply the new position to the handle
                    handle_node.top = Val::Percent(scroll_percent * 100.0);

                    dragging.last_cursor_y = cursor_y;
                }
            }
        }
    }

    // Start dragging when the user presses the left mouse button on the handle
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            for (_handle, handle_transform) in handle_query.iter() {
                let handle_y = handle_transform.translation().y;
                let handle_height = 20.0; // Adjust to your actual handle size

                // Check if the cursor is within the scrollbar handle bounds
                if cursor_position.y >= handle_y && cursor_position.y <= handle_y + handle_height {
                    dragging.active = true;
                    dragging.last_cursor_y = cursor_position.y;
                }
            }
        }
    }

    // Stop dragging when the mouse button is released
    if mouse_button_input.just_released(MouseButton::Left) {
        dragging.active = false;
    }
}


pub fn debug_input_log(
    mut query: Query<&mut TextSpan, With<DebugText>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut debug_text) = query.get_single_mut() {
        let mut log_entries = vec![];

        // Log Mouse Scroll Events
        for event in mouse_wheel_events.read() {
            log_entries.push(format!("Mouse Scroll: x={:.1}, y={:.1}", event.x, event.y));
        }

        // Log Key Presses
        for key in keyboard_input.get_just_pressed() {
            log_entries.push(format!("Key Pressed: {:?}", key));
        }

        // Update the UI text if we have new logs
        if !log_entries.is_empty() {
            **debug_text = log_entries.join("\n");
        }
    } else {
        warn!("DebugText entity not found! Make sure it was spawned.");
    }
}



