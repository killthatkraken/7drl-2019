use crate::map::{get_line, Map, Palette, TILE_SIZE};
use crate::ui::{MessageLog, UIData};
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::*, Color, Image},
    lifecycle::Window,
};
use slotmap::{DefaultKey, SlotMap};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Entity {
    pub key: DefaultKey,
    pub name: &'static str,
    pub glyph: char,
    pub color: Color,
    pub hp: i32,
    pub max_hp: i32,
    pub is_in_fov: bool,
    pub color_in_fov: Color,
    pub pos: Vector,
    pickable: bool,
    z: i32,
}

impl Entity {
    fn new_pebble(key: DefaultKey, pos: Vector) -> Entity {
        Entity {
            key,
            name: "pebble",
            glyph: '.',
            color: Palette::DARK_BLUE,
            hp: 0,
            max_hp: 0,
            is_in_fov: false,
            color_in_fov: Palette::WHITE,
            pos,
            pickable: true,
            z: 1,
        }
    }
    pub fn new_crosshair(key: DefaultKey, pos: Vector) -> Entity {
        Entity {
            key,
            name: "crosshair",
            glyph: 'x',
            color: Palette::WHITE,
            hp: 0,
            max_hp: 0,
            is_in_fov: true,
            color_in_fov: Palette::WHITE,
            pos,
            pickable: false,
            z: 10,
        }
    }
    pub fn new_player(key: DefaultKey, pos: Vector) -> Entity {
        Entity {
            key,
            name: "player",
            glyph: '@',
            color: Palette::WHITE,
            hp: 3,
            max_hp: 5,
            is_in_fov: true,
            color_in_fov: Palette::WHITE,
            pos,
            pickable: false,
            z: 2,
        }
    }
}

const PEBBLE_PERC: u32 = 7;

pub fn generate(map: &Map) -> SlotMap<DefaultKey, Entity> {
    use rand::distributions::Uniform;
    use rand::prelude::*;

    let mut rng = thread_rng();
    let die_range = Uniform::new_inclusive(1, 100);

    let mut entities: SlotMap<DefaultKey, Entity> = SlotMap::new();

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if !map[x][y].blocks {
                let roll = die_range.sample(&mut rng);
                if roll <= PEBBLE_PERC {
                    entities.insert_with_key(|k| {
                        Entity::new_pebble(k, Vector::new(x as i32, y as i32))
                    });
                }
            }
        }
    }

    entities
}

pub fn draw_entities(
    window: &mut Window,
    entities: &SlotMap<DefaultKey, Entity>,
    tileset: &mut HashMap<char, Image>,
) {
    entities.iter().for_each(|(_k, entity)| {
        let image = tileset.get(&entity.glyph).unwrap();
        if entity.name == "crosshair" {
            window.draw_ex(
                &Rectangle::new(entity.pos.times(TILE_SIZE), image.area().size()),
                Col(Palette::DARK_BLUE),
                Transform::IDENTITY,
                entity.z - 1
            )
        }
        window.draw_ex(
            &Rectangle::new(entity.pos.times(TILE_SIZE), image.area().size()),
            Blended(
                &image,
                if entity.is_in_fov {
                    entity.color_in_fov
                } else {
                    entity.color
                },
            ),
            Transform::IDENTITY,
            entity.z,
        )
    });
}

pub fn compute_fov(entities: &mut SlotMap<DefaultKey, Entity>, player_pos: Vector) {
    entities.iter_mut().for_each(|(_k, entity)| {
        if entity.name != "crosshair" {
            entity.is_in_fov = is_in_range(entity.pos, player_pos);
        } else {
            entity.is_in_fov = true;
        }
    });
}

fn is_in_range(from: Vector, to: Vector) -> bool {
    get_line(from, to).len() <= 2
}

pub fn pickup(
    entities: &mut SlotMap<DefaultKey, Entity>,
    player_pos: Vector,
    ui_data: &mut UIData,
    message_log: &mut MessageLog,
) {
    let mut to_pickup = 0;
    entities.retain(|_k, entity| {
        if entity.pos == player_pos && entity.pickable {
            to_pickup += 1;
            false
        } else {
            true
        }
    });
    if to_pickup > 0 {
        message_log.push("pickup");
    }

    ui_data.pebbles += to_pickup;
}
