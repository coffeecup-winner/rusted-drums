use mlua::{prelude::LuaResult, Lua};

const VELOCITY_DEFAULT: u8 = 100;
const VELOCITY_OFF: u8 = 64;

pub const NOTE_ON: mlua::Integer = 1;
pub const NOTE_OFF: mlua::Integer = 0;

#[derive(Debug)]
pub struct LuaEvent {
    pub tick: mlua::Integer,
    pub type_: mlua::Integer, // TODO: enum
    pub note: mlua::Integer,
    pub velocity: mlua::Integer,
}

pub struct Runtime {
    lua: Lua,
}

impl Runtime {
    pub fn new() -> LuaResult<Runtime> {
        let runtime = Runtime { lua: Lua::new() };
        Self::init(&runtime)?;
        Ok(runtime)
    }

    fn init(&self) -> LuaResult<()> {
        for v in 0..=127u8 {
            let names = match v % 12 {
                0 => vec!["C"],
                1 => vec!["Cs", "Db"],
                2 => vec!["D"],
                3 => vec!["Ds", "Eb"],
                4 => vec!["E"],
                5 => vec!["F"],
                6 => vec!["Fs", "Gb"],
                7 => vec!["G"],
                8 => vec!["Gs", "Ab"],
                9 => vec!["A"],
                10 => vec!["As", "Bb"],
                11 => vec!["B"],
                _ => panic!("Impossible"),
            };
            // Octaves are 0-based
            let octave = v / 12;
            for name in names {
                self.lua.globals().set(format!("{}{}", name, octave), v)?;
            }
        }

        self.lua
            .globals()
            .set("VELOCITY_DEFAULT", VELOCITY_DEFAULT)?;
        self.lua.globals().set("VELOCITY_OFF", VELOCITY_OFF)?;

        self.lua.globals().set("NOTE_ON", NOTE_ON)?;
        self.lua.globals().set("NOTE_OFF", NOTE_OFF)?;

        self.lua.load(include_str!("keymaps.lua")).exec()?;
        self.lua.load(include_str!("lib.lua")).exec()?;

        Ok(())
    }

    pub fn exec(&self, s: &[u8]) -> LuaResult<()> {
        self.lua.load(s).exec()
    }

    pub fn get_events(&self) -> LuaResult<Vec<LuaEvent>> {
        let events: mlua::prelude::LuaTable = self.lua.globals().get("__events")?;
        let mut result = vec![];

        for pair in events.pairs::<mlua::Integer, mlua::Table>() {
            let (_, v) = pair.unwrap();
            let event = LuaEvent {
                tick: v.get("tick")?,
                type_: v.get("event")?,
                note: v.get("note")?,
                velocity: v.get("velocity")?,
            };
            result.push(event);
        }
        result.sort_by_key(|e| e.tick);
        Ok(result)
    }
}
