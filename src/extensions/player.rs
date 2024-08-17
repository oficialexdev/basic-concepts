pub struct Player {
    name: String,
    pub number: i32,
}

impl Player {
    pub fn new(name: String, number: i32) -> Player {
        return Player { name, number };
    }
    pub fn to_string(&self) -> String {
        return format!("Player: {}, Number: {};", self.name, self.number);
    }
}
