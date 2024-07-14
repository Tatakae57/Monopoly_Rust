use ncurses::{endwin, erase, getmaxyx, refresh, stdscr};
mod files;
mod game;
mod keyboard;
mod screen;

// Structs
struct Player {
    nickname: String,
    position: u8,
    money: u16,
}

struct Terrain {
    name: String,
    owned: bool,
    owner: String,
    cost: u16,
    upgrade_cost: u16,
    upgrades: u8,
    posx: i16,
    posy: i16,
    color: i16,
}

// Main
fn main() {
    // Init screen
    screen::init_screen();
    screen::colors();

    // Variables
    let mut exit: bool = false;
    let (mut x, mut y, mut key): (i32, i32, i32) = (0, 0, 0);
    let (mut current_frame, mut current_option, mut total_players): (u8, u8, u8) = (3, 0, 3);
    let (mut ip, mut port, mut nickname, mut message_box) =
        (String::new(), String::new(), String::new(), String::new());

    // Load config
    //files::load_config(&mut total_players, &mut ip, &mut port, &mut nickname);

    // Create terrains
    let mut terrains: Vec<Terrain> = game::set_terrains(x, y);

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
            game::server_menu(x, current_option, total_players, &nickname, &ip, &port);
            keyboard::get_server_entry(&mut key, &mut current_frame);
        } else if current_frame == 3 {
            game::draw_game(&terrains);
            keyboard::get_game_entry(&mut key, &mut current_frame, &mut message_box);
        }
    }
    endwin();
}
