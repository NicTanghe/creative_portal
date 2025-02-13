
use bevy::prelude::*;
use crate::prelude::{
    inserted_resources::resources::ChunkersR, // Ensure this path is correct
    components::{
        chunks::Chunks,
        texts::*,
    }
};

pub fn cursor_movement_system_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut chunkers: ResMut<ChunkersR>,
) {
    let screenplay = &mut chunkers.screenplay; // Access screenplay

    let mut moved = false; // Track if movement happened

    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        screenplay.move_up();
        moved = true;
    } 
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        screenplay.move_down();
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        screenplay.move_left();
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        screenplay.move_right();
        moved = true;
    }

    // Print cursor position only if movement occurred
    if moved {
        let cursor_x = screenplay.get_cursor_x(12.0); // Adjust based on your font size
        let cursor_y = screenplay.get_cursor_y(21.0); // Adjust based on your line height
        println!("Cursor moved to: X = {:.2}, Y = {:.2}", cursor_x, cursor_y);
    }
}



pub fn cursor_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut chunkers: ResMut<ChunkersR>,
    mut queries: ParamSet<(
        Query<&mut Node, With<CursorIndicator>>, // Mutable access for updating cursor position
    )>,
) {
    let screenplay = &mut chunkers.screenplay; // Access screenplay from ChunkersR
    let mut moved = false; // Track if movement happened

    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        screenplay.move_up();
        moved = true;
    } 
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        screenplay.move_down();
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        screenplay.move_left();
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        screenplay.move_right();
        moved = true;
    }

    // If cursor moved, update the UI position
    // this is ok for now but sadly not handelling line breaking
    if moved {
        
        let cursor_x = screenplay.get_cursor_x(7.0); // Adjust based on font size
        let cursor_y = screenplay.get_cursor_y(14.0); // Adjust based on line height

        // Drop the first borrow before modifying the cursor position
        let mut cursor_query = queries.p0();
        for mut cursor_node in cursor_query.iter_mut() {
            cursor_node.left = Val::Px(cursor_x);
            cursor_node.top = Val::Px(cursor_y);
        }

        println!("Cursor moved to: X = {:.2}, Y = {:.2}", cursor_x, cursor_y);
    }
}

