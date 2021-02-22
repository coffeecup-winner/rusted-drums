use std::io::Result;

use apres::{MIDIEvent, MIDI};

const TICKS_PER_QUARTER_NOTE: usize = 120;
const NOTE_C3: u8 = 36;
const NOTE_D3: u8 = 38;
const VELOCITY_DEFAULT: u8 = 100;
const VELOCITY_OFF: u8 = 64;

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

fn main() -> Result<()> {
    let command = &std::env::args().nth(1).unwrap();
    let file = &std::env::args().nth(2).unwrap();
    match command as &str {
        "print" => print_midi(MIDI::from_path(file)),
        "generate" => {
            let midi = generate_midi();
            midi.save(file);
        }
        _ => println!("Unknown command `{}`", command),
    }
    Ok(())
}
