use std::marker::PhantomData;

/// State Machine
///
/// Actually a finit-state transducer
pub struct Sm<State, Input, Output, Transition>
where
  Transition: Fn(State, Input) -> (State, Output),
{
  state: State,
  transition: Transition,
  _input: PhantomData<Input>,
  _output: PhantomData<Output>,
}

impl<State, Input, Output, Transition> Sm<State, Input, Output, Transition>
where
  Transition: Fn(State, Input) -> (State, Output),
{
  pub fn new(state: State, transition: Transition) -> Self {
    Self {
      state,
      transition,
      _input: PhantomData,
      _output: PhantomData,
    }
  }

  pub fn step(&mut self, input: Input) -> Output
  where
    State: Copy,
  {
    let (state, output) = (self.transition)(self.state, input);
    self.state = state;
    output
  }

  pub fn run<InputIterator>(&mut self, inputs: InputIterator) -> Vec<Output>
  where
    State: Copy,
    InputIterator: IntoIterator<Item = Input>,
  {
    let mut outputs = Vec::new();
    for input in inputs {
      outputs.push(self.step(input));
    }
    outputs
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn turnstile_sm() {
    #[derive(Debug, Copy, Clone, PartialEq)]
    enum State {
      Locked,
      Unlocked,
    }
    enum Input {
      Push,
      Coin,
    }
    fn transition(state: State, input: Input) -> (State, State) {
      match (state, input) {
        (State::Locked, Input::Push) => (State::Locked, State::Locked),
        (State::Locked, Input::Coin) => (State::Unlocked, State::Unlocked),
        (State::Unlocked, Input::Coin) => (State::Unlocked, State::Unlocked),
        (State::Unlocked, Input::Push) => (State::Locked, State::Locked),
      }
    }

    let mut turnstile = Sm::new(State::Locked, transition);
    assert_eq!(turnstile.step(Input::Coin), State::Unlocked);
    assert_eq!(turnstile.step(Input::Push), State::Locked);
  }
}
