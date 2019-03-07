use crate::ui::UIData;
use crate::map::{get_line, Map, Palette, TILE_SIZE};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Blended, Color, Image},
    lifecycle::Window,
};
use std::collections::HashMap;
use slotmap::{SlotMap, DefaultKey};

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
    pickable: bool
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
            pickable: false
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
                        Entity::new_pebble(k, Vector::new(x as f32, y as f32))
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
        window.draw(
            &Rectangle::new(
                entity.pos.times(TILE_SIZE),
                image.area().size(),
            ),
            Blended(
                &image,
                if entity.is_in_fov {
                    entity.color_in_fov
                } else {
                    entity.color
                },
            ),
        )
    });
}

pub fn compute_fov(entities: &mut SlotMap<DefaultKey, Entity>, player_pos: Vector) {
    entities.iter_mut().for_each(|(_k, entity)| {
        entity.is_in_fov = is_in_range(entity.pos, player_pos);
    });
}

fn is_in_range(from: Vector, to: Vector) -> bool {
    get_line(from, to).len() <= 2
}

pub fn pickup(entities: &mut SlotMap<DefaultKey, Entity>, player_pos: Vector, ui_data: &mut UIData) {
    let mut to_pickup = 0;
    entities.retain(|_k, entity| {
        if entity.pos == player_pos && entity.pickable {
            to_pickup += 1;
            false
        } else {
            true
        }
    });

    ui_data.pebbles += to_pickup;
}
