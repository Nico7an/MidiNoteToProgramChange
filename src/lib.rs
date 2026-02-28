use nih_plug::prelude::*;
use std::sync::Arc;

/// A VST3/CLAP plugin that converts incoming MIDI note-on events into
/// MIDI Program Change messages.
///
/// Mapping: note number → program number
///   C0  (note 0)  → Program Change 0
///   C#0 (note 1)  → Program Change 1
///   D0  (note 2)  → Program Change 2
///   …up to note 99 (D#8) → Program Change 99
///
/// Note-off events are silently consumed (Program Change has no "off").
/// All other MIDI events (CCs, pitch bend, etc.) are passed through unchanged.
struct MidiNoteToPc {
    params: Arc<MidiNoteToPcParams>,
}

#[derive(Params)]
struct MidiNoteToPcParams {
    /// The MIDI channel to send Program Change messages on.
    /// When set to 0, uses the same channel as the incoming note.
    /// Values 1–16 force output to that specific channel.
    #[id = "channel"]
    pub output_channel: IntParam,

    /// Maximum note number to convert. Notes above this are ignored.
    /// Default: 99 (as per spec), max: 127.
    #[id = "max_note"]
    pub max_note: IntParam,

    /// Whether to pass through non-note MIDI events (CCs, pitch bend, etc.)
    #[id = "passthrough"]
    pub pass_through: BoolParam,
}

impl Default for MidiNoteToPcParams {
    fn default() -> Self {
        Self {
            output_channel: IntParam::new(
                "Output Channel",
                0, // 0 = follow input channel
                IntRange::Linear { min: 0, max: 16 },
            )
            .with_unit("")
            .with_value_to_string(Arc::new(|value| {
                if value == 0 {
                    "Auto".to_string()
                } else {
                    format!("Ch {}", value)
                }
            })),

            max_note: IntParam::new(
                "Max Note",
                99,
                IntRange::Linear { min: 0, max: 127 },
            ),

            pass_through: BoolParam::new("Pass Through Other MIDI", true),
        }
    }
}

impl Default for MidiNoteToPc {
    fn default() -> Self {
        Self {
            params: Arc::new(MidiNoteToPcParams::default()),
        }
    }
}

impl Plugin for MidiNoteToPc {
    const NAME: &'static str = "MIDI Note to Program Change";
    const VENDOR: &'static str = "Nico";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // No audio I/O — this is a pure MIDI effect
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];

    // Accept and output MIDI (including CCs, pitch bend, etc.)
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
        let output_ch = self.params.output_channel.value() as u8;
        let max_note = self.params.max_note.value() as u8;
        let pass_through = self.params.pass_through.value();

        while let Some(event) = context.next_event() {
            match event {
                // ── Note On → Program Change ──────────────────────────
                NoteEvent::NoteOn {
                    timing,
                    channel,
                    note,
                    ..
                } => {
                    // Only convert notes within the configured range
                    if note <= max_note {
                        let ch = if output_ch == 0 {
                            channel // follow the incoming channel
                        } else {
                            output_ch - 1 // user picks 1–16, nih-plug uses 0–15
                        };

                        context.send_event(NoteEvent::MidiProgramChange {
                            timing,
                            channel: ch,
                            program: note,
                        });
                    }
                    // Note is consumed — not forwarded
                }

                // ── Note Off → silently consumed ──────────────────────
                NoteEvent::NoteOff { .. } => {
                    // Program Change has no "off" concept — just drop it
                }

                // ── Everything else → pass through (if enabled) ───────
                other => {
                    if pass_through {
                        context.send_event(other);
                    }
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for MidiNoteToPc {
    const CLAP_ID: &'static str = "com.nico.midi-note-to-pc";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Converts MIDI notes to Program Change messages");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::NoteEffect, ClapFeature::Utility];
}

impl Vst3Plugin for MidiNoteToPc {
    const VST3_CLASS_ID: [u8; 16] = *b"N2PChg_Nico_v01!";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Tools];
}

nih_export_clap!(MidiNoteToPc);
nih_export_vst3!(MidiNoteToPc);
