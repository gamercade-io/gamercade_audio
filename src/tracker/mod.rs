mod chain;
mod effect;
mod instrument_definition;
mod phrase;
mod song;

pub use chain::*;
pub use effect::*;
pub use instrument_definition::*;
pub use phrase::*;
pub use song::*;

pub const SONG_TRACK_CHANNELS: usize = 8;
pub const EFFECT_COUNT: usize = 3;

// Maximum allowed phrases in a chain
pub const CHAIN_MAX_PHRASE_COUNT: usize = 16;

// Maximum allowed entries in a phrase
pub const PHRASE_MAX_ENTRIES: usize = 16;

pub type PhraseVolumeType = u8;
