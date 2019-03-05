mod entities;
mod map;
mod ui;

use crate::entities::Entity;
use crate::map::{Map, Palette};
use crate::ui::UIData;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Font, FontStyle, Image},
    input::{ButtonState::*, Key},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};
use std::collections::HashMap;
use slotmap::{SlotMap, DefaultKey};

struct Game {
    tileset: Asset<HashMap<char, Image>>,
    map: Map,
    entities: SlotMap<DefaultKey, Entity>,
    player_key: DefaultKey,
    ui_data: UIData,
}

impl State for Game {
    //Load assets and initialize
    fn new() -> Result<Self> {
        let square_font = "square.ttf";

        let (map, player_spawn) = map::generate();
        let mut entities = entities::generate(&map);

        let player_key = entities.insert_with_key(|k| {
            Entity::new_player(k, player_spawn)
        });

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

        let ui_text = Asset::new(Font::load(square_font).and_then(move |text| {
            let mut texts = HashMap::new();
            let style = FontStyle::new(12.0, Palette::WHITE);
            texts.insert(String::from("pebbles"), text.render("Pebbles: ", &style)?);
            texts.insert(String::from("turn"), text.render("Turn: ", &style)?);
            Ok(texts)
        }));

        Ok(Self {
            map,
            entities,
            tileset,
            player_key,
            ui_data: UIData::new_game(ui_text),
        })
    }

    //Process keyboard, mouse, update game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
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

        let player_pos = self.entities.get(self.player_key).unwrap().pos.clone();

        let future_x = player_pos.x as i32 + direction.x as i32;
        let future_y = player_pos.y as i32 + direction.y as i32;
        if !self.map[future_x as usize][future_y as usize].blocks
            && future_x != 0
            && future_x != map::MAP_SIZE.x as i32
            && future_y != 0
            && future_y != map::MAP_SIZE.y as i32
        {
            self.entities.get_mut(self.player_key).unwrap().pos = Vector::new(future_x, future_y);
        }

        if window.keyboard()[Key::R] == Pressed {
            let (map, _player_spawn) = map::generate();
            self.map = map;
        }

        entities::pickup(&mut self.entities, &player_pos, &mut self.ui_data);
        entities::compute_fov(&mut self.entities, &player_pos);
        map::compute_fov(&mut self.map, &player_pos);
        Ok(())
    }

    //Draw stuff
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Palette::DARK_BLUE)?;

        let tileset = &mut self.tileset;

        let map = &self.map;
        let entities = &self.entities;
        let ui_data = &mut self.ui_data;

        tileset.execute(|tileset| {
            map::draw_map(window, map, tileset);
            entities::draw_entities(window, entities, tileset);
            ui::draw_ui(window, ui_data, tileset);
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
