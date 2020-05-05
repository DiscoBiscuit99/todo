extern crate termion;
extern crate term_cursor;

use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;

pub struct Todo {
    term_size: (u16, u16),
    list: Vec<String>,
    input: String,
    x: u16,
    y_start: u16,
}

impl Todo {
    pub fn new() -> Todo {
        let term_width = termion::terminal_size().unwrap().0;

        let mut todo = Todo {
            term_size: termion::terminal_size().unwrap(),
            list: vec![],
            input: String::new(),
            x: (term_width as f32 / 2.75) as u16,
            y_start: termion::terminal_size().unwrap().1 / 5,
        };

        todo.list = Todo::read_saved_list();

        if term_width <= 105 {
            todo.x = term_width / 3;
        } else if term_width <= 145 {
            todo.x = term_width / 4;
        } else if term_width <= 185 {
            todo.x = term_width / 5;
        }

        todo
    }

    pub fn get_input(&mut self) {
        // Print the prompt.
        print!("{}> ", 
            termion::cursor::Goto(self.x, 
                self.y_start + self.list.len() as u16 + 3));

        // Get user input.
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut self.input)
            .expect("Failed to read user input");
    }

    pub fn handle_input(&mut self) {
        // Match the input against valid arguments.
        match &self.input.as_str()[..self.input.as_str().len()-1] { // Exclude the newline.
            "quit" | "q" | "exit" => self.exit(),
            "add" =>                 self.add(),
            "remove" =>              self.remove(),
            _ =>                     (),
        }

        self.input = String::new();
    }

    pub fn add(&mut self) {
        print!("{}Add item: ", 
            termion::cursor::Goto(self.x, 
                self.y_start + self.list.len() as u16 + 3));

        let mut tmp_input = String::new();

        let _ = io::stdout().flush();
        io::stdin().read_line(&mut tmp_input)
            .expect("Failed to read user input");

        tmp_input = 
            String::from(&tmp_input.as_str()[..&tmp_input.as_str().len()-1]);

        self.list.push(tmp_input);
    }

    pub fn remove(&mut self) {
        print!("{}Remove item: ", 
            termion::cursor::Goto(self.x, 
                self.y_start + self.list.len() as u16 + 3));

        let mut tmp_input = String::new();

        let _ = io::stdout().flush();
        io::stdin().read_line(&mut tmp_input)
            .expect("Failed to read user input");

        tmp_input = 
            String::from(&tmp_input.as_str()[..&tmp_input.as_str().len()-1]);

        let index = tmp_input.parse::<usize>();
        let index = match index {
            Ok(i) => i,
            Err(error) => panic!("Failed to parse user input: {}", error),
        };

        if index == 0 {
            println!("not a valid index...");
        } else {
            self.list.remove(index - 1);
        }
    }

    pub fn print_list(&mut self) {
        print!("{}{}{}TODO{}\n",
            termion::clear::All,
            termion::cursor::Goto(
                self.term_size.0 / 2 - 1, 
                self.term_size.1 / 5),
            termion::style::Bold,
            termion::style::Reset);

        let mut y_pos = term_cursor::get_pos().unwrap().1 as u16 + 1;

        // Print items.
        for (i, mut item) in self.list.iter().enumerate() {
            // Align after the first digit.
            if (i + 1) % 10 == 0 {
                self.x -= 1;
            }

            print!("{}{}   {}",
                termion::cursor::Goto(self.x, y_pos),
                i + 1,
                item);

            y_pos += 1;
        }
        println!(""); // Print an extra line.
    }

    pub fn adapt_x(&mut self) {
        self.term_size = termion::terminal_size().unwrap();

        let term_width = self.term_size.0;

        if term_width <= 105 {
            self.x = term_width / 3;
        } else if term_width <= 145 {
            self.x = term_width / 4;
        } else if term_width <= 185 {
            self.x = term_width / 5;
        }
    }

    fn exit(&self) {
        self.save_list();

        // Clear the terminal and go to position (1, 1).
        print!("{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1));

        // Exit without error.
        std::process::exit(0);
    }

    fn save_list(&self) {
        let _ = File::create("todo_list");

        let save_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("todo_list")
            .unwrap();
        
        for item in &self.list {
            writeln!(&save_file, "{}", item).expect("Failed to save todo list");
        }
    }

    fn read_saved_list() -> Vec<String> {
        let mut todo_list: Vec<String> = vec![];

        // If the save file exists, read it and return a vector of strings.
        if Path::new("todo_list").exists() {
            let save_file_contents = std::fs::read_to_string("todo_list")
                .expect("Failed to read savefile (todo_list)");

            let mut tmp_list: Vec<String> = 
                save_file_contents.as_str()
                    .split("\n").map(|s| s.to_string())
                    .collect();

            tmp_list.remove(tmp_list.len()-1);

            for item in tmp_list {
                todo_list.push(String::from(item));
            }
        } 

        todo_list
    }
}
