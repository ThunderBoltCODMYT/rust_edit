// main.rs
/**
MIT License

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
mod editor;
mod events;
mod utils;

use crate::editor::Editor;
use crate::events::map_key;
use crate::utils::{default_filename, is_valid_filename};
use crossterm::{
    event::{Event, read},
    terminal::disable_raw_mode,
    terminal::enable_raw_mode,
};

use crossterm::event::KeyEventKind;
use std::env::args;
use std::io::Write;
use std::process::exit;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    let mut editor = Editor::new();

    if args.len() > 1 {
        if args[1].trim().is_empty() {
            eprintln!("Filename is empty!");
            exit(1);
        }

        let file = args[1].clone();

        // deref coercion
        if !is_valid_filename(&file) {
            eprintln!("Invalid filename: {}", file);
            exit(1);
        }

        editor.filename = Some(file.clone());

        if let Ok(contents) = std::fs::read_to_string(&file) {
            editor.buffer = contents.lines().map(String::from).collect();
        } else {
            editor.buffer = vec![String::new()];
        }
    } else {
        editor.filename = Some(default_filename());
        editor.buffer = vec![String::new()];
    }

    enable_raw_mode().unwrap();

    editor.render();

    loop {
        if editor.dirty {
            editor.render();
            editor.dirty = false;
        }

        if let Event::Key(key_event) = read().unwrap() {
            if key_event.kind != KeyEventKind::Press {
                continue;
            }

            if let Some(event) = map_key(key_event) {
                if !editor.handle_event(event) {
                    print!("\x1b[2J\x1b[H");
                    std::io::stdout().flush().unwrap();
                    break;
                }
            }
        }
    }

    disable_raw_mode().unwrap();
    Ok(())
}
