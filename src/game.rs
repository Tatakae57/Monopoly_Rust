use crate::screen;
use crate::Terrain;
use ncurses::{box_, mvprintw, mvwprintw, newwin, wrefresh};

fn convert_to_str(content: u8) -> String {
    content.to_string()
}

//     Menu
fn draw_rectangle(
    title: &str,
    width: u8,
    height: u8,
    posx: i16,
    posy: i16,
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
            (x as i16 / 2) / 2,
            3 + (i as i16 * 3),
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
        draw_rectangle(
            message,
            x as u8 / 2,
            3,
            11,
            1 + (i as i16 * 3),
            color,
            true,
            1,
        );
    }
}

// Screen game
pub fn draw_game(terrains: &Vec<Terrain>) {
    for i in 0..27 {
        draw_rectangle(
            &terrains[i].name,
            7,
            4,
            terrains[i].posx,
            terrains[i].posy,
            terrains[i].color,
            true,
            1,
        );
    }
}

// Players and Terrains
pub fn set_terrains(x: i32, y: i32) -> Vec<Terrain> {
    let mut terrains: Vec<Terrain> = vec![];
    let start_cost: u8 = 50;
    let terrain_names: [&str; 28] = [
        "Base", "Merc.", "Venus", "Stat.", "Earth", "Moon", "Mars", "Lucky", "Deim.", "Phob.",
        "Mine", "Jupi.", "Gany.", "Satu.", "Jail", "Titan", "Uran.", "Sun", "Ariel", "Nept.",
        "Trit.", "Auct.", "Ceres", "Pluto", "Stars", "Haum.", "Make.", "Eris",
    ];
    let colors: [i16; 28] = [
        15, 3, 3, 16, 4, 4, 4, 15, 5, 5, 16, 6, 6, 6, 15, 7, 7, 15, 8, 8, 8, 16, 9, 9, 15, 10, 10,
        10,
    ];

    for i in 0..27 {
        let mut positions: (i16, i16) = (0, 0);
        if i <= 7 {
            positions.0 = (x / 2 / 2) as i16;
            positions.1 = (y + (y / 2 / 2) + (i * 4)) as i16;
        } else if i <= 14 {
            positions.0 = (x / 2 / 2) as i16 + (i as i16 * 7);
            positions.1 = (y / 2 / 2) as i16;
        } else if i <= 21 {
            positions.0 = (x - (x / 2 / 2)) as i16;
            positions.1 = ((y / 2 / 2) + (i * 4)) as i16;
        } else {
            positions.0 = (x + (x / 2 / 2) + (i * 7)) as i16;
            positions.1 = (y + (y / 2 / 2)) as i16;
        }
        terrains.push(Terrain {
            name: String::from(terrain_names[i as usize]),
            owned: false,
            owner: String::from(""),
            cost: start_cost as u16 * i as u16,
            upgrade_cost: 10,
            upgrades: 0,
            posx: positions.0,
            posy: positions.1,
            color: colors[i as usize],
        });
    }
    return terrains;
}
