use std::io::Result;

use midly::Smf;
use mlua::{prelude::LuaResult, Lua};

mod runtime;
mod smf;

fn print_midi(midi: &Smf) {
    println!("MIDI file debug print: {:#?}", midi);
}

fn generate_midi<'a, 'b>(text: &'a [u8]) -> LuaResult<Smf<'b>> {
    let lua = Lua::new();
    runtime::load_into(&lua)?;
    lua.load(text).exec()?;
    let events = runtime::get_events(&lua)?;
    let midi = smf::create_from_events(&events);
    Ok(midi)
}

fn main() -> Result<()> {
    let command = &std::env::args().nth(1).unwrap();
    let file = &std::env::args().nth(2).unwrap();
    match command as &str {
        "print" => {
            print_midi(&Smf::parse(&std::fs::read(file)?).expect("Failed to parse a MIDI file"))
        }
        "generate" => {
            let outfile = &std::env::args().nth(3).unwrap();
            match generate_midi(&std::fs::read(file)?) {
                Ok(midi) => midi.save(outfile)?,
                Err(err) => match err {
                    mlua::Error::SyntaxError { message, .. } => {
                        eprintln!("Syntax error: {}", message)
                    }
                    mlua::Error::RuntimeError(m) => eprintln!("Runtime error: {}", m),
                    _ => eprintln!("Unknown error: {}", err),
                },
            }
        }
        _ => println!("Unknown command `{}`", command),
    }
    Ok(())
}
