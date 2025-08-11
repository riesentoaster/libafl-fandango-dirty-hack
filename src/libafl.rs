use libafl::{generators::Generator, inputs::BytesInput};
use libafl_bolts::ErrorBacktrace;

use crate::fandango::FandangoPythonModule;

pub struct FandangoGenerator {
    fandango: FandangoPythonModule,
}

impl FandangoGenerator {
    pub fn new(fandango: FandangoPythonModule) -> Self {
        Self { fandango }
    }
}

impl<S> Generator<BytesInput, S> for FandangoGenerator {
    fn generate(&mut self, _state: &mut S) -> Result<BytesInput, libafl::Error> {
        let input = self
            .fandango
            .next_input()
            .map_err(|e| libafl_bolts::Error::IllegalState(e.to_string(), ErrorBacktrace::new()))?;
        Ok(BytesInput::new(input))
    }
}
