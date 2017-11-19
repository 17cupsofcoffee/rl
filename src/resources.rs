pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}