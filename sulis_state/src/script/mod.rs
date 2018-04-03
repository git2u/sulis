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

mod area_targeter;
use self::area_targeter::AreaTargeter;

pub mod script_callback;
use self::script_callback::CallbackData;
pub use self::script_callback::ScriptCallback;
pub use self::script_callback::ScriptHitKind;

mod script_effect;
use self::script_effect::ScriptEffect;

mod script_entity;
pub use self::script_entity::ScriptEntity;
pub use self::script_entity::ScriptEntitySet;

mod script_inventory;
pub use self::script_inventory::ScriptInventory;

mod script_particle_generator;
use self::script_particle_generator::ScriptParticleGenerator;

mod script_subpos_animation;
use self::script_subpos_animation::ScriptSubposAnimation;

pub mod targeter;
use self::targeter::Targeter;
use self::targeter::TargeterData;

use std;
use std::rc::Rc;
use std::cell::RefCell;

use rlua::{self, Function, Lua, UserData, UserDataMethods};

use sulis_core::config::CONFIG;
use sulis_core::util::Point;
use sulis_module::{Ability};
use {EntityState, GameState};

type Result<T> = std::result::Result<T, rlua::Error>;

pub struct ScriptState {
    lua: Lua,
}

impl ScriptState {
    pub fn new() -> ScriptState {
        let lua = Lua::new();

        {
            let globals = lua.globals();
            match globals.set("game", ScriptInterface {}) {
                Ok(()) => (),
                Err(e) => {
                    warn!("Error setting up Lua globals");
                    warn!("{}", e);
                },
            }
        }

        ScriptState { lua }
    }

    fn execute_script(&self, parent: &Rc<RefCell<EntityState>>, function_args: &str,
                          script: &str, function: &str) -> Result<()> {
        let globals = self.lua.globals();
        globals.set("parent", ScriptEntity::from(&parent))?;

        debug!("Loading script for '{}'", function);

        let script = format!("{}\n{}{}", script, function, function_args);
        let func: Function = self.lua.load(&script, Some(function))?;

        debug!("Calling script function '{}'", function);

        func.call::<_, ()>("")?;

        Ok(())
    }

    pub fn ability_on_activate(&self, parent: &Rc<RefCell<EntityState>>,
                                       ability: &Rc<Ability>) -> Result<()> {
        let t: Option<(&str, usize)> = None;
        self.ability_script(parent, ability, ScriptEntitySet::new(parent, &Vec::new()), t, "on_activate")
    }

    pub fn ability_on_target_select(&self, parent: &Rc<RefCell<EntityState>>,
                                    ability: &Rc<Ability>, targets: Vec<Option<Rc<RefCell<EntityState>>>>,
                                    selected_point: Point) -> Result<()> {
        let mut targets = ScriptEntitySet::new(parent, &targets);
        targets.point = Some((selected_point.x, selected_point.y));
        let t: Option<(&str, usize)> = None;
        self.ability_script(parent, ability, targets, t,"on_target_select")
    }

    pub fn ability_script<'a, T>(&'a self, parent: &Rc<RefCell<EntityState>>, ability: &Rc<Ability>,
                                 targets: ScriptEntitySet, arg: Option<(&str, T)>,
                                 func: &str) -> Result<()> where T: rlua::prelude::ToLua<'a> + Send {
        let script = get_script(ability)?;
        self.lua.globals().set("ability", ScriptAbility::from(ability))?;
        self.lua.globals().set("targets", targets)?;

        let mut args_string = "(parent, ability, targets".to_string();
        if let Some((arg_str, arg)) = arg {
            args_string.push_str(", ");
            args_string.push_str(arg_str);

            self.lua.globals().set(arg_str, arg)?;
        }
        args_string.push(')');
        self.execute_script(parent, &args_string, script, func)
    }
}

fn get_script(ability: &Rc<Ability>) -> Result<&str> {
    match ability.active {
        None => Err(rlua::Error::ToLuaConversionError {
            from: "Rc<Ability>",
            to: "ScriptAbility",
            message: Some("The Ability is not active".to_string()),
        }),
        Some(ref active) => Ok(&active.script),
    }
}

struct ScriptInterface { }

impl UserData for ScriptInterface {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        methods.add_method("log", |_, _, val: String| {
            info!("[LUA]: {}", val);
            Ok(())
        });

        methods.add_method("anim_base_time", |_, _, ()| {
            let secs = CONFIG.display.animation_base_time_millis as f32 / 1000.0;
            Ok(secs)
        });

        methods.add_method("atan2", |_, _, (x, y): (f32, f32)| {
            Ok(y.atan2(x))
        });

        methods.add_method("is_passable", |_, _, (entity, x, y): (ScriptEntity, i32, i32)| {
            let area_state = GameState::area_state();
            let area_state = area_state.borrow();
            let entity = entity.try_unwrap()?;
            let entity = entity.borrow();
            Ok(area_state.is_passable(&entity, x, y))
        });
    }
}

#[derive(Clone)]
pub struct ScriptAbility {
    id: String,
    name: String,
    duration: u32,
    ap: u32,
}

impl ScriptAbility {
    fn from(ability: &Rc<Ability>) -> ScriptAbility {
        assert!(ability.active.is_some());

        ScriptAbility {
            id: ability.id.to_string(),
            name: ability.name.to_string(),
            duration: ability.active.as_ref().unwrap().duration,
            ap: ability.active.as_ref().unwrap().ap,
        }
    }
}

impl UserData for ScriptAbility {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        methods.add_method("activate", &activate);
        methods.add_method("name", |_, ability, ()| {
            Ok(ability.name.to_string())
        });
        methods.add_method("duration", |_, ability, ()| Ok(ability.duration));

        methods.add_method("create_callback", |_, ability, parent: ScriptEntity| {
            let index = parent.try_unwrap_index()?;
            let cb_data = CallbackData::new(index, &ability.id);
            Ok(cb_data)
        });
    }
}

fn activate(_lua: &Lua, ability: &ScriptAbility, target: ScriptEntity) -> Result<()> {
    let entity = target.try_unwrap()?;

    let area_state = GameState::area_state();
    if area_state.borrow().turn_timer.is_active() {
        entity.borrow_mut().actor.remove_ap(ability.ap);
    }

    entity.borrow_mut().actor.activate_ability_state(&ability.id);

    Ok(())
}