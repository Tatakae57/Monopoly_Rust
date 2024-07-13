use ncurses::{
    attroff, attron, curs_set, init_pair, initscr, keypad, noecho, start_color, stdscr, wattroff,
    wattron, wbkgd, COLOR_BLACK, COLOR_BLUE, COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_PAIR,
    COLOR_RED, COLOR_WHITE, COLOR_YELLOW, CURSOR_VISIBILITY,
};

//   Colors
pub fn on(color: i16) {
    attron(COLOR_PAIR(color));
}

pub fn off(color: i16) {
    attroff(COLOR_PAIR(color));
}

pub fn won(color: i16, window: &mut i8) {
    wattron(window, COLOR_PAIR(color));
}

pub fn woff(color: i16, window: &mut i8) {
    wattroff(window, COLOR_PAIR(color));
}

pub fn fill_window(color: i16, window: &mut i8) {
    wbkgd(window, COLOR_PAIR(color));
}

pub fn colors() {
    start_color();
    //   Menu
    // Selected
    init_pair(1, COLOR_BLACK, COLOR_WHITE);
    // Unselected
    init_pair(2, COLOR_WHITE, COLOR_BLACK);

    //   Table
    // Start, Lucky, Jail and Auction
    init_pair(15, COLOR_RED, COLOR_WHITE);
    // Section 1
    init_pair(3, COLOR_WHITE, COLOR_GREEN);
    // Section 2
    init_pair(4, COLOR_WHITE, COLOR_BLUE);
    // Section 3
    init_pair(5, COLOR_WHITE, COLOR_MAGENTA);
    // Section 4
    init_pair(6, COLOR_WHITE, COLOR_YELLOW);
    // Section 5
    init_pair(7, COLOR_WHITE, COLOR_CYAN);
    // Section 6
    init_pair(8, COLOR_WHITE, COLOR_BLACK);
    // Section 7
    init_pair(9, COLOR_BLACK, COLOR_WHITE);
    // Section 8
    init_pair(10, COLOR_WHITE, COLOR_RED);

    //   Players
    // Player 1
    init_pair(11, COLOR_YELLOW, COLOR_BLUE);
    // Player 2
    init_pair(12, COLOR_CYAN, COLOR_BLACK);
    // Player 3
    init_pair(13, COLOR_GREEN, COLOR_RED);
    // Player 4
    init_pair(14, COLOR_WHITE, COLOR_MAGENTA);
}

//   Screen
pub fn init_screen() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
    keypad(stdscr(), true);
}
