extern crate termion;
extern crate term_cursor;

use std::io::{self, Write};

fn main() {
    let mut list: Vec<String> = vec![];

    let mut x: u16 = 0;

    loop {
        let term_size = match termion::terminal_size() {
            Ok(term_size) => term_size,
            Err(_) => panic!("Failed to retrieve terminal size"),
        };

        x = (term_size.0 as f32 / 2.5) as u16;

        print!("{}{}{}TODO{}\n",
            termion::clear::All, 
            termion::cursor::Goto(term_size.0 / 2 - 1, term_size.1 / 5), 
            termion::style::Bold,
            termion::style::Reset);

        // Print items.
        for (i, item) in list.iter().enumerate() {
            print!("{}{}   {}", 
                termion::cursor::Goto(*&x, term_cursor::get_pos().unwrap().1 as u16 + 1 as u16),
                i + 1,
                item);
        }
        println!(""); // Print an extra line.

        let input = get_input(term_size, &x, &list);

        handle_input(input, term_size, &x, &mut list);
    }
}

fn get_input(term_size: (u16, u16), x: &u16, list: &Vec<String>) -> String {
    let mut input = String::new();

    // Print the prompt.
    print!("{}> ", termion::cursor::Goto(*x, term_size.1 / 5 + *&list.len() as u16 + 3));

    // Get user input.
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input)
        .expect("Failed to read user input");

    input
}

fn handle_input(input: String, term_size: (u16, u16), x: &u16, list: &mut Vec<String>) {
    // Match the input against valid arguments.
    match &input.as_str()[..input.as_str().len()-1] { // Exclude the newline.
        "quit" | "exit" => exit(),
                  "add" => add(term_size, &x, list),
               "remove" => remove(term_size, &x, list),
                      _ => println!("invalid command..."),
    }
}

fn add(term_size: (u16, u16), x: &u16, list: &mut Vec<String>) {
    print!("{}Add item: ", termion::cursor::Goto(*x, term_size.1 / 5 + *&list.len() as u16 + 3));

    let mut input = String::new();

    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input)
        .expect("Failed to read user input");

    let input = String::from(&input.as_str()[..input.as_str().len()-1]);

    list.push(input);
}

fn remove(term_size: (u16, u16), x: &u16, list: &mut Vec<String>) {
    print!("{}Remove item: ", termion::cursor::Goto(*x, term_size.1 / 5 + *&list.len() as u16 + 3));

    let mut input = String::new();

    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input)
        .expect("Failed to read user input");

    let input = String::from(&input.as_str()[..input.as_str().len()-1]);

    let index = input.parse::<usize>();

    let index = match index {
        Ok(i) => i,
        Err(error) => panic!("Failed to parse user input: {}", error),
    };

    if index == 0 {
        println!("not a valid index...");
    } else {
        list.remove(index - 1);
    }
}

fn exit() {
    // Clear the screen and go to position (1, 1).
    print!("{}{}", 
        termion::clear::All, 
        termion::cursor::Goto(1, 1));

    // Exit without error.
    std::process::exit(0);
}

