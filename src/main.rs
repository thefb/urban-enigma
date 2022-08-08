use ncurses::*;

fn main() {
    initscr();
    addstr("Hello World");
    refresh();
    let mut quit = false;
    let mut todos = vec![
        "Buy a bread", 
        "Make a cup of tea"
        "Write a todo app", 
    ];

    while !quit {
        let key = getch();
        for todo in todos.iter() {
            addstr(*todo);
        }
        match key as u8 as char {
            'q' => quit = true,
            _ => {}
        }
    }
    endwin();
}
