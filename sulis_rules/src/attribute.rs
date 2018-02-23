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

use std::slice::Iter;

use self::Attribute::*;

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(deny_unknown_fields)]
pub struct AttributeList {
    #[serde(rename="str")]
    strength: u8,

    #[serde(rename="dex")]
    dexterity: u8,

    #[serde(rename="end")]
    endurance: u8,

    #[serde(rename="per")]
    perception: u8,

    #[serde(rename="int")]
    intellect: u8,

    #[serde(rename="wis")]
    wisdom: u8,
}

impl AttributeList {
    pub fn new(base_value: u8) -> AttributeList {
        AttributeList {
            strength: base_value,
            dexterity: base_value,
            endurance: base_value,
            perception: base_value,
            intellect: base_value,
            wisdom: base_value,
        }
    }

    pub fn bonus(&self, attr: Attribute, base_attr: i32) -> i32 {
        (self.get(attr) as i32 - base_attr)
    }

    pub fn get(&self, attr: Attribute) -> u8 {
        match attr {
            Strength => self.strength,
            Dexterity => self.dexterity,
            Endurance => self.endurance,
            Perception => self.perception,
            Intellect => self.intellect,
            Wisdom => self.wisdom,
        }
    }

    pub fn set(&mut self, attr: Attribute, value: u8) {
        match attr {
            Strength => self.strength = value,
            Dexterity => self.dexterity = value,
            Endurance => self.endurance = value,
            Perception => self.perception = value,
            Intellect => self.intellect = value,
            Wisdom => self.wisdom = value,
        }
    }

    pub fn add_all(&mut self, attrs: &Vec<(Attribute, u8)>) {
        for &(attr, value) in attrs.iter() {
            self.add(attr, value);
        }
    }

    pub fn add(&mut self, attr: Attribute, value: u8) {
        match attr {
            Strength => self.strength += value,
            Dexterity => self.dexterity += value,
            Endurance => self.endurance += value,
            Perception => self.perception += value,
            Intellect => self.intellect += value,
            Wisdom => self.wisdom += value,
        }
    }

    pub fn sum(&self, other: &AttributeList) -> AttributeList {
        AttributeList {
            strength: self.strength + other.strength,
            dexterity: self.dexterity + other.dexterity,
            endurance: self.endurance + other.endurance,
            perception: self.perception + other.perception,
            intellect: self.intellect + other.intellect,
            wisdom: self.wisdom + other.wisdom,
        }
    }
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub enum Attribute {
    Strength,
    Dexterity,
    Endurance,
    Perception,
    Intellect,
    Wisdom,
}

const ATTRS_LIST: [Attribute; 6] = [ Strength, Dexterity, Endurance, Perception, Intellect, Wisdom ];

impl Attribute {
    pub fn from(text: &str) -> Option<Attribute> {
        Some(match text {
            "Strength" => Strength,
            "Dexterity" => Dexterity,
            "Endurance" => Endurance,
            "Perception" => Perception,
            "Intellect" => Intellect,
            "Wisdom" => Wisdom,
            _ => return None,
        })
    }

    pub fn name(&self) -> &str {
        match *self {
            Strength => "Strength",
            Dexterity => "Dexterity",
            Endurance => "Endurance",
            Perception => "Perception",
            Intellect => "Intellect",
            Wisdom => "Wisdom",
        }
    }

    pub fn short_name(&self) -> &str {
        match *self {
            Strength => "str",
            Dexterity => "dex",
            Endurance => "end",
            Perception => "per",
            Intellect => "int",
            Wisdom => "wis",
        }
    }

    pub fn iter() -> Iter<'static, Attribute> {
        ATTRS_LIST.iter()
    }
}