# taker

Make FSM on any that that has `Default` implementation.

How to use it:

```rust
#[derive(Debug, PartialEq, Eq)]
enum Fsm {
    State1,
    State2,
    Transition,
}

impl Default for Fsm {
    fn default() -> Self {
        Self::Transition
    }
}

let mut fsm = Fsm::State1;
match fsm.take() {
    Fsm::State1 => {
        fsm.set(Fsm::State2);
    }
    Fsm::State2 => {
        fsm.set(Fsm::State1);
    }
    _ => {
        panic!("Stucked in transition state...");
    }
}
```
