pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(
            (0..row_count).fold( Vec::<Vec<u32>>::new(),
                |mut rows , row | {
                    rows.push(
                        match row {
                            0 => vec![1],
                            _ => {
                                [1_u32].iter().chain(
                                    &rows.last().unwrap().windows(2)
                                        .map(|a| a.iter().sum::<u32>())
                                        .collect::<Vec<u32>>()
                                    )
                                    .chain(&[1_u32]).copied()
                                    .collect::<Vec<u32>>()
                            }
                        }
                    );
                    rows
                }
            )
        )
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
