use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Blended, Background::Img, Font, Image},
    lifecycle::{Asset, Window},
};

use crate::map::Palette;
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
    data.text
        .execute(|ui_text| {
            let mut y_offset = 1;
            for (text_type, text) in ui_text {
                window.draw(
                    &text.area().translate(Vector::new(762.0, 20 * y_offset)),
                    Img(&text),
                );
                match text_type.as_str() {
                    "pebbles" => {}
                    "turn" => {}
                    _ => {}
                }
                y_offset += 1;
            }

            Ok(())
        })
        .unwrap();
}
