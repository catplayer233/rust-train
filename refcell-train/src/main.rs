mod refcell_explorer;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use crate::refcell_explorer::{LimitTracker, Messenger};

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn test() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    fn test_same_time_get_mutable_ref() {
        let mock_messenger = MockMessenger::new();
        let mut ref_mut_a = mock_messenger.sent_messages.borrow_mut();
        let mut ref_mut_b = mock_messenger.sent_messages.borrow_mut();
        ref_mut_a.push(String::from("use a"));
    }
}
