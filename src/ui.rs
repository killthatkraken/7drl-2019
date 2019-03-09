use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Blended, Background::Img, Image},
    lifecycle::{Asset, Window},
};

use crate::map::{Palette, TILE_SIZE};
use std::collections::HashMap;

pub struct UIData {
    pub turn: u32,
    pub pebbles: u32,
    text: Asset<HashMap<&'static str, Image>>,
}

impl UIData {
    pub fn new(text: Asset<HashMap<&'static str, Image>>) -> UIData {
        UIData {
            turn: 0,
            pebbles: 0,
            text,
        }
    }
}

pub struct MessageLog {
    pub log: Vec<&'static str>,
    show: usize,
}

impl MessageLog {
    pub fn new() -> MessageLog {
        MessageLog {
            log: vec!["dark"],
            show: 15,
        }
    }
    pub fn push(&mut self, message: &'static str) {
        self.log.push(message);
        if self.log.len() > self.show {
            self.log.remove(0);
        }
    }
}

const MAP_R_BORDER: i32 = 696;
const MAP_B_BORDER: i32 = 480;

pub fn draw_ui(
    window: &mut Window,
    data: &mut UIData,
    message_log: &mut MessageLog,
    tileset: &mut HashMap<char, Image>,
) {
    //Borders
    let v_border = tileset.get(&'|').unwrap();
    for y in 0..=760 {
        window.draw(
            &Rectangle::new(Vector::new(MAP_R_BORDER, y), v_border.area().size()),
            Blended(&v_border, Palette::LIGHT_BLUE),
        );
    }

    let h_border = tileset.get(&'_').unwrap();
    for x in 0..=684 {
        window.draw(
            &Rectangle::new(Vector::new(x, MAP_B_BORDER), h_border.area().size()),
            Blended(&h_border, Palette::LIGHT_BLUE),
        );
    }

    //Text
    let pebbles = data.pebbles;
    let mut y_offset = 2.0;
    data.text
        .execute(|ui_text| {
            for (text_type, text) in ui_text.iter() {
                match *text_type {
                    "pebbles" => {
                        window.draw(
                            &text.area().translate(Vector::new(
                                TILE_SIZE.x as f32,
                                MAP_B_BORDER as f32 + TILE_SIZE.y * y_offset,
                            )),
                            Img(&text),
                        );

                        let pebble_ui = &tileset.get(&'o').unwrap();
                        for n in 1..=pebbles {
                            window.draw(
                                &Rectangle::new(
                                    Vector::new(
                                        text.area().size.x + (12 * n) as f32,
                                        MAP_B_BORDER as f32 + TILE_SIZE.y * y_offset,
                                    ),
                                    pebble_ui.area().size(),
                                ),
                                Blended(&pebble_ui, Palette::WHITE),
                            );
                        }
                        y_offset += 2.0;
                    }
                    "message_log" => {
                        window.draw(
                            &text.area().translate(Vector::new(
                                TILE_SIZE.x as f32,
                                MAP_B_BORDER as f32 + TILE_SIZE.y * y_offset,
                            )),
                            Img(&text),
                        );
                        y_offset += 2.0;
                    }
                    _ => {}
                }
            }

            let mut log_offset = y_offset + 2.0;
            message_log
                .log
                .iter()
                .enumerate()
                .rev()
                .for_each(|(_i, message)| {
                    let text = ui_text.get(message).unwrap();
                    window.draw(
                        &text.area().translate(Vector::new(
                            TILE_SIZE.x as f32,
                            MAP_B_BORDER as f32 + 10.0 * log_offset,
                        )),
                        Img(&text),
                    );
                    log_offset += 1.0;
                });

            Ok(())
        })
        .unwrap();
}
