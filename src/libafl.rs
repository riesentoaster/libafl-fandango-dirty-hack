use std::borrow::Cow;

use libafl::{
    corpus::CorpusId,
    generators::Generator,
    inputs::BytesInput,
    mutators::{MutationResult, Mutator},
};
use libafl_bolts::{Error, ErrorBacktrace, Named};

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
    fn generate(&mut self, _state: &mut S) -> Result<BytesInput, Error> {
        let input = self
            .fandango
            .next_input()
            .map_err(|e| Error::IllegalState(e.to_string(), ErrorBacktrace::new()))?;
        Ok(input.into())
    }
}

pub struct FandangoPseudoMutator {
    fandango: FandangoPythonModule,
}

impl FandangoPseudoMutator {
    pub fn new(fandango: FandangoPythonModule) -> Self {
        Self { fandango }
    }
}

impl<S> Mutator<BytesInput, S> for FandangoPseudoMutator {
    fn mutate(&mut self, _state: &mut S, input: &mut BytesInput) -> Result<MutationResult, Error> {
        let new_input = self
            .fandango
            .next_input()
            .map_err(|e| Error::IllegalState(e.to_string(), ErrorBacktrace::new()))?;
        *input = BytesInput::new(new_input);
        Ok(MutationResult::Mutated)
    }

    fn post_exec(&mut self, _state: &mut S, _new_corpus_id: Option<CorpusId>) -> Result<(), Error> {
        Ok(())
    }
}

impl Named for FandangoPseudoMutator {
    fn name(&self) -> &Cow<'static, str> {
        &Cow::Borrowed("FandangoPseudoMutator")
    }
}
