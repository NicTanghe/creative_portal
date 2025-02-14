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
const LINE_HEIGHT: f32 = 14.5;

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
///
/// add some formatting that fades in and out cutoff text.
///
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



// note that every button press something is pprinted
pub fn update_scrollbar_position(
    mut queries: ParamSet<(
        Query<(&ScrollableContainer, &ScrollPosition, &Node)>, // Read-only access
        Query<&mut Node, With<ScrollbarHandle>>, // Mutable access
    )>,
) {
    // Extract the scroll data first and store it in a variable
    let scroll_data = queries.p0().iter().next().map(|(scrollable, scroll_position, _)| {
        let max_scroll_height = scrollable.content_height;

        //either scroll position offset is behaving strangly with height or something else is fucked
        let scroll_percent = (scroll_position.offset_y / max_scroll_height).clamp(0.0, 1.0) * 100.0;
        scroll_percent
    });

    // Drop the first borrow before accessing `p1()`
    if let Some(scroll_percent) = scroll_data {
        for mut scrollbar_node in queries.p1().iter_mut() {
            //println!("Updating scrollbar: mapped to {:.2}%", scroll_percent);

            scrollbar_node.top = Val::Percent(scroll_percent);

            //println!("Scrollbar handle new top: {:?}", scrollbar_node.top);
        }
    } else {
        //println!("Could not find a ScrollableContainer with a ScrollPosition.");
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



