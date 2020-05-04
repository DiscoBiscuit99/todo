# Terminal based todo app written in Rust

Just a minimal todo app (still under 200 lines) quickly thrown together using the language Rust and the crates [termion](https://github.com/redox-os/termion) and [term_cursor](https://github.com/Lisoph/term_cursor).

It isn't done yet, but most of the functionality is implemented. 

### Features

Nothing really, except the following:

+ Somewhat adapts to the terminal size.
+ Saves the todo list in a file called `todo_list`.

### Usage

The program is still quite bare bones. Still, using it is quite simple. To add an item to the list, one types simply `add` and the user is then prompted to type their item, ending the prompt with a carriage return. To remove an item, one types `remove` and the user is similarly prompted to type the index (starting from one), again ending the prompt with a carriage return. To quit the program while saving the list, type `quit`. To quit the program without saving, if one so desires, one can hit `ctrl+C`, `ctrl+Z`, `ctrl+D` or similar commands that kill a running program.

### Still some things to do

Including:

+ [ ] React immediately on terminal resize (and adapt accordingly).
+ [ ] Implement the ability to type just `add <something>` to add something (same for remove).
+ [ ] Clean up bits of code.
+ [ ] Add some nice colors (?)

