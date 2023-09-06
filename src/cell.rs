#[derive(Default, Debug)]

pub struct Cell {
    pub code: String,
}

impl Cell {
    pub fn new(code: String) -> Self {
        return Self {code};
    }
}
