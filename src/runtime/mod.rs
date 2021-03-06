use mlua::{prelude::LuaResult, Lua};

const VELOCITY_DEFAULT: u8 = 100;
const VELOCITY_OFF: u8 = 64;

pub fn load_into(lua: &Lua) -> LuaResult<()> {
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
            lua.globals().set(format!("{}{}", name, octave), v)?;
        }
    }

    lua.globals().set("VELOCITY_DEFAULT", VELOCITY_DEFAULT)?;
    lua.globals().set("VELOCITY_OFF", VELOCITY_OFF)?;

    lua.load(include_str!("lib.lua")).exec()?;

    Ok(())
}
