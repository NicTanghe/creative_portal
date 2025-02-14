use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
};

// note that instead of doin all the keybinds in their own systems.
// there should be 1 keybind that checks buttons pressed and calls other functions to excecute


use bevy::prelude::*;
use bevy::window::WindowResized;

pub fn change_scale_factor(
    mut windows: Query<(Entity, &mut Window)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut resize_events: EventWriter<WindowResized>,
) {
    let (window_entity, mut window) = windows.single_mut();
    let mut resolution = window.resolution.clone();
    let mut scale = resolution.scale_factor_override().unwrap_or(resolution.scale_factor());

    // Check if Ctrl is held down
    let ctrl_held = keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight);

    if keyboard_input.pressed(KeyCode::Equal) {
        scale += 0.1;
        resolution.set_scale_factor_override(Some(scale));
        window.resolution = resolution.clone();
        resize_events.send(WindowResized {
            window: window_entity, // Use entity ID instead
            width: resolution.physical_width() as f32,
            height: resolution.physical_height() as f32,
        });
        println!("Increased scale factor to: {}", scale);
    }

    if keyboard_input.just_pressed(KeyCode::Minus) {
        scale = (scale - 0.1).max(0.1); // Prevent negative scale
        resolution.set_scale_factor_override(Some(scale));
        window.resolution = resolution.clone();
        resize_events.send(WindowResized {
            window: window_entity, // Use entity ID instead
            width: resolution.physical_width() as f32,
            height: resolution.physical_height() as f32,
        });
        println!("Decreased scale factor to: {}", scale);
    }
}

