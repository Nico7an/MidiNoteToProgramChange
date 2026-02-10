use nih_plug::prelude::*;
use std::sync::Arc;

struct MidiNoteToPc {
    params: Arc<MidiNoteToPcParams>,
}

#[derive(Params)]
struct MidiNoteToPcParams {}

impl Default for MidiNoteToPc {
    fn default() -> Self {
        Self {
            params: Arc::new(MidiNoteToPcParams::default()),
        }
    }
}

impl Default for MidiNoteToPcParams {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for MidiNoteToPc {
    const NAME: &'static str = "Midi Note to Program Change";
    const VENDOR: &'static str = "Nico7an";
    const URL: &'static str = "https://github.com/nico7an";
    const EMAIL: &'static str = "nico7an@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // Keep Audio I/O for host stability (Ableton compatibility)
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        const BASE_NOTE: u8 = 24; // C0

        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { timing, channel, note, .. } => {
                    // Logic: PC = Note - BaseNote
                    // Only if Note >= BaseNote (C0) and Note <= BaseNote + 99
                    if note >= BASE_NOTE {
                        let pc_val = note - BASE_NOTE;
                        if pc_val <= 99 {
                            context.send_event(NoteEvent::MidiProgramChange {
                                timing,
                                channel,
                                program: pc_val,
                            });
                        }
                    }
                    // Filter out all other Notes (no pass-through)
                }
                NoteEvent::NoteOff { .. } => {
                    // Ignore NoteOff
                }
                _ => {
                    // Pass through other events (CC, PitchBend, etc.)
                    context.send_event(event);
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for MidiNoteToPc {
    const CLAP_ID: &'static str = "com.nico7an.midi-note-to-pc";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Converts MIDI Notes to Program Change messages");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::NoteEffect,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for MidiNoteToPc {
    const VST3_CLASS_ID: [u8; 16] = *b"NoteToPrgChgGraB";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Tools,
    ];
}

nih_export_clap!(MidiNoteToPc);
nih_export_vst3!(MidiNoteToPc);
