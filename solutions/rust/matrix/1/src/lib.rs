pub struct Matrix (Vec<Vec<u32>>);

impl Matrix {
    pub fn new(input: &str) -> Self {
        Self (
            input.lines()
            .map( |r| {
                r.split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap_or(0_u32))
                    .collect::<Vec<u32>>()
            })
            .collect()
        )
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.0.get(row_no-1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        match self.0.iter().map(|row| row.get(col_no-1))
        {
            col if col.clone().any(|n| n.is_none()) => None,
            col => Some(col.map(|n| *n.unwrap()).collect::<Vec<u32>>())
        }
    }
}
