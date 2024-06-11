pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("ERROR: You went over")
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("DANGER WARNING: Almost going over")
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("WARNING: Watch out for quota")
        }
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use super::*;
    struct MockMessenger {
        sent_message: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_message: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_message.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn sends_an_over_75_perc_msg() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_message.borrow().len(), 1);
    }
}
