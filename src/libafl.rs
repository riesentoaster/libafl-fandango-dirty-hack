use std::borrow::Cow;

use libafl::{
    corpus::CorpusId,
    executors::{Executor, ExitKind, HasObservers},
    generators::Generator,
    inputs::{BytesInput, HasTargetBytes as _},
    mutators::{MutationResult, Mutator},
    observers::RefCellValueObserver,
};
use libafl_bolts::{
    Error, ErrorBacktrace, Named,
    tuples::{Handle, MatchNameRef, RefIndexable},
};

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

pub struct FandangoParseExecutor<'a, OT> {
    fandango: FandangoPythonModule,
    num_parses_observer: Handle<RefCellValueObserver<'a, u32>>,
    observers: OT,
}

impl<'a, OT> FandangoParseExecutor<'a, OT> {
    pub fn new(
        fandango: FandangoPythonModule,
        num_parses_observer: Handle<RefCellValueObserver<'a, u32>>,
        observers: OT,
    ) -> Self {
        Self {
            fandango,
            num_parses_observer,
            observers,
        }
    }
}

impl<'a, EM, OT, S, Z> Executor<EM, BytesInput, S, Z> for FandangoParseExecutor<'a, OT>
where
    OT: MatchNameRef,
{
    fn run_target(
        &mut self,
        _fuzzer: &mut Z,
        _state: &mut S,
        _mgr: &mut EM,
        input: &BytesInput,
    ) -> Result<libafl::executors::ExitKind, Error> {
        let num_parses = self
            .fandango
            .parse_input(&input.target_bytes())
            .map_err(|e| Error::IllegalState(e.to_string(), ErrorBacktrace::new()))?;

        self.observers
            .get_mut(&self.num_parses_observer)
            .ok_or(Error::IllegalState(
                "num_parses_observer not found".to_string(),
                ErrorBacktrace::new(),
            ))?
            .set(num_parses);
        Ok(ExitKind::Ok)
    }
}

impl<'a, OT> HasObservers for FandangoParseExecutor<'a, OT>
where
    OT: MatchNameRef,
{
    type Observers = OT;

    fn observers(&self) -> RefIndexable<&Self::Observers, Self::Observers> {
        RefIndexable::from(&self.observers)
    }

    fn observers_mut(&mut self) -> RefIndexable<&mut Self::Observers, Self::Observers> {
        RefIndexable::from(&mut self.observers)
    }
}
