mod entities;
mod map;
mod ui;

use crate::entities::Entity;
use crate::map::{Map, Palette};
use crate::ui::{MessageLog, UIData};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Font, FontStyle, Image},
    input::{ButtonState::*, Key},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};
use slotmap::{DefaultKey, SlotMap};
use std::collections::HashMap;

enum GameState {
    Moving,
    Throwing,
    Dead,
}

struct Game {
    tileset: Asset<HashMap<char, Image>>,
    map: Map,
    entities: SlotMap<DefaultKey, Entity>,
    player_key: DefaultKey,
    crosshair_key: Option<DefaultKey>,
    ui_data: UIData,
    message_log: MessageLog,
    state: GameState,
}

impl State for Game {
    //Load assets and initialize
    fn new() -> Result<Self> {
        let square_font = "square.ttf";

        let (map, player_spawn) = map::generate();
        let mut entities = entities::generate(&map);

        let player_key = entities.insert_with_key(|k| Entity::new_player(k, player_spawn));

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
            let log_style = FontStyle::new(9.0, Palette::WHITE);
            texts.insert("pebbles", text.render("Pebbles: ", &style)?);
            texts.insert("message_log", text.render("Messages:", &style)?);
            texts.insert("dark", text.render("It's dark around you...", &log_style)?);
            texts.insert("pickup", text.render("You pickup a pebble", &log_style)?);
            texts.insert(
                "throw_mode_enter",
                text.render("Where do you want to throw?", &log_style)?,
            );
            texts.insert(
                "throw_mode_exit",
                text.render("You stopped throwing.", &log_style)?,
            );
            texts.insert(
                "died",
                text.render("You died. Press R to restart or Q to quit.", &log_style)?,
            );
            Ok(texts)
        }));

        Ok(Self {
            map,
            entities,
            tileset,
            player_key,
            crosshair_key: None,
            ui_data: UIData::new(ui_text),
            message_log: MessageLog::new(),
            state: GameState::Moving,
        })
    }

    //Process keyboard, mouse, update game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        let player_pos = self.entities.get(self.player_key).unwrap().pos;
        
        match &mut self.state {
            GameState::Moving => {
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
                if window.keyboard()[Key::T] == Pressed {
                    self.crosshair_key = Some(
                        self.entities
                            .insert_with_key(|k| Entity::new_crosshair(k, player_pos)),
                    );
                    self.message_log.push("throw_mode_enter");
                    self.state = GameState::Throwing;
                }

                let future_pos =
                    Vector::new(player_pos.x + direction.x, player_pos.y + direction.y);
                if !self.map[future_pos.x as usize][future_pos.y as usize].blocks
                    && map::is_in_bounds(future_pos)
                {
                    self.entities.get_mut(self.player_key).unwrap().pos = future_pos
                }
            }
            GameState::Throwing => {
                let crosshair_pos = self.entities.get(self.crosshair_key.unwrap()).unwrap().pos;
                let mut crosshair_direction = Vector::ZERO;

                if window.keyboard()[Key::Escape] == Pressed {
                    self.message_log.push("throw_mode_exit");
                    self.state = GameState::Moving;
                }
                if window.keyboard()[Key::Right] == Pressed {
                    crosshair_direction = Vector::new(1, 0);
                }
                if window.keyboard()[Key::Left] == Pressed {
                    crosshair_direction = Vector::new(-1, 0);
                }
                if window.keyboard()[Key::Up] == Pressed {
                    crosshair_direction = Vector::new(0, -1);
                }
                if window.keyboard()[Key::Down] == Pressed {
                    crosshair_direction = Vector::new(0, 1);
                }

                let future_pos = Vector::new(
                    crosshair_pos.x + crosshair_direction.x,
                    crosshair_pos.y + crosshair_direction.y,
                );
                if map::is_in_bounds(future_pos) {
                    let crosshair = self.entities.get_mut(self.crosshair_key.unwrap()).unwrap();
                    crosshair.pos = future_pos;
                }
            }
            GameState::Dead => {
                if window.keyboard()[Key::Q] == Pressed {
                    window.close();
                }
                if window.keyboard()[Key::R] == Pressed {
                    let (map, _player_spawn) = map::generate();
                    self.map = map;
                    self.ui_data.pebbles = 0;
                }
            }
        }

        entities::pickup(
            &mut self.entities,
            player_pos,
            &mut self.ui_data,
            &mut self.message_log,
        );
        entities::compute_fov(&mut self.entities, player_pos);
        map::compute_fov(&mut self.map, player_pos);
        Ok(())
    }

    //Draw stuff
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Palette::DARK_BLUE)?;

        let tileset = &mut self.tileset;

        let map = &self.map;
        let entities = &self.entities;
        let ui_data = &mut self.ui_data;
        let message_log = &mut self.message_log;

        tileset.execute(|tileset| {
            map::draw_map(window, map, tileset);
            entities::draw_entities(window, entities, tileset);
            ui::draw_ui(window, ui_data, message_log, tileset);
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
