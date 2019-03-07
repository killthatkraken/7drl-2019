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
    text: Asset<HashMap<String, Image>>,
}

impl UIData {
    pub fn new_game(text: Asset<HashMap<String, Image>>) -> UIData {
        UIData {
            turn: 0,
            pebbles: 0,
            text,
        }
    }
}

const MAP_R_BORDER: f32 = 696.0;
const MAP_B_BORDER: f32 = 480.0;

pub fn draw_ui(window: &mut Window, data: &mut UIData, tileset: &mut HashMap<char, Image>) {
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
    data.text
        .execute(|ui_text| {
            for (text_type, text) in ui_text {
                match text_type.as_str() {
                    "pebbles" => {
                        window.draw(
                            &text.area().translate(Vector::new(
                                TILE_SIZE.x as f32,
                                MAP_B_BORDER + TILE_SIZE.y * 2.0,
                            )),
                            Img(&text),
                        );

                        let pebble_ui = &tileset.get(&'o').unwrap();
                        for n in 1..=pebbles {
                            window.draw(
                                &Rectangle::new(
                                    Vector::new(
                                        text.area().size.x + (12 * n) as f32,
                                        MAP_B_BORDER + TILE_SIZE.y * 2.0,
                                    ),
                                    pebble_ui.area().size(),
                                ),
                                Blended(&pebble_ui, Palette::WHITE),
                            );
                        }
                    }
                    "turn" => {}
                    _ => {}
                }
            }

            Ok(())
        })
        .unwrap();
}
