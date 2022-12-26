use crate::Screen;
use crate::vga_buf::AsciiChar;

const MAP: [&str; 25] = [
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                    x                                           ",
    "                                  x x                                           ",
    "                        xx      xx            xx                                ",
    "                       x   x    xx            xx                                ",
    "            xx        x     x   xx                                              ",
    "            xx        x   x xx    x x                                           ",
    "                      x     x       x                                           ",
    "                       x   x                                                    ",
    "                        xx                                                      ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                "
];

pub fn game_of_life(screen: &mut Screen) {
    let mut current_gen: [[u8; 80]; 25] = [[0; 80]; 25];
    for i in 0..MAP.len() {
        for (j, byte) in MAP[i].bytes().enumerate() {
            current_gen[i][j] = byte;
        }
    }

    // TODO: implement game of life
}
