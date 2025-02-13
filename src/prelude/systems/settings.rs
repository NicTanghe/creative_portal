use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
};




pub fn change_scale_factor(
    mut windows: Query<&mut Window>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();
    let mut scale = Some(window.resolution.scale_factor());

    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        scale = Some(scale.unwrap() + 0.1);
        window.resolution.set_scale_factor_override(scale);
        let resolution = window.resolution.clone(); // Clone the resolution to modify safely
        window.resolution.set(resolution.physical_width() as f32, resolution.physical_height() as f32); // Force window update
        println!("Increased scale factor to: {}", scale.unwrap());
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        scale = Some((scale.unwrap() - 0.1).max(0.1)); // Prevent negative scale
        window.resolution.set_scale_factor_override(scale);
        let resolution = window.resolution.clone();
        window.resolution.set(resolution.physical_width() as f32, resolution.physical_height() as f32); // Force window update
        println!("Decreased scale factor to: {}", scale.unwrap());
    }
}

