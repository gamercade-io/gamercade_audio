use crate::{
    ActiveState, EnvelopeInstance, FMWaveform, OperatorDefinition, OperatorDefinitionBundle,
    WavetableOscillator, LUT_FULL_LEN, OPERATOR_COUNT,
};

#[derive(Debug, Clone)]
pub struct OperatorInstance {
    pub oscillator: WavetableOscillator,
    envelope: EnvelopeInstance,
}

impl OperatorInstance {
    /// Constructs a new operator instance based on the passed in definition
    pub fn new(source: &OperatorDefinition, output_sample_rate: usize) -> Self {
        Self {
            oscillator: WavetableOscillator::new(LUT_FULL_LEN, output_sample_rate),
            envelope: EnvelopeInstance::new(&source.envlope_definition, output_sample_rate),
        }
    }

    /// Sets the frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        self.oscillator.set_frequency(frequency);
    }

    /// Get's the current sample value including any modulation and
    /// interpolates between the next sample.
    /// Also ticks the operator.
    pub fn tick(&mut self, waveform: FMWaveform, modulation: f32, active: ActiveState) -> f32 {
        let index = self.oscillator.tick() + self.oscillator.modulation(modulation);

        let next_weight = index.fract();
        let index_weight = 1.0 - next_weight;

        let index = index as usize % LUT_FULL_LEN;
        let next = (index + 1) % LUT_FULL_LEN;

        let index = waveform.lookup(index);
        let next = waveform.lookup(next);

        let output = (index * index_weight) + (next * next_weight);
        let envelope = self.envelope.tick(active);

        output * envelope
    }
}

#[derive(Clone, Debug)]
pub struct OperatorInstanceBundle {
    pub operators: [OperatorInstance; OPERATOR_COUNT],
}

impl OperatorInstanceBundle {
    pub fn new(source: &OperatorDefinitionBundle, output_sample_rate: usize) -> Self {
        Self {
            operators: std::array::from_fn(|index| {
                OperatorInstance::new(&source.operators[index], output_sample_rate)
            }),
        }
    }
}
