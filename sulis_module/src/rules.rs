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

use std::io::Error;
use rand::{self, Rng};

use sulis_core::resource::ResourceBuilder;
use sulis_core::util::invalid_data_error;
use sulis_core::serde_json;
use sulis_core::serde_yaml;
use sulis_rules::{HitKind};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Rules {
    pub base_ap: u32,
    pub movement_ap: u32,
    pub attack_ap: u32,
    pub base_initiative: i32,

    pub graze_percentile: u32,
    pub hit_percentile: u32,
    pub crit_percentile: u32,

    pub graze_damage_multiplier: f32,
    pub hit_damage_multiplier: f32,
    pub crit_damage_multiplier: f32,

    pub dual_wield_damage_multiplier: f32,

    pub base_attribute: i32,
    pub builder_max_attribute: i32,
    pub builder_min_attribute: i32,
    pub builder_attribute_points: i32,
}

impl Rules {
    pub fn attack_roll(&self, accuracy: i32, defense: i32) -> HitKind {
        let roll = rand::thread_rng().gen_range(1, 101);
        debug!("Attack roll: {} with accuracy {} against {}", roll, accuracy, defense);

        if roll + accuracy < defense { return HitKind::Miss; }

        let result = (roll + accuracy - defense) as u32;

        if result >= self.crit_percentile {
            HitKind::Crit
        } else if result >= self.hit_percentile {
            HitKind::Hit
        } else if result >= self.graze_percentile {
            HitKind::Graze
        } else {
            HitKind::Miss
        }
    }
}

impl ResourceBuilder for Rules {
    fn owned_id(&self) -> String {
        "Rules".to_string()
    }

    fn from_json(data: &str) -> Result<Rules, Error> {
        let rules: Rules = serde_json::from_str(data)?;

        Ok(rules)
    }

    fn from_yaml(data: &str) -> Result<Rules, Error> {
        let resource: Result<Rules, serde_yaml::Error> = serde_yaml::from_str(data);

        match resource {
            Ok(resource) => Ok(resource),
            Err(e) => invalid_data_error(&format!("{}", e)),
        }
    }
}