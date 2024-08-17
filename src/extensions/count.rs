use std::{thread, time::Duration};

pub struct Count {
    def_value: i32,
    value: i32,
}

impl Count {
    pub fn new(value: i32) -> Count {
        Count {
            def_value: value,
            value,
        }
    }
    pub fn count_down(&mut self) -> () {
        println!("{}", self.value);
        thread::sleep(Duration::from_secs(1));
        if self.value > 0 {
            self.value -= 1;
            self.count_down();
        }
    }
    pub fn count_up(&mut self) -> () {
        println!("{}", self.value);
        thread::sleep(Duration::from_secs(1));
        if self.value < self.def_value {
            self.value += 1;
            self.count_up();
        }
    }
}
