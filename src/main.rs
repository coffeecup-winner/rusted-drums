use std::io::Result;

use midly::{
    num::{u15, u28, u4, u7},
    Format, Header, MetaMessage, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind,
};
use mlua::{prelude::LuaResult, Lua};

mod runtime;

const TICKS_PER_QUARTER_NOTE: u32 = 120;

fn print_midi(midi: &Smf) {
    println!("MIDI file debug print: {:#?}", midi);
}

fn generate_midi<'a, 'b>(text: &'a [u8]) -> LuaResult<Smf<'b>> {
    let lua = Lua::new();

    runtime::load_into(&lua)?;

    lua.load(text).exec()?;

    let events = runtime::get_events(&lua)?;

    let header = Header::new(
        Format::SingleTrack,
        Timing::Metrical(u15::from_int_lossy(TICKS_PER_QUARTER_NOTE as u16)),
    );
    let mut midi = Smf::new(header);

    let mut track = Track::new();

    let mut time = 0u32;
    for e in events {
        let new_time = TICKS_PER_QUARTER_NOTE * e.tick as u32 / 2;
        let delta = new_time - time;
        time = new_time;
        let message = match e.type_ {
            runtime::NOTE_ON => MidiMessage::NoteOn {
                key: u7::from_int_lossy(e.note as u8),
                vel: u7::from_int_lossy(e.velocity as u8),
            },
            runtime::NOTE_OFF => MidiMessage::NoteOff {
                key: u7::from_int_lossy(e.note as u8),
                vel: u7::from_int_lossy(e.velocity as u8),
            },
            _ => panic!("Unknown event: {}", e.type_),
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
