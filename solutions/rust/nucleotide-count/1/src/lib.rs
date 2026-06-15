use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    
    if ! "ACGT".contains(nucleotide) {
        return Err(nucleotide);
    }

    let invalid = dna.chars().filter(|c| ! "ACGT".contains(*c)).collect::<Vec<char>>();
    if ! invalid.is_empty() {
        return Err(*invalid.first().unwrap());
    }

    let count = dna.chars().filter(|c| *c == nucleotide).count();
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hs = HashMap::new();
    for c in "ACGT".chars() {
        hs.insert(c , count(c, dna)?);
    }
    Ok(hs)
}
