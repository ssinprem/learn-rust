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
            _ => unreachable!()
        }
    }

impl Dna {
    const NUECLEOSET : &str = "GCTA";

    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(n) = dna.chars().position(|c| ! Self::NUECLEOSET.contains(c)) {
            Err(n)
        } else {
            Ok (Self (dna.to_string()))
        }
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
    const NUECLEOSET : &str = "CGAU";

    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(n) = rna.chars().position(|c| ! Self::NUECLEOSET.contains(c)) {
            Err(n)
        } else {
            Ok (Self (rna.to_string()))
        }
    }
}
