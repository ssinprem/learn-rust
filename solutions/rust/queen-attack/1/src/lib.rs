#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen ( ChessPosition );

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if ! (0..=7).contains(&rank) ||
           ! (0..=7).contains(&file) {
            None
        } else {
            Some(Self{ rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self ( position )
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.0.rank == other.0.rank {             // Same Row
            true
        } else if self.0.file == other.0.file {      // Same Column
            true
        } else if (self.0.rank - other.0.rank).abs() == 
                  (self.0.file - other.0.file).abs() { // Diagonal
            true
        } else {                                     // Not match any case
            false
        }
    }
}
