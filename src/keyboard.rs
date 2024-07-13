use ncurses::getch;

pub fn get_menu_entry(
    key: &mut i32,
    current_option: &mut u8,
    current_frame: &mut u8,
    exit: &mut bool,
) {
    *key = getch();
    // Up
    if *key == 259 || *key == 260 {
        if *current_option != 0 {
            *current_option -= 1;
        } else {
            *current_option = 2;
        }
    }
    // Down
    else if *key == 258 || *key == 261 {
        if *current_option != 2 {
            *current_option += 1;
        } else {
            *current_option = 0;
        }
    }
    // Enter
    else if *key == 10 {
        if *current_option == 2 {
            *exit = true;
        } else if *current_option == 0 {
            *current_frame = 1;
        } else {
            *current_frame = 2;
        }
        *current_option = 0;
    }
}

pub fn get_server_entry(key: &mut i32, current_frame: &mut u8) {
    *key = getch();
}
