use std::collections::BTreeMap;
use std::io::Result;

use apres::{MIDIEvent, MIDI};
use mlua::{prelude::LuaResult, Lua};

const TICKS_PER_QUARTER_NOTE: usize = 120;
const NOTE_C3: u8 = 36;
const NOTE_D3: u8 = 38;
const VELOCITY_DEFAULT: u8 = 100;
const VELOCITY_OFF: u8 = 64;

#[derive(Debug)]
struct LuaEvent<'lua> {
    type_: mlua::String<'lua>,
    note: mlua::Integer,
    velocity: mlua::Integer,
}

fn print_midi(midi: MIDI) {
    println!("Events count: {}", midi.count_events());
    for i in 1..=midi.count_events() {
        match midi.get_event(i as u64) {
            Some(event) => match event {
                MIDIEvent::NoteOn(a, b, c) => println!("NoteOn: {}, {}, {}", a, b, c),
                MIDIEvent::NoteOff(a, b, c) => println!("NoteOff: {}, {}, {}", a, b, c),
                _ => println!("UNKNOWN"),
            },
            None => println!("NONE"),
        }
    }
}

fn generate_midi() -> MIDI {
    let mut midi = MIDI::new();
    midi.insert_event(
        0,
        TICKS_PER_QUARTER_NOTE * 0,
        MIDIEvent::NoteOn(0, NOTE_C3, VELOCITY_DEFAULT),
    );
    midi.insert_event(
        0,
        TICKS_PER_QUARTER_NOTE * 1 / 2,
        MIDIEvent::NoteOff(0, NOTE_C3, VELOCITY_OFF),
    );
    midi.insert_event(
        0,
        TICKS_PER_QUARTER_NOTE * 1,
        MIDIEvent::NoteOn(0, NOTE_D3, VELOCITY_DEFAULT),
    );
    midi.insert_event(
        0,
        TICKS_PER_QUARTER_NOTE * 3 / 2,
        MIDIEvent::NoteOff(0, NOTE_D3, VELOCITY_OFF),
    );
    midi.insert_event(
        0,
        TICKS_PER_QUARTER_NOTE * 2,
        MIDIEvent::NoteOn(0, NOTE_C3, VELOCITY_DEFAULT),
    );
    midi.insert_event(
        0,
        TICKS_PER_QUARTER_NOTE * 5 / 2,
        MIDIEvent::NoteOff(0, NOTE_C3, VELOCITY_OFF),
    );
    midi.insert_event(
        0,
        TICKS_PER_QUARTER_NOTE * 3,
        MIDIEvent::NoteOn(0, NOTE_D3, VELOCITY_DEFAULT),
    );
    midi.insert_event(
        0,
        TICKS_PER_QUARTER_NOTE * 7 / 2,
        MIDIEvent::NoteOff(0, NOTE_D3, VELOCITY_OFF),
    );
    midi
}

fn generate_midi_lua(text: &[u8]) -> LuaResult<MIDI> {
    let lua = Lua::new();

    lua.globals().set("C3", NOTE_C3)?;
    lua.globals().set("D3", NOTE_D3)?;
    lua.globals().set("VELOCITY_DEFAULT", VELOCITY_DEFAULT)?;
    lua.globals().set("VELOCITY_OFF", VELOCITY_OFF)?;

    lua.load(text).exec()?;

    let events: mlua::prelude::LuaTable = lua.globals().get("__events")?;
    let mut map = BTreeMap::new();

    for pair in events.pairs::<mlua::Integer, mlua::Table>() {
        let (k, v) = pair.unwrap();
        let event = LuaEvent {
            type_: v.get("event")?,
            note: v.get("note")?,
            velocity: v.get("velocity")?,
        };
        map.insert(k, event);
    }

    let mut midi = MIDI::new();

    for (k, v) in map {
        let event = match v.type_.as_ref() {
            b"NOTE_ON" => MIDIEvent::NoteOn(0, v.note as u8, v.velocity as u8),
            b"NOTE_OFF" => MIDIEvent::NoteOff(0, v.note as u8, v.velocity as u8),
            _ => panic!("Unknown event: {}", v.type_.to_str()?),
        };
        midi.insert_event(0, TICKS_PER_QUARTER_NOTE * k as usize / 2, event);
    }

    Ok(midi)
}

fn main() -> Result<()> {
    let command = &std::env::args().nth(1).unwrap();
    let file = &std::env::args().nth(2).unwrap();
    match command as &str {
        "print" => print_midi(MIDI::from_path(file)),
        "generate" => {
            let midi = generate_midi();
            midi.save(file);
        }
        "lua" => {
            let outfile = &std::env::args().nth(3).unwrap();
            let midi = generate_midi_lua(&std::fs::read(file)?).expect("Lua script failed");
            midi.save(outfile);
        }
        _ => println!("Unknown command `{}`", command),
    }
    Ok(())
}
