extern crate rand;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Blended, Color, Image},
    lifecycle::Window,
};
use std::collections::HashMap;

pub struct Palette;

#[allow(dead_code)]
impl Palette {
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const DARK_GRAY: Color = Color {
        r: 0.37,
        g: 0.24,
        b: 0.23,
        a: 1.0,
    };
    pub const LIGHT_GRAY: Color = Color {
        r: 0.76,
        g: 0.76,
        b: 0.78,
        a: 1.0,
    };
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 0.94,
        b: 0.9,
        a: 1.0,
    };
    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 0.92,
        b: 0.15,
        a: 1.0,
    };
    pub const ORANGE: Color = Color {
        r: 1.0,
        g: 0.63,
        b: 0.0,
        a: 1.0,
    };
    pub const PEACH: Color = Color {
        r: 1.0,
        g: 0.8,
        b: 0.66,
        a: 1.0,
    };
    pub const BROWN: Color = Color {
        r: 0.67,
        g: 0.32,
        b: 0.21,
        a: 1.0,
    };
    pub const PINK: Color = Color {
        r: 1.0,
        g: 0.46,
        b: 0.65,
        a: 1.0,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.30,
        a: 1.0,
    };
    pub const INDIGO: Color = Color {
        r: 0.51,
        g: 0.46,
        b: 0.61,
        a: 1.0,
    };
    pub const PURPLE: Color = Color {
        r: 0.49,
        g: 0.14,
        b: 0.32,
        a: 1.0,
    };
    pub const LIGHT_BLUE: Color = Color {
        r: 0.16,
        g: 0.67,
        b: 1.0,
        a: 1.0,
    };
    pub const DARK_BLUE: Color = Color {
        r: 0.11,
        g: 0.16,
        b: 0.32,
        a: 1.0,
    };
    pub const DARK_GREEN: Color = Color {
        r: 0.0,
        g: 0.53,
        b: 0.31,
        a: 1.0,
    };
    pub const LIGHT_GREEN: Color = Color {
        r: 0.0,
        g: 0.89,
        b: 0.21,
        a: 1.0,
    };
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub name: String,
    pub glyph: char,
    pub color: Color,
    pub is_in_fov: bool,
    pub color_in_fov: Color,
    pub blocks: bool,
}

impl Tile {
    fn new_wall() -> Tile {
        Tile {
            name: String::from("wall"),
            glyph: '#',
            color: Palette::DARK_BLUE,
            is_in_fov: false,
            color_in_fov: Palette::WHITE,
            blocks: true,
        }
    }
    fn new_floor() -> Tile {
        Tile {
            name: String::from(String::from("floor")),
            glyph: '.',
            color: Palette::DARK_BLUE,
            is_in_fov: false,
            color_in_fov: Palette::WHITE,
            blocks: false,
        }
    }
}

pub const WINDOW_SIZE: Vector = Vector {
    x: 1024.0,
    y: 768.0,
};
pub const GLYPHS: &str = "@#.x >_+Xo:|";
pub const TILE_SIZE: Vector = Vector { x: 12.0, y: 12.0 };
pub const MAP_SIZE: Vector = Vector { x: 57.0, y: 40.0 };

pub fn compute_fov(player_pos: Vector, map: &mut Vec<Vec<Tile>>) -> () {
    for dx in -2..3 {
        for dy in -2..3 {
            let future_x = player_pos.x + dx as f32;
            let future_y = player_pos.y + dy as f32;
            if future_x != 0.0
                && future_x != MAP_SIZE.x
                && future_y != 0.0
                && future_y != MAP_SIZE.y
            {
                let line_to_point = get_line(player_pos, Vector::new(future_x, future_y));
                for point in line_to_point.iter() {
                    let tile = &mut map[point.x as usize][point.y as usize];
                    tile.is_in_fov = true;
                    if tile.blocks {
                        break;
                    }
                }
            }
        }
    }
}

pub fn clear_fov(player_pos: Vector, map: &mut Vec<Vec<Tile>>) -> () {
    for dx in -2..3 {
        for dy in -2..3 {
            let future_x = player_pos.x + dx as f32;
            let future_y = player_pos.y + dy as f32;
            if future_x != 0.0
                && future_x != MAP_SIZE.x
                && future_y != 0.0
                && future_y != MAP_SIZE.y
            {
                map[future_x as usize][future_y as usize].is_in_fov = false;
            }
        }
    }
}

pub fn generate() -> (Vec<Vec<Tile>>, Vector) {
    use rand::distributions::{Distribution, Uniform};
    use rand::prelude::*;

    const WALL_PERC: u32 = 35;
    const ITERATIONS: u32 = 5;
    let mut rng = thread_rng();
    let die_range = Uniform::new_inclusive(1, 100);

    fn count_walls(pos: Vector, map: &Vec<Vec<Tile>>) -> u32 {
        let mut total_walls = 0;
        let pos_x: i32 = pos.x as i32;
        let pos_y: i32 = pos.y as i32;
        for sx in -1..2 {
            for sy in -1..2 {
                if map[(pos_x + sx) as usize][(pos_y + sy) as usize].name == String::from("wall") {
                    total_walls += 1;
                }
            }
        }

        total_walls
    }

    let width = MAP_SIZE.x as usize;
    let height = MAP_SIZE.y as usize;
    let mut map: Vec<Vec<Tile>> = vec![vec![Tile::new_floor(); height]; width];
    let mut first_floor = Vector::new(0, 0);
    for x in 0..width {
        for y in 0..height {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                map[x][y] = Tile::new_wall();
            } else {
                let roll_die = die_range.sample(&mut rng);
                if roll_die <= WALL_PERC {
                    map[x][y] = Tile::new_wall();
                }
            }
        }
    }

    for _i in 0..ITERATIONS {
        let mut to_wall: Vec<Vector> = vec![];
        let mut to_floor: Vec<Vector> = vec![];
        for x in 0..width {
            for y in 0..height {
                let this_tile = &map[x][y];
                let this_pos = Vector::new(x as f32, y as f32);

                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    to_floor.push(this_pos);
                } else {
                    let surrounding_walls = count_walls(this_pos, &map);
                    if surrounding_walls >= 5 && this_tile.name != "wall" {
                        to_wall.push(this_pos);
                    } else if surrounding_walls <= 3 && this_tile.name != "floor" {
                        to_floor.push(this_pos);
                    }
                }
            }
        }
        for future_wall in to_wall.iter() {
            map[future_wall.x as usize][future_wall.y as usize] = Tile::new_wall();
        }
        for future_floor in to_wall.iter() {
            map[future_floor.x as usize][future_floor.y as usize] = Tile::new_floor();
        }
    }

    for (x, col) in map.iter().enumerate() {
        for (y, tile) in col.iter().enumerate() {
            if tile.name == "floor" {
                first_floor = Vector::new(x as i32, y as i32)
            }
        }
    }

    (map, first_floor)
}

pub fn draw_map(
    window: &mut Window,
    map: &Vec<Vec<Tile>>,
    tileset: &mut HashMap<char, Image>,
) -> () {
    for (x, tile_col) in map.iter().enumerate() {
        for (y, tile) in tile_col.iter().enumerate() {
            if let Some(image) = tileset.get(&tile.glyph) {
                let pos_px = Vector::new(x as i32, y as i32).times(TILE_SIZE);
                window.draw(
                    &Rectangle::new(pos_px, image.area().size()),
                    Blended(
                        &image,
                        if tile.is_in_fov {
                            tile.color_in_fov
                        } else {
                            tile.color
                        },
                    ),
                )
            }
        }
    }
}

//Bresenham's from http://www.roguebasin.com/index.php?title=Bresenham%27s_Line_Algorithm
pub fn get_line(a: Vector, b: Vector) -> Vec<Vector> {
    let mut points = Vec::<Vector>::new();
    let mut x1 = a.x as i32;
    let mut y1 = a.y as i32;
    let mut x2 = b.x as i32;
    let mut y2 = b.y as i32;
    let is_steep = (y2 - y1).abs() > (x2 - x1).abs();
    if is_steep {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
    }
    let mut reversed = false;
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
        reversed = true;
    }
    let dx = x2 - x1;
    let dy = (y2 - y1).abs();
    let mut err = dx / 2;
    let mut y = y1;
    let ystep: i32;
    if y1 < y2 {
        ystep = 1;
    } else {
        ystep = -1;
    }
    for x in x1..(x2 + 1) {
        if is_steep {
            points.push(Vector {
                x: y as f32,
                y: x as f32,
            });
        } else {
            points.push(Vector {
                x: x as f32,
                y: y as f32,
            });
        }
        err -= dy;
        if err < 0 {
            y += ystep;
            err += dx;
        }
    }

    if reversed {
        for i in 0..(points.len() / 2) {
            let end = points.len() - 1;
            points.swap(i, end - i);
        }
    }
    points
}
