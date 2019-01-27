pub struct BucketLadder {
    is_enabled: bool
}

impl BucketLadder {
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn raise(&mut self) {
        unimplemented!()
    }

    pub fn lower(&mut self) {
        unimplemented!()
    }

    pub fn run_cycle(&mut self) {
        unimplemented!()
    }
}