pub struct Allergies (u32);

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

fn allergen_index( allergen: &Allergen) -> u32 {
    match allergen {
        Allergen::Eggs => 0,
        Allergen::Peanuts => 1,
        Allergen::Shellfish => 2,
        Allergen::Strawberries => 3,
        Allergen::Tomatoes => 4,
        Allergen::Chocolate => 5,
        Allergen::Pollen => 6,
        Allergen::Cats => 7,
    }
}

fn index_allergen( index: u32) -> Option<Allergen> {
    match index {
        0 => Some(Allergen::Eggs),
        1 => Some(Allergen::Peanuts),
        2 => Some(Allergen::Shellfish),
        3 => Some(Allergen::Strawberries),
        4 => Some(Allergen::Tomatoes),
        5 => Some(Allergen::Chocolate),
        6 => Some(Allergen::Pollen),
        7 => Some(Allergen::Cats),
        _ => None
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self (score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.0 >> allergen_index(allergen) ) % 2 == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result = Vec::new();
        for i in 0..8u32 {
            if (self.0 >> i) % 2 == 1 {
                result.push(index_allergen(i).unwrap())
            }
        }
        result
    }
}
