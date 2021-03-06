use std::collections::BTreeMap;
use std::io::Result;

use midly::{
    num::{u15, u28, u4, u7},
    Format, Header, MetaMessage, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind,
};
use mlua::{prelude::LuaResult, Lua};

mod runtime;

const TICKS_PER_QUARTER_NOTE: u32 = 120;

#[derive(Debug)]
struct LuaEvent<'lua> {
    type_: mlua::String<'lua>,
    note: mlua::Integer,
    velocity: mlua::Integer,
}

fn print_midi(midi: &Smf) {
    println!("MIDI file debug print: {:#?}", midi);
}

fn generate_midi<'a, 'b>(text: &'a [u8]) -> LuaResult<Smf<'b>> {
    let lua = Lua::new();

    runtime::load_into(&lua)?;

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

    let header = Header::new(
        Format::SingleTrack,
        Timing::Metrical(u15::from_int_lossy(TICKS_PER_QUARTER_NOTE as u16)),
    );
    let mut midi = Smf::new(header);

    let mut track = Track::new();

    let mut time = 0u32;
    for (k, v) in map {
        let new_time = TICKS_PER_QUARTER_NOTE * k as u32 / 2;
        let delta = new_time - time;
        time = new_time;
        let message = match v.type_.as_ref() {
            b"NOTE_ON" => MidiMessage::NoteOn {
                key: u7::from_int_lossy(v.note as u8),
                vel: u7::from_int_lossy(v.velocity as u8),
            },
            b"NOTE_OFF" => MidiMessage::NoteOff {
                key: u7::from_int_lossy(v.note as u8),
                vel: u7::from_int_lossy(v.velocity as u8),
            },
            _ => panic!("Unknown event: {}", v.type_.to_str()?),
        };
        track.push(TrackEvent {
            delta: u28::from_int_lossy(delta),
            kind: TrackEventKind::Midi {
                channel: u4::from_int_lossy(0),
                message,
            },
        });
    }

    track.push(TrackEvent {
        delta: u28::from_int_lossy(0),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });
    midi.tracks.push(track);

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
            let midi = generate_midi(&std::fs::read(file)?).expect("Lua script failed");
            midi.save(outfile)?;
        }
        _ => println!("Unknown command `{}`", command),
    }
    Ok(())
}
