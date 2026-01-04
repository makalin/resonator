#[derive(Clone, Debug)]
pub struct Step {
    pub active: bool,
    pub velocity: u8,
    pub pitch: u8,
}

impl Step {
    pub fn new() -> Self {
        Self {
            active: false,
            velocity: 100,
            pitch: 60, // Middle C
        }
    }
}

#[derive(Clone, Debug)]
pub struct StepTrack {
    pub name: String,
    pub steps: Vec<Step>,
    pub pitch: u8,
    pub channel: u8,
    pub muted: bool,
}

impl StepTrack {
    pub fn new(name: String, num_steps: usize) -> Self {
        Self {
            name,
            steps: vec![Step::new(); num_steps],
            pitch: 60,
            channel: 0,
            muted: false,
        }
    }
}

pub struct StepSequencer {
    tracks: Vec<StepTrack>,
    selected_track: usize,
    selected_step: Option<usize>,
    num_steps: usize,
    current_step: usize,
    steps_per_beat: usize,
}

impl StepSequencer {
    pub fn new(num_steps: usize, steps_per_beat: usize) -> Self {
        Self {
            tracks: Vec::new(),
            selected_track: 0,
            selected_step: None,
            num_steps,
            current_step: 0,
            steps_per_beat,
        }
    }

    pub fn add_track(&mut self, name: String) {
        self.tracks.push(StepTrack::new(name, self.num_steps));
    }

    pub fn tracks(&self) -> &[StepTrack] {
        &self.tracks
    }

    pub fn tracks_mut(&mut self) -> &mut Vec<StepTrack> {
        &mut self.tracks
    }

    pub fn selected_track(&self) -> usize {
        self.selected_track
    }

    pub fn select_track(&mut self, idx: usize) {
        if idx < self.tracks.len() {
            self.selected_track = idx;
        }
    }

    pub fn toggle_step(&mut self, track_idx: usize, step_idx: usize) {
        if let Some(track) = self.tracks.get_mut(track_idx) {
            if step_idx < track.steps.len() {
                track.steps[step_idx].active = !track.steps[step_idx].active;
            }
        }
    }

    pub fn set_step_velocity(&mut self, track_idx: usize, step_idx: usize, velocity: u8) {
        if let Some(track) = self.tracks.get_mut(track_idx) {
            if step_idx < track.steps.len() {
                track.steps[step_idx].velocity = velocity;
            }
        }
    }

    pub fn set_step_pitch(&mut self, track_idx: usize, step_idx: usize, pitch: u8) {
        if let Some(track) = self.tracks.get_mut(track_idx) {
            if step_idx < track.steps.len() {
                track.steps[step_idx].pitch = pitch;
            }
        }
    }

    pub fn current_step(&self) -> usize {
        self.current_step
    }

    pub fn advance_step(&mut self) {
        self.current_step = (self.current_step + 1) % self.num_steps;
    }

    pub fn reset(&mut self) {
        self.current_step = 0;
    }

    pub fn num_steps(&self) -> usize {
        self.num_steps
    }

    pub fn steps_per_beat(&self) -> usize {
        self.steps_per_beat
    }

    pub fn set_num_steps(&mut self, num_steps: usize) {
        self.num_steps = num_steps;
        for track in &mut self.tracks {
            track.steps.resize(num_steps, Step::new());
        }
    }

    pub fn convert_to_notes(&self, ticks_per_beat: u32) -> Vec<(usize, crate::sequencer::Note)> {
        let mut notes = Vec::new();
        let ticks_per_step = ticks_per_beat / self.steps_per_beat as u32;

        for (track_idx, track) in self.tracks.iter().enumerate() {
            for (step_idx, step) in track.steps.iter().enumerate() {
                if step.active {
                    notes.push((
                        track_idx,
                        crate::sequencer::Note {
                            pitch: step.pitch,
                            start: (step_idx as u32) * ticks_per_step,
                            length: ticks_per_step,
                            velocity: step.velocity,
                        },
                    ));
                }
            }
        }

        notes
    }
}

