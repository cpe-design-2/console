
type Pck = String;


#[derive(Debug, PartialEq)]
pub struct Game {
    pck: Pck,
}

impl Game {
    pub fn new(pck: Pck) -> Self{
        Self {
            pck: pck
        }
    }

    pub fn get_pck(&self) -> &Pck {
        &self.pck
    }
}