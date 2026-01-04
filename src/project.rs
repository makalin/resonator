use crate::mixer::Mixer;
use crate::sequencer::Sequencer;
use crate::transport::Transport;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct ProjectData {
    sequencer: SequencerData,
    mixer: MixerData,
    transport: TransportData,
}

#[derive(Serialize, Deserialize)]
struct SequencerData {
    tracks: Vec<TrackData>,
    playhead: u32,
}

#[derive(Serialize, Deserialize)]
struct TrackData {
    name: String,
    notes: Vec<NoteData>,
    channel: u8,
    muted: bool,
}

#[derive(Serialize, Deserialize)]
struct NoteData {
    pitch: u8,
    start: u32,
    length: u32,
    velocity: u8,
}

#[derive(Serialize, Deserialize)]
struct MixerData {
    tracks: Vec<TrackMixerData>,
}

#[derive(Serialize, Deserialize)]
struct TrackMixerData {
    volume: f32,
    pan: f32,
    muted: bool,
    solo: bool,
}

#[derive(Serialize, Deserialize)]
struct TransportData {
    bpm: f64,
    time_signature_numerator: u8,
    time_signature_denominator: u8,
    loop_start: u32,
    loop_end: u32,
}

pub struct Project {
    filename: Option<String>,
}

impl Project {
    pub fn new() -> Self {
        Self { filename: None }
    }

    pub fn save(&self, sequencer: &Sequencer, mixer: &Mixer, transport: &Transport) -> Result<()> {
        let filename = self.filename.as_deref().unwrap_or("project.res");
        
        let project_data = ProjectData {
            sequencer: SequencerData {
                tracks: sequencer
                    .tracks()
                    .iter()
                    .map(|t| TrackData {
                        name: t.name.clone(),
                        notes: t
                            .notes
                            .iter()
                            .map(|n| NoteData {
                                pitch: n.pitch,
                                start: n.start,
                                length: n.length,
                                velocity: n.velocity,
                            })
                            .collect(),
                        channel: t.channel,
                        muted: t.muted,
                    })
                    .collect(),
                playhead: sequencer.playhead(),
            },
            mixer: MixerData {
                tracks: mixer
                    .tracks()
                    .iter()
                    .map(|t| TrackMixerData {
                        volume: t.volume,
                        pan: t.pan,
                        muted: t.muted,
                        solo: t.solo,
                    })
                    .collect(),
            },
            transport: TransportData {
                bpm: transport.bpm(),
                time_signature_numerator: transport.time_signature().0,
                time_signature_denominator: transport.time_signature().1,
                loop_start: transport.loop_range().0,
                loop_end: transport.loop_range().1,
            },
        };

        let json = serde_json::to_string_pretty(&project_data)?;
        fs::write(filename, json)?;
        Ok(())
    }

    pub fn load(&mut self, sequencer: &mut Sequencer, mixer: &mut Mixer, transport: &mut Transport, filename: &str) -> Result<()> {
        let content = fs::read_to_string(filename)?;
        let project_data: ProjectData = serde_json::from_str(&content)?;

        // Load sequencer
        let tracks: Vec<crate::sequencer::Track> = project_data
            .sequencer
            .tracks
            .into_iter()
            .map(|t| crate::sequencer::Track {
                name: t.name,
                notes: t
                    .notes
                    .into_iter()
                    .map(|n| crate::sequencer::Note {
                        pitch: n.pitch,
                        start: n.start,
                        length: n.length,
                        velocity: n.velocity,
                    })
                    .collect(),
                channel: t.channel,
                muted: t.muted,
            })
            .collect();
        sequencer.set_tracks(tracks);
        sequencer.set_playhead(project_data.sequencer.playhead);

        // Load mixer
        let mixer_tracks: Vec<crate::mixer::TrackMixer> = project_data
            .mixer
            .tracks
            .into_iter()
            .map(|t| crate::mixer::TrackMixer {
                volume: t.volume,
                pan: t.pan,
                muted: t.muted,
                solo: t.solo,
            })
            .collect();
        mixer.set_tracks(mixer_tracks);

        // Load transport
        transport.set_bpm(project_data.transport.bpm);
        transport.set_time_signature(
            project_data.transport.time_signature_numerator,
            project_data.transport.time_signature_denominator,
        );
        transport.set_loop_range(
            project_data.transport.loop_start,
            project_data.transport.loop_end,
        );

        self.filename = Some(filename.to_string());
        Ok(())
    }
}

