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
}