use bevy::{
    prelude::Component,
    ecs::system::Resource,
};

use std::fs;

#[derive(Component)]
pub struct TextChunk {
    pub lines: Vec<String>, 
}

//maybe unpup the fields and exclusivly use getters.
#[derive(Resource)]
pub struct Chunks {
    pub chunks: Vec<TextChunk>,
    pub cursor: (usize, usize, usize), // (chunk_index, line_index, horizontal_position)
}

impl Chunks {
    /// Loads a text file and splits it into chunks of 7 lines
    pub fn new(file_path: &str) -> Self {
        let content = fs::read_to_string(file_path).expect("Failed to read file");
        let lines: Vec<String> = content.split('\n').map(String::from).collect();

        let chunks = lines
            .chunks(7)
            .map(|chunk| TextChunk {
                lines: chunk.to_vec(),
            })
            .collect();

        Self { chunks, cursor: (0, 0, 0) }
    }

    pub fn into_text(&self) -> String {
        self.chunks
            .iter()
            .flat_map(|chunk| chunk.lines.iter())
            .cloned()
            .collect::<Vec<String>>()
            .join("\n")
    }


    /// Returns a vector of (chunk_index, line_index) pairs
    pub fn line_indices(&self) -> Vec<(usize, usize)> {
        self.chunks
            .iter()
            .enumerate()
            .flat_map(|(chunk_idx, chunk)| {
                (0..chunk.lines.len()).map(move |line_idx| (chunk_idx, line_idx))
            })
            .collect()
    }

    /// Fetch a line using chunk and line index
    pub fn get_line(&self, chunk_idx: usize, line_idx: usize) -> Option<&str> {
        self.chunks.get(chunk_idx)?.lines.get(line_idx).map(String::as_str)
    }

    
    pub fn get_cursor_line_index(&self) -> (usize,usize) {
        (self.cursor.0,self.cursor.1)
    }

    /// Computes the absolute X position of the cursor in pixels
    pub fn get_cursor_x(&self, char_width: f32) -> f32 {
        let (_, _, hor) = self.cursor;
        hor as f32 * char_width
    }

    /// Computes the absolute Y position of the cursor in pixels
    pub fn get_cursor_y(&self, line_height: f32) -> f32 {
        let (chunk, line, _) = self.cursor;

        // Sum up all lines from previous chunks
        let mut linepos = 0;
        for i in 0..chunk {
            linepos += self.chunks[i].lines.len();
        }
        linepos += line; // Add the current line position within the chunk

        linepos as f32 * line_height
    }
    //sets cursor position
    pub fn set_cursor(&mut self, chunk: usize, line: usize, hor_pos: usize) {
        if chunk < self.chunks.len() {
            if line < self.chunks[chunk].lines.len() {
                let max_hor = self.chunks[chunk].lines[line].len();
                self.cursor = (chunk, line, hor_pos.min(max_hor));
            }
        }
    }

    pub fn move_up(&mut self) {
        let (mut chunk, mut line, hor_pos) = self.cursor;
        if line > 0 {
            line -= 1;
        } else if chunk > 0 {
            chunk -= 1;
            line = self.chunks[chunk].lines.len() - 1;
        }
        self.cursor = (chunk, line, hor_pos.min(self.chunks[chunk].lines[line].len()));
    }

    pub fn move_down(&mut self) {
        let (mut chunk, mut line, hor_pos) = self.cursor;
        if line + 1 < self.chunks[chunk].lines.len() {
            line += 1;
        } else if chunk + 1 < self.chunks.len() {
            chunk += 1;
            line = 0;
        }
        self.cursor = (chunk, line, hor_pos.min(self.chunks[chunk].lines[line].len()));
    }

    pub fn move_left(&mut self) {
        let (mut chunk, mut line, mut hor_pos) = self.cursor;
        if hor_pos > 0 {
            hor_pos -= 1;
        } else if line > 0 {
            line -= 1;
            hor_pos = self.chunks[chunk].lines[line].len();
        } else if chunk > 0 {
            chunk -= 1;
            line = self.chunks[chunk].lines.len() - 1;
            hor_pos = self.chunks[chunk].lines[line].len();
        }
        self.cursor = (chunk, line, hor_pos);
    }

    pub fn move_right(&mut self) {
        let (mut chunk, mut line, mut hor_pos) = self.cursor;
        let line_len = self.chunks[chunk].lines[line].len();
        if hor_pos < line_len {
            hor_pos += 1;
        } else if line + 1 < self.chunks[chunk].lines.len() {
            line += 1;
            hor_pos = 0;
        } else if chunk + 1 < self.chunks.len() {
            chunk += 1;
            line = 0;
            hor_pos = 0;
        }
        self.cursor = (chunk, line, hor_pos);
    }

}


