mod envelope;
mod instruments;
mod notes;
mod playback;
mod sound_engine;
mod sound_engine2;
mod sound_rom;
mod sound_rom_instance;
mod tracker;
mod wavetable_oscillator;

pub use envelope::*;
pub use instruments::*;
pub use notes::*;
pub use playback::*;
pub use sound_engine::*;
pub use sound_engine2::*;
pub use sound_rom::*;
pub use sound_rom_instance::*;
pub use tracker::*;
pub use wavetable_oscillator::*;

fn initialize_globals() {
    init_fm_lut();
    initialize_notes();
    unsafe {
        NO_SOUND_DEFINITION.write(std::sync::Arc::new(WavetableDefinition {
            data: Box::new([0, 0]),
            envelope: EnvelopeDefinition::default(),
        }));
    }
}
