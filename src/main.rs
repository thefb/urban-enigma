use ncurses::*;
use std::cmp::*;
const REGULAR_PAIR: i16 = 0;
const HIGHLIGHTED_PAIR: i16 = 1;

fn main() {
    initscr();

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHTED_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let todos = vec!["Buy a bread", "Make a cup of tea", "Write a todo app"];
    let mut todo_current: usize = 0;

    while !quit {
        for (index, todo) in todos.iter().enumerate() {
            let pair = {
                if todo_current == index {
                    HIGHLIGHTED_PAIR
                } else {
                    REGULAR_PAIR
                }
            };
            attron(COLOR_PAIR(pair));
            mv(index as i32, 0);
            addstr(*todo);
            attroff(COLOR_PAIR(pair));
        }

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' => {
                if todo_current > 0 {
                    todo_current -= 1
                }
            }
            's' => todo_current = min(todo_current + 1, todos.len() - 1),
            _ => {}
        }
    }

    endwin();
}
