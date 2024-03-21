#[derive(Debug)]
pub struct ChessPosition { 
    rank: i32,
    file: i32,

}


#[derive(Debug)]
pub struct Queen { 
    position: ChessPosition,
}



impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        if ChessPosition::new(position.rank, position.file).is_none() {
            panic!("Invalid position for Queen");
        }
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.rank == other.position.rank
            || self.position.file == other.position.file
            || (self.position.rank - other.position.rank).abs()
                == (self.position.file - other.position.file).abs()
        {
            true
        } else {
            false
        }
    }
}
