pub trait Messenger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    max: usize,
    value: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            max,
            value: 0,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let used_percent = self.value as f64 / self.max as f64;

        if used_percent >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if used_percent >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if used_percent >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        };
    }
}