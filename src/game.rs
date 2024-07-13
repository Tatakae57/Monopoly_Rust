use crate::screen;
use ncurses::{box_, mvprintw, mvwprintw, newwin, wrefresh};

fn convert_to_str(content: u8) -> String {
    content.to_string()
}

//     Menu
fn draw_rectangle(
    title: &str,
    width: u8,
    height: u8,
    posx: u8,
    posy: u8,
    color: i16,
    draw_box: bool,
    text_posx: i32,
) {
    let window = newwin(height as i32, width as i32, posy as i32, posx as i32);
    let mut_window: &mut i8 = unsafe { &mut *window };
    screen::won(color, mut_window);
    screen::fill_window(color, mut_window);
    if draw_box {
        box_(window, 0, 0);
    }
    mvwprintw(window, 1, text_posx, title).unwrap();
    wrefresh(window);
    screen::woff(color, mut_window);
}

pub fn draw_menu(x: i32, current_option: u8) {
    // For buttons
    for i in 0..3 {
        let color: i16;
        let message: &str;
        let draw_box: bool;
        // If selected
        if current_option == i {
            color = 1;
            draw_box = true;
        }
        // If not
        else {
            color = 2;
            draw_box = false;
        }
        // Set message
        match i {
            0 => message = "Create server",
            1 => message = "Connect to server",
            2 => message = "Quit",
            _ => message = "Error",
        }
        // Draw button
        draw_rectangle(
            message,
            x as u8 / 2,
            3,
            (x as u8 / 2) / 2,
            3 + (i * 3),
            color,
            draw_box,
            (x as i32 / 2) / 2 - (message.len() as i32 / 2 + 1),
        );
    }
}

//     Server menu
pub fn server_menu(
    x: i32,
    current_option: u8,
    total_players: u8,
    nickname: &str,
    ip: &str,
    port: &str,
) {
    // Messages
    mvprintw(2, 1, "Nickname:").unwrap();
    mvprintw(5, 7, "IP:").unwrap();
    mvprintw(8, 5, "Port:").unwrap();
    mvprintw(11, 2, "Players:").unwrap();

    // Content
    for i in 0..4 {
        let message: &str;
        let convert: String;
        let color: i16;
        // Set message
        if i == 0 {
            message = nickname;
        } else if i == 1 {
            message = ip;
        } else if i == 2 {
            message = port;
        } else {
            convert = convert_to_str(total_players);
            message = &convert;
        }
        // Set color
        if i == current_option {
            color = 1;
        } else {
            color = 2;
        }
        draw_rectangle(message, x as u8 / 2, 3, 11, 1 + (i * 3), color, true, 1);
    }
}
