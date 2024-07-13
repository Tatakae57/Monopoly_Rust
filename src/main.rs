use ncurses::{endwin, erase, getmaxyx, refresh, stdscr};
mod game;
mod keyboard;
mod screen;

fn main() {
    // Init screen
    screen::init_screen();
    screen::colors();

    // Variables
    let mut exit: bool = false;
    let (mut x, mut y, mut key): (i32, i32, i32) = (0, 0, 0);
    let (mut current_frame, mut current_option, mut total_players): (u8, u8, u8) = (0, 0, 3);

    // Game Loop
    while !exit {
        getmaxyx(stdscr(), &mut y, &mut x);
        erase();
        refresh();

        // Frames
        if current_frame == 0 {
            game::draw_menu(x, current_option);
            keyboard::get_menu_entry(&mut key, &mut current_option, &mut current_frame, &mut exit);
        } else if current_frame == 1 {
            game::server_menu(
                x,
                current_option,
                total_players,
                "Kae",
                "192.168.1.36",
                "8000",
            );
            keyboard::get_server_entry(&mut key, &mut current_frame);
        }
    }
    endwin();
}
