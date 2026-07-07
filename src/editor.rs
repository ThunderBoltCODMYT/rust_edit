// editor.rs:
/**
* MIT License

Copyright (c) 2026 ThunderBoltCODMYT

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
use crate::events::EditorEvent;
use crate::utils::default_filename;
use crossterm::terminal::size;
use std::io::{Write, stdout};
use std::vec;

const LINE_NUMBER_WIDTH: usize = 4;

pub struct Editor {
    pub buffer: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub viewport_y: usize,
    pub filename: Option<String>,
    pub dirty: bool,
}

impl Editor {
    pub fn new() -> Self {
        return Self {
            buffer: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            viewport_y: 0,
            filename: None,
            dirty: false,
        };
    }

    pub fn save(&mut self) -> bool {
        let name = self.filename.get_or_insert_with(default_filename);

        let text = self.buffer.join("\n");

        std::fs::write(name, text).is_ok()
    }

    pub fn insert_char(&mut self, c: char) {
        self.buffer[self.cursor_y].insert(self.cursor_x, c);
        self.cursor_x += 1;
        self.dirty = true;
    }

    pub fn backspace(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            self.buffer[self.cursor_y].remove(self.cursor_x);
            self.dirty = true;
        } else if self.cursor_y > 0 {
            let current = self.buffer.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.buffer[self.cursor_y].len();
            self.buffer[self.cursor_y].push_str(&current);
            self.dirty = true;
        }
    }

    pub fn insert_newline(&mut self) {
        let current = &mut self.buffer[self.cursor_y];
        let new = current.split_off(self.cursor_x);

        self.buffer.insert(self.cursor_y + 1, new);
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.dirty = true;
    }

    pub fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            self.dirty = true;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor_x < self.buffer[self.cursor_y].len() {
            self.cursor_x += 1;
            self.dirty = true;
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;

            if self.cursor_y < self.viewport_y {
                self.viewport_y -= 1;
            }

            self.cursor_x = self.cursor_x.min(self.buffer[self.cursor_y].len());
            self.dirty = true;
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor_y + 1 < self.buffer.len() {
            let (_, term_height) = size().unwrap_or((0, 20));
            let screen_height = (term_height as usize).saturating_sub(2);

            if self.cursor_y >= self.viewport_y + screen_height {
                self.viewport_y += 1;
            }

            self.cursor_x = self.cursor_x.min(self.buffer[self.cursor_y].len());
            self.dirty = true;
        }
    }

    pub fn render(&self) {
        let mut out = stdout();
        print!("\x1b[2J\x1b[H");
        let (_, term_height) = size().unwrap();
        let screen_height = (term_height as usize).saturating_sub(2);
        let start = self.viewport_y;
        let end = (start + screen_height).min(self.buffer.len());

        for (i, line) in self.buffer[start..end].iter().enumerate() {
            let screen_y = i + 1;

            let line_no = start + i + 1;

            print!("\x1b[{};1H{:>4} {}", screen_y, line_no, line);
        }

        let status_y = screen_height + 1;

        print!(
            "\x1b[{};1H\x1b[7m Ln {} Col {} | Lines {} \x1b[0m",
            status_y,
            self.cursor_y + 1,
            self.cursor_x + 1,
            self.buffer.len()
        );

        let screen_y = (self.cursor_y - self.viewport_y) + 1;
        let screen_x = self.cursor_x + LINE_NUMBER_WIDTH + 2;

        print!("\x1b[{};{}H", screen_y, screen_x);
        out.flush().unwrap();
    }

    pub fn handle_event(&mut self, event: EditorEvent) -> bool {
        match event {
            EditorEvent::Insert(c) => self.insert_char(c),
            EditorEvent::Backspace => self.backspace(),
            EditorEvent::Enter => self.insert_newline(),
            EditorEvent::MoveLeft => self.move_left(),
            EditorEvent::MoveRight => self.move_right(),
            EditorEvent::MoveUp => self.move_up(),
            EditorEvent::MoveDown => self.move_down(),
            EditorEvent::Save => return self.save(),
            EditorEvent::Quit => return false,
        }
        return true;
    }
}

