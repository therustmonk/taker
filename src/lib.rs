/// Takes the value with replacing it to the `Default` value.
pub trait Taker {
    /// Takes the current value.
    fn take_it(&mut self) -> Self;
    /// Set a new value.
    fn set_it(&mut self, new_value: Self);
}

impl<T: Default> Taker for T {
    fn take_it(&mut self) -> Self {
        let mut def = T::default();
        std::mem::swap(self, &mut def);
        def
    }

    fn set_it(&mut self, new_value: Self) {
        *self = new_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_own() {
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
        assert_eq!(fsm, Fsm::State1);
        assert_eq!(fsm.take_it(), Fsm::State1);
        assert_eq!(fsm, Fsm::Transition);
        fsm.set_it(Fsm::State2);
        assert_eq!(fsm, Fsm::State2);
    }

    #[test]
    fn test_option() {
        let mut opt = Some(10);
        assert_eq!(opt.take_it(), Some(10));
        assert_eq!(opt, None);
    }
}
