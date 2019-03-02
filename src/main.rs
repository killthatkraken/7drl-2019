mod entities;
mod map;
mod ui;

use crate::entities::Entity;
use crate::map::{Map, Palette, Tile};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Font, FontStyle, Image},
    input::{ButtonState::*, Key},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};
use std::collections::HashMap;

struct Game {
    turn: u32,
    map: Vec<Vec<Tile>>,
    entities: Vec<Entity>,
    player_id: usize,
    tileset: Asset<HashMap<char, Image>>,
}

impl State for Game {
    //Load assets and initialize
    fn new() -> Result<Self> {
        let square_font = "square.ttf";

        let (map, player_spawn) = map::generate();
        let mut entities = entities::generate(&map);

        let player_id = entities.len();
        entities::place_player(&mut entities, player_spawn);

        let tileset = Asset::new(Font::load(square_font).and_then(move |text| {
            let tiles = text
                .render(map::GLYPHS, &FontStyle::new(map::TILE_SIZE.y, Color::WHITE))
                .expect("Could not render the font tileset.");

            let mut tileset = HashMap::new();
            for (index, glyph) in map::GLYPHS.chars().enumerate() {
                let pos = (index as i32 * map::TILE_SIZE.x as i32, 0);
                let tile = tiles.subimage(Rectangle::new(pos, map::TILE_SIZE));
                tileset.insert(glyph, tile);
            }
            Ok(tileset)
        }));

        Ok(Self {
            turn: 0,
            map,
            entities,
            player_id,
            tileset,
        })
    }

    //Process keyboard, mouse, update game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        map::clear_fov(self.entities[self.player_id].pos, &mut self.map);

        let mut direction = Vector::ZERO;

        if window.keyboard()[Key::Right] == Pressed {
            direction = Vector::new(1, 0);
        }
        if window.keyboard()[Key::Left] == Pressed {
            direction = Vector::new(-1, 0);
        }
        if window.keyboard()[Key::Up] == Pressed {
            direction = Vector::new(0, -1);
        }
        if window.keyboard()[Key::Down] == Pressed {
            direction = Vector::new(0, 1);
        }

        move_to(direction, &self.map, &mut self.entities[self.player_id]).and_then(|_| {
            self.turn += 1;
            Ok(())
        })?;

        if window.keyboard()[Key::R] == Pressed {
            let (map, _player_spawn) = map::generate();
            self.map = map;
        }

        fn move_to(dir: Vector, map: &Map, player: &mut Entity) -> Result<()> {
            let future_x = player.pos.x as i32 + dir.x as i32;
            let future_y = player.pos.y as i32 + dir.y as i32;
            if !map[future_x as usize][future_y as usize].blocks
                && future_x != 0
                && future_x != map::MAP_SIZE.x as i32
                && future_y != 0
                && future_y != map::MAP_SIZE.y as i32
            {
                entities::move_player(dir, player);
                Ok(())
            } else {
                Ok(())
            }
        }

        entities::compute_fov(&mut self.entities, self.player_id);
        map::compute_fov(self.entities[self.player_id].pos, &mut self.map);
        Ok(())
    }

    //Draw stuff
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Palette::DARK_BLUE)?;

        let tileset = &mut self.tileset;

        let map = &self.map;
        let entities = &self.entities;

        tileset.execute(|tileset| {
            map::draw_map(window, map, tileset);
            entities::draw_entities(window, entities, tileset);
            Ok(())
        })?;

        Ok(())
    }
}

fn main() {
    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");
    let settings = Settings {
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };

    run::<Game>("7DRL 2019", map::WINDOW_SIZE, settings);
}
