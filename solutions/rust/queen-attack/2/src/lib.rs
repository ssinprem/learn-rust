#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen ( ChessPosition );

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self{ rank, file }),
            _ => None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self ( position )
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.rank == other.0.rank ||          // Same Row
        self.0.file == other.0.file ||          // Same Column
        (self.0.rank - other.0.rank).abs() ==
        (self.0.file - other.0.file).abs()      // Diagonal
    }
}
