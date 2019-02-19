use crate::map::{Palette, TILE_SIZE};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::{Col, Blended}, Color, Image},
    lifecycle::{Window}
};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub pos: Vector,
    pub glyph: char,
    pub color: Color,
    pub hp: i32,
    pub max_hp: i32,
    pub is_in_fov: bool,
    pub color_in_fov: Color,
}

pub fn generate() -> Vec<Entity> {
    vec![
        Entity {
            pos: Vector::new(2, 4),
            glyph: ':',
            color: Palette::DARK_BLUE,
            hp: 1,
            max_hp: 1,
            is_in_fov: false,
            color_in_fov: Palette::RED,
        },
        Entity {
            pos: Vector::new(7, 5),
            glyph: 'o',
            color: Palette::DARK_BLUE,
            hp: 0,
            max_hp: 0,
            is_in_fov: false,
            color_in_fov: Palette::WHITE,
        },
    ]
}

pub fn draw_entities(window: &mut Window, entities: &Vec<Entity>, tileset: &mut HashMap<char, Image>) -> () {
    for entity in entities.iter() {
        if let Some(image) = tileset.get(&entity.glyph) {
            let pos_px = entity.pos.times(TILE_SIZE);
            //Clear the cell
            window.draw(
                &Rectangle::new(pos_px, TILE_SIZE),
                Col(Palette::DARK_BLUE),
            );
            window.draw(
                &Rectangle::new(pos_px, image.area().size()),
                Blended(
                    &image,
                    if entity.is_in_fov {
                        entity.color_in_fov
                    } else {
                        entity.color
                    },
                ),
            )
        }
    }
}

pub fn place_player(entities: &mut Vec<Entity>, player_spawn: Vector) -> () {
    entities.push(Entity {
        pos: player_spawn,
        glyph: '@',
        color: Palette::WHITE,
        hp: 3,
        max_hp: 5,
        is_in_fov: true,
        color_in_fov: Palette::WHITE,
    });
}

pub fn move_player(dir: Vector, player: &mut Entity) -> () {
    player.pos.x += dir.x;
    player.pos.y += dir.y;
}
