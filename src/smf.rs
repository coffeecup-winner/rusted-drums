use midly::{
    num::{u15, u28, u4, u7},
    Format, Header, MetaMessage, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind,
};

use crate::runtime::{self, LuaEvent};

const TICKS_PER_QUARTER_NOTE: u32 = 120;

pub fn create_from_events<'a>(events: &[LuaEvent]) -> Smf<'a> {
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

    midi
}
