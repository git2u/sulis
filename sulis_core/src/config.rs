//  This file is part of Sulis, a turn based RPG written in Rust.
//  Copyright 2018 Jared Stephen
//
//  Sulis is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  Sulis is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with Sulis.  If not, see <http://www.gnu.org/licenses/>

use std::env;
use std::io::{Read, Error, ErrorKind};
use std::path::Path;
use std::fs::{self, File};
use std::path::PathBuf;
use std::collections::HashMap;

use io::keyboard_event::Key;
use io::{KeyboardEvent, InputAction};

use serde_yaml;

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
  pub display: DisplayConfig,
  pub resources: ResourcesConfig,
  pub input: InputConfig,
  pub logging: LoggingConfig,
  pub editor: EditorConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct EditorConfig {
    pub module: String,
    pub cursor: String,
    pub transition_image: String,
    pub transition_size: String,
    pub area: EditorAreaConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct EditorAreaConfig {
    pub filename: String,
    pub id: String,
    pub name: String,
    pub visibility_tile: String,
    pub explored_tile: String,
    pub layers: Vec<String>,
    pub elev_tiles: Vec<String>,
    pub entity_layer: usize,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct LoggingConfig {
    pub log_level: String,
    pub use_timestamps: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DisplayConfig {
    pub adapter: IOAdapter,
    pub frame_rate: u32,
    pub animation_base_time_millis: u32,
    pub width: i32,
    pub height: i32,
    pub width_pixels: u32,
    pub height_pixels: u32,
    pub default_font: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ResourcesConfig {
    pub directory: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct InputConfig {
    pub keybindings: HashMap<Key, InputAction>
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(deny_unknown_fields)]
pub enum IOAdapter {
    Auto,
    Glium,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::init();
    pub static ref USER_DIR: PathBuf = get_user_dir();
}

#[cfg(not(target_os = "windows"))]
fn get_user_dir() -> PathBuf {
    let mut path = get_home_dir();
    path.push(".sulis/");
    path
}

#[cfg(target_os = "windows")]
fn get_user_dir() -> PathBuf {
    let mut path = get_home_dir();
    path.push("My Documents");
    path.push("My Games");
    path.push("Sulis");
    path
}

fn get_home_dir() -> PathBuf {
    match env::home_dir() {
        Some(path) => path,
        None => PathBuf::new(),
    }
}

const CONFIG_FILENAME: &str = "config.yml";
const CONFIG_BASE: &str = "config.sample.yml";

impl Config {
    fn init() -> Config {
        let mut config_path = USER_DIR.clone();
        config_path.push(CONFIG_FILENAME);
        let config_path = config_path.as_path();

        let config_base_path = Path::new(CONFIG_BASE);

        if !config_path.is_file() {
            println!("{} not found, attempting to create it from {}", CONFIG_FILENAME, CONFIG_BASE);
            if let Some(path) = config_path.parent() {
                match fs::create_dir_all(path) {
                    Err(_) => (),
                    Ok(_) => (),
                };
            }

            match fs::copy(config_base_path, config_path) {
                Err(_) => {
                    let config_base_str = format!("../{}", CONFIG_BASE);
                    let config_base_path = Path::new(&config_base_str);
                    match fs::copy(config_base_path, config_path) {
                        Err(e) => {
                            eprintln!("{}", e);
                            eprintln!("Unable to create configuration file '{}'", CONFIG_FILENAME);
                            eprintln!("Exiting...");
                            ::std::process::exit(1);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }

        }

        let config = Config::new(config_path);
        match config {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                eprintln!("Fatal error loading the configuration from '{}'", CONFIG_FILENAME);
                eprintln!("Exiting...");
                ::std::process::exit(1);
            }
        }
    }

    fn new(filepath: &Path) -> Result<Config, Error> {
        let mut f = File::open(filepath)?;
        let mut file_data = String::new();
        f.read_to_string(&mut file_data)?;

        let config: Result<Config, serde_yaml::Error> = serde_yaml::from_str(&file_data);
        let config = match config {
            Ok(config) => config,
            Err(e) => {
                return Err(Error::new(ErrorKind::InvalidData, format!("{}", e)));
            }
        };

        match config.logging.log_level.as_ref() {
            "error" | "warn" | "info" | "debug" | "trace" => (),
            _ => return Err(Error::new(ErrorKind::InvalidData,
                    format!("log_level must be one of error, warn, info, debug, or trace")))
        };

        if config.display.width < 80 || config.display.height < 24 {
            return Err(Error::new(ErrorKind::InvalidData,
                "Minimum terminal display size is 80x24"));
        }

        Ok(config)
    }

    pub fn get_input_action(&self, k: Option<KeyboardEvent>) -> Option<InputAction> {
        match k {
            None => None,
            Some(k) => {
                debug!("Got keyboard input '{:?}'", k);
                match self.input.keybindings.get(&k.key) {
                    None => None,
                    Some(action) => Some(*action),
                }
            }
        }
    }
}
