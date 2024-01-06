pub struct Player {
    score: i64,
    cleared_tally: usize,
    final_tally_str: String,
}

impl Player {
    pub fn new() -> Self {
        Self {
            score: 0,
            cleared_tally: 0,
            final_tally_str: String::new(),
        }
    }

    pub fn get_score(&self) -> i64 {
        return self.score;
    }

    pub fn get_cleared(&self) -> usize {
        return self.cleared_tally;
    }

    pub fn cleared_as_doormoji(&mut self) -> &str {
        let demoji: &str = "\u{1F6AA}";
        for _ in 0..self.cleared_tally {
            self.final_tally_str.push_str(demoji)
        }

        return &self.final_tally_str as &str;
    }

    pub fn increase_score(&mut self, value: i64) {
        self.score += value
    }

    pub fn incr_cleared_tally(&mut self) {
        self.cleared_tally += 1
    }
}
