use std::borrow::Cow;

use super::types;

use crate::{
    gp::Song,
    track::Track,
    measure::Measure,
    beat::{Beat, Voice},
    note::Note,
    key_signature::Duration
};

fn note_to_note(note: &types::Note) -> Note {
    let mut rnote = Note::default();

    for p in note.properties() {
        match p {
            types::NoteProperty::String(n) => {rnote.string = *n as i8;} // TODO: safety?
            types::NoteProperty::Fret(n) => {rnote.value = *n as i16;}
            _ => ()
        }
    }
    rnote
}

fn beat_to_beat(beat: &types::Beat, gpif: &types::GPIF) -> Beat {
    let ridx = beat.rhythm_ref.rhythm;
    let rhythm = &gpif.rhythms.rythms[ridx as usize];

    let mut d = Duration {
        value: match rhythm.note_value.as_ref() {
            "Whole" => 0,
            "Half" => 1,
            "Quarter" => 2,
            "Eighth" => 3,
            "16th" => 4,
            "32nd" => 5,
            nv => {
                log::error!("Unsupported note value: {:?}", nv);
                panic!()
            }
        },
        dotted: rhythm.augmentation_dot.as_ref()
            .map(|ad| ad.count == 1)
            .or(Some(false))
            .unwrap(),
        double_dotted: rhythm.augmentation_dot.as_ref()
            .map(|ad| ad.count == 2)
            .or(Some(false))
            .unwrap(),
        ..Default::default()
    };

    rhythm.primary_tuplet.iter()
        .for_each(|types::PrimaryTuplet {num, den}| {
            d.tuplet_enters = *num; // Correct?
            d.tuplet_times = *den;
        });

    Beat {
        duration: d,
        notes: beat.notes
            .as_ref()
            .map_or(Vec::<Note>::new(), |notes| notes.vec.iter()
                    .map(|note_id|
                         note_to_note(&gpif.notes.notes[*note_id as usize])
                    )
                    .collect()
            ),
        ..Default::default()
    }
}

fn voice_to_voice(voice: &types::Voice, gpif: &types::GPIF) -> Voice {
    Voice {
        beats: voice.beats.vec.iter()
            .map(|beat_id| beat_to_beat(&gpif.beats.beats[*beat_id as usize], gpif))
            .collect(),
        ..Default::default()
    }
}

fn bar_to_measure(bar: &types::Bar, gpif: &types::GPIF) -> Measure {
    Measure {
        voices: bar.voices.vec.iter()
            .filter_map(|voice_id| {
                if *voice_id < 0i16 {
                    None
                } else {
                    let idx = usize::try_from(*voice_id).unwrap(); // Known to be >= 0
                    Some(voice_to_voice(&gpif.voices.voices[idx], gpif))
                }
            })
            .collect(),
        ..Default::default()
    }
}

fn masterbars_to_tracks(masterbars: &types::MasterBars, gpif: &types::GPIF) -> Vec<Track> {
    let num_tracks = masterbars.master_bars[0].bars.len();

    let mut tracks = Vec::<Track>::with_capacity(num_tracks);

    for track_index in 0..num_tracks {
        let bar_iterator = masterbars.master_bars.iter().map(|mb| mb.bars.vec[track_index]);
        let track = Track {
            measures: bar_iterator
                .map(|bar_index| bar_to_measure(&gpif.bars.bars[bar_index as usize], gpif))
                .collect(),
            ..Default::default()
        };
        tracks.push(track);
    }

    tracks
}

fn gpif_to_tracks(gpif: &types::GPIF) -> Vec<Track> {
    masterbars_to_tracks(&gpif.master_bars, gpif)
}

pub fn gpif_to_song(gpif: &types::GPIF) -> Song {
    Song {
        artist: gpif.score.artist.to_string(),
        name: gpif.score.title.to_string(),
        album: gpif.score.album.to_string(),
        author: gpif.score.music.to_string(),
        words: gpif.score.words.to_string(),
        copyright: gpif.score.copyright.to_string(),
        transcriber: gpif.score.tabber.to_string(),
        instructions: gpif.score.instructions.to_string(),
        notice: gpif.score.notices.iter().map(Cow::to_string).collect(),
        tracks: gpif_to_tracks(&gpif),
        ..Default::default()
    }
}
