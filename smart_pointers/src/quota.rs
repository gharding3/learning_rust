
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct Quota<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> Quota<'a, T> 
where T : Messenger
{
    pub fn new(messenger: &T, max: usize) -> Quota<T> {
        Quota {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        self.check_value();
    }

    fn check_value(&self) {
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error, you are over the quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Warning, you are at over 90% of your quota!");
        }
    }
}