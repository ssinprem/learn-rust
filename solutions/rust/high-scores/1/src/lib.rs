#[derive(Debug)]
pub struct HighScores {
    list : Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            list: scores.to_vec()
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.list.iter().as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.list.iter().last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.list.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.list.is_empty() {
            return vec![];
        }
        let mut rank = self.list.clone();
        rank.dedup();
        rank.sort();
        rank.reverse();
        if self.list.len() < 3 {
            rank.to_vec()
        } else {
            rank.split_at(3).0.to_vec()
        }
    }
}
