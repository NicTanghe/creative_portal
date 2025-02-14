
use bevy::prelude::*;
use crate::prelude::{
    inserted_resources::resources::ChunkersR, // Ensure this path is correct
    components::{
        chunks::Chunks,
        texts::*,
    }
};



//ok best change how this works.
//
//we should get the cursor y by polling the selected line
//we should have a current line variable thats what they used in iced REMEMBER1 also thx 
//this should also make it way easyer to get a location when everything is formatted.
//
//


// make this just change what line has AmISelectedLine this might be slower but at this point f it
// then make it poll Am i selected to get the position
// Im not sure this is posssible as it starts with a querry.
// it might need 2 systems that chain with one proccing the other one.
pub fn cursor_movement_system_v2(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut chunkers: ResMut<ChunkersR>,
    mut queries: ParamSet<(
        Query<&mut Node, With<CursorIndicator>>, // Mutable access for updating cursor position
    )>,
) {
    let screenplay = &mut chunkers.screenplay; // Access screenplay from ChunkersR
    let mut moved = false; // Track if movement happened


    //just_pressed is once and pressed is contineusely
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        screenplay.move_up();
        moved = true;
    } 
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        screenplay.move_down();
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        screenplay.move_left();
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
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
pub fn cursor_movement_system_v3(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut chunkers: ResMut<ChunkersR>,
    mut q_cursor: ParamSet<(
        Query<&mut Node, With<CursorIndicator>>, // Mutable access for updating cursor position
    )>,
    mut q_linecoord: ParamSet<(
        Query<(&mut Node, &GlobalTransform, &LinePosition), Without<CursorIndicator>>, // Ensure no conflict with cursor
    )>,
) {
    let screenplay = &mut chunkers.screenplay; // Access screenplay from ChunkersR
    let mut moved = false; // Track if movement happened

    // Handle key press logic
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        screenplay.move_up();
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        screenplay.move_down();
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        screenplay.move_left();
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        screenplay.move_right();
        moved = true;
    }

    // If cursor moved, update the UI position
    if moved {
        let cursor_x = screenplay.get_cursor_x(7.0); // Adjust based on font size
        let cursor_y = screenplay.get_cursor_y(14.0); // Adjust based on line height

        let (chunkpos, linepos) = chunkers.screenplay.get_cursor_line_index();
        
        // Iterate over the line position query (excluding the cursor)
        for (_node, transform, line_position) in q_linecoord.p0().iter_mut() {
            // Compare chunkpos and linepos with LinePosition's chunk_idx and line_idx
            if chunkpos == line_position.chunk_idx && linepos == line_position.line_idx {
                // Store the transform for the matched line
                println!("UI Position of Line {:?}: {:?}", (chunkpos, linepos), transform.translation());
                println!("Cursor moved to: X = {:.2}, Y = {:.2}", cursor_x, transform.translation());

                // Now modify the cursor position
                let mut cursor_query = q_cursor.p0();
                for mut cursor_node in cursor_query.iter_mut() {
                    cursor_node.left = Val::Px(120.);
                    cursor_node.top = Val::Px(transform.translation().y+5.);
                }
            }
        }

            }
}
pub fn update_selected_line(
    mut query: Query<(&mut Node, &GlobalTransform, &LinePosition)>,
    chunkers: Res<ChunkersR>,
) {
    let (chunkpos, linepos) = chunkers.screenplay.get_cursor_line_index(); // Assuming this returns a (usize, usize, usize)

    // Iterate over the query results
    for (_node, transform, line_position) in query.iter_mut() {
        // Compare chunkpos and linepos with LinePosition's chunk_idx and line_idx
        if chunkpos == line_position.chunk_idx && linepos == line_position.line_idx {
            // Print the UI position (translation) of the matched node
            println!("UI Position of Line {:?}: {:?}", (chunkpos, linepos), transform.translation());
        }
    }
}

