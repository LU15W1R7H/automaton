use std::{collections::HashMap, hash::Hash, marker::PhantomData};

/// State Machine
///
/// Actually a finite-state transducer
pub struct StateMachine<State> {
  state: State,
}
impl<State> StateMachine<State> {
  pub fn new(state: State) -> Self {
    Self { state }
  }
}

/// State machine driver
pub trait Driver<Input, Output> {
  fn step(&mut self, input: Input) -> Output;
}

pub trait DriverExt<Input, Output>: Driver<Input, Output> {
  fn run<InputIterator>(&mut self, inputs: InputIterator) -> Vec<Output>
  where
    InputIterator: IntoIterator<Item = Input>,
  {
    let mut outputs = Vec::new();
    for input in inputs {
      outputs.push(self.step(input));
    }
    outputs
  }
}

/// State machine driver with transition table
///
/// Zero-cost construction
pub struct DriverTransitionTable<'a, State, Input, Output> {
  sm: &'a mut StateMachine<State>,
  tt: &'a HashMap<(State, Input), (State, Output)>,
}
impl<'a, State, Input, Output> DriverTransitionTable<'a, State, Input, Output> {
  pub fn new(
    sm: &'a mut StateMachine<State>,
    tt: &'a HashMap<(State, Input), (State, Output)>,
  ) -> Self {
    Self { sm, tt }
  }
}

impl<'a, State, Input, Output> Driver<Input, Output>
  for DriverTransitionTable<'a, State, Input, Output>
where
  Input: Hash + Eq,
  State: Copy + Hash + Eq,
  Output: Copy,
{
  fn step(&mut self, input: Input) -> Output {
    let (state, output) = self.tt.get(&(self.sm.state, input)).unwrap();
    self.sm.state = *state;
    *output
  }
}

/// State machine driver with transition function
///
/// Zero-cost construction
pub struct DriverTransitionFunction<'a, State, Input, Output, F> {
  sm: &'a mut StateMachine<State>,
  tf: &'a F,
  _input: PhantomData<Input>,
  _output: PhantomData<Output>,
}
impl<'a, State, Input, Output, F> DriverTransitionFunction<'a, State, Input, Output, F> {
  pub fn new(sm: &'a mut StateMachine<State>, tf: &'a F) -> Self {
    Self {
      sm,
      tf,
      _input: PhantomData,
      _output: PhantomData,
    }
  }
}

impl<'a, State, Input, Output, F> Driver<Input, Output>
  for DriverTransitionFunction<'a, State, Input, Output, F>
where
  State: Copy,
  F: Fn(State, Input) -> (State, Output),
{
  fn step(&mut self, input: Input) -> Output {
    let (state, output) = (self.tf)(self.sm.state, input);
    self.sm.state = state;
    output
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn turnstile_transition_table() {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    enum State {
      Locked,
      Unlocked,
    }
    #[derive(PartialEq, Eq, Hash)]
    enum Input {
      Push,
      Coin,
    }
    let transition_table = HashMap::from([
      ((State::Locked, Input::Push), (State::Locked, State::Locked)),
      (
        (State::Locked, Input::Coin),
        (State::Unlocked, State::Unlocked),
      ),
      (
        (State::Unlocked, Input::Coin),
        (State::Unlocked, State::Unlocked),
      ),
      (
        (State::Unlocked, Input::Push),
        (State::Locked, State::Locked),
      ),
    ]);

    let mut state_machine = StateMachine::new(State::Locked);
    let mut driver = DriverTransitionTable::new(&mut state_machine, &transition_table);
    assert_eq!(driver.step(Input::Coin), State::Unlocked);
    assert_eq!(driver.step(Input::Push), State::Locked);
  }

  #[test]
  fn turnstile_transition_function() {
    #[derive(Debug, Copy, Clone, PartialEq)]
    enum State {
      Locked,
      Unlocked,
    }
    enum Input {
      Push,
      Coin,
    }
    fn transition_function(state: State, input: Input) -> (State, State) {
      match (state, input) {
        (State::Locked, Input::Push) => (State::Locked, State::Locked),
        (State::Locked, Input::Coin) => (State::Unlocked, State::Unlocked),
        (State::Unlocked, Input::Coin) => (State::Unlocked, State::Unlocked),
        (State::Unlocked, Input::Push) => (State::Locked, State::Locked),
      }
    }

    let mut state_machine = StateMachine::new(State::Locked);
    let mut driver = DriverTransitionFunction::new(&mut state_machine, &transition_function);
    assert_eq!(driver.step(Input::Coin), State::Unlocked);
    assert_eq!(driver.step(Input::Push), State::Locked);
  }
}
