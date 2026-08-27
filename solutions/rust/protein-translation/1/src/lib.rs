pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut list = Vec::new();
    let mut temp = rna;
    while temp.len() >= 3 {
        let (str, rem) = temp.split_at(3);
        temp = rem;
        let name = match str {
            "AUG" => "Methionine",
            "UUU" | "UUC" => "Phenylalanine",
            "UUA" | "UUG" => "Leucine",
            "UCU" | "UCC" | "UCA" | "UCG" => "Serine",
            "UAU" | "UAC" => "Tyrosine",
            "UGU" | "UGC" => "Cysteine",
            "UGG" => "Tryptophan",
            "UAA" | "UAG" | "UGA" => return Some(list),
            _ => return None
        };
        list.push(name);
    }
    if temp.len().is_multiple_of(3) {
        Some(list)
    } else {
        None
    }
}
