extern crate termion;
extern crate term_cursor;

mod todo;

use todo::Todo;

fn main() {
    let mut todo = Todo::new();

    loop {
        todo.print_list();
        todo.get_input();
        todo.handle_input();
        todo.adapt_x();
    }
}

