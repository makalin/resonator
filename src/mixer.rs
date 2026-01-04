#[derive(Clone, Debug)]
pub struct TrackMixer {
    pub volume: f32,
    pub pan: f32,
    pub muted: bool,
    pub solo: bool,
}

pub struct Mixer {
    tracks: Vec<TrackMixer>,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
        }
    }

    pub fn add_track(&mut self) {
        self.tracks.push(TrackMixer {
            volume: 1.0,
            pan: 0.0,
            muted: false,
            solo: false,
        });
    }

    pub fn get_track(&self, index: usize) -> Option<&TrackMixer> {
        self.tracks.get(index)
    }

    pub fn get_track_mut(&mut self, index: usize) -> Option<&mut TrackMixer> {
        self.tracks.get_mut(index)
    }

    pub fn set_volume(&mut self, track_index: usize, volume: f32) {
        if let Some(track) = self.get_track_mut(track_index) {
            track.volume = volume.max(0.0).min(1.0);
        }
    }

    pub fn set_pan(&mut self, track_index: usize, pan: f32) {
        if let Some(track) = self.get_track_mut(track_index) {
            track.pan = pan.max(-1.0).min(1.0);
        }
    }

    pub fn tracks(&self) -> &[TrackMixer] {
        &self.tracks
    }

    pub fn set_tracks(&mut self, tracks: Vec<TrackMixer>) {
        self.tracks = tracks;
    }
}

