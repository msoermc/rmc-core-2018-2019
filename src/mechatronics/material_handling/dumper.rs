pub struct Dumper {
    is_enabled: bool
}

impl Dumper {
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn dump(&mut self) {
        unimplemented!()
    }

    pub fn reset(&mut self) {
        unimplemented!()
    }

    pub fn run_cycle(&mut self) {
        unimplemented!()
    }
}