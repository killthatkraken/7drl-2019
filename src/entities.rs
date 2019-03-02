use crate::map::{get_line, Map, Palette, TILE_SIZE};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Blended, Color, Image},
    lifecycle::Window,
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

impl Entity {
    fn new_pebble(pos: Vector) -> Entity {
        Entity {
            pos,
            glyph: '.',
            color: Palette::DARK_BLUE,
            hp: 0,
            max_hp: 0,
            is_in_fov: false,
            color_in_fov: Palette::WHITE,
        }
    }
    fn new_up_stairs(pos: Vector) -> Entity {
        Entity {
            pos,
            glyph: '<',
            color: Palette::DARK_BLUE,
            hp: 0,
            max_hp: 0,
            is_in_fov: false,
            color_in_fov: Palette::WHITE,
        }
    }
    fn is_in_player_fov(&self, player_pos: Vector) -> bool {
        get_line(self.pos, player_pos).len() <= 2
    }
}

const MAX_PEBBLES: u32 = 15;
const PEBBLE_PERC: u32 = 5;

pub fn generate(map: &Map) -> Vec<Entity> {
    use rand::distributions::{Distribution, Uniform};
    use rand::prelude::*;

    let mut rng = thread_rng();
    let die_range = Uniform::new_inclusive(1, 100);

    let mut entities: Vec<Entity> = vec![];
    let mut pebbles_count = 0;

    'main_loop: for x in 0..map.len() {
        for y in 0..map[x].len() {
            if !map[x][y].blocks {
                let roll = die_range.sample(&mut rng);
                if roll <= PEBBLE_PERC {
                    entities.push(Entity::new_pebble(Vector::new(x as f32, y as f32)));
                    pebbles_count += 1;
                    if pebbles_count == MAX_PEBBLES {
                        break 'main_loop;
                    }
                }
            }
        }
    }

    entities
}

pub fn draw_entities(window: &mut Window, entities: &[Entity], tileset: &mut HashMap<char, Image>) {
    for entity in entities.iter() {
        if let Some(image) = tileset.get(&entity.glyph) {
            let pos_px = entity.pos.times(TILE_SIZE);
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

pub fn place_player(entities: &mut Vec<Entity>, player_spawn: Vector) {
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

pub fn compute_fov(entities: &mut Vec<Entity>, player_id: usize) {
    let player = entities[player_id].clone();
    entities.iter_mut().for_each(|entity| {
        entity.is_in_fov = entity.is_in_player_fov(player.pos);
    });
}

pub fn move_player(dir: Vector, player: &mut Entity) {
    player.pos.x += dir.x;
    player.pos.y += dir.y;
}
