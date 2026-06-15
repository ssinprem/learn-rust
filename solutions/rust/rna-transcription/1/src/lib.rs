#[derive(Debug, PartialEq, Eq)]
pub struct Dna (String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna (String);

pub fn transcribe(c: char) -> char {
        match c {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => '?'
        }
    }

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (n,c) in dna.chars().enumerate() {
            if ! "GCTA".contains(c) {
                return Err(n)
            }
        }
        Ok (Self (dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(self.0.chars()
            .map(transcribe)
            .collect::<String>()
            .as_str()
        ).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (n,c) in rna.chars().enumerate() {
            if ! "CGAU".contains(c) {
                return Err(n)
            }
        }
        Ok (Self (rna.to_string()))
    }
}
