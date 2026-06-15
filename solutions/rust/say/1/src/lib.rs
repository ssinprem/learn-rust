pub fn encode(n: u64) -> String {
    fn lower_20(num: u64) -> &'static str {
        match num {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => unreachable!()
        }
    }
    
    fn tens_word(tens: u64) -> &'static str {
        match tens {
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => unreachable!(),
        }
    }

    fn units_pow6(pow3: u32) -> &'static str {
        match pow3 {
            1 => "thousand",
            2 => "million",
            3 => "billion",
            4 => "trillion",
            5 => "quadrillion",
            6 => "quintillion",
            _ => unreachable!()
        }
    }

    match n {
        0..=19 => lower_20(n).to_string(),
        20..=99 => {
            if n.is_multiple_of(10) {
                tens_word(n / 10).to_string()
            } else {
                format!("{}-{}", tens_word(n / 10), encode(n % 10))
            }
        }
        100..=999 => {
            if n.is_multiple_of(100) {
                format!("{} hundred", encode(n / 100))
            } else {
                format!("{} hundred {}", encode(n / 100), encode(n % 100))
            }
        }
        n => {
            let p = (1..=6_u32).rev()
                .find(|p| n >= 1000_u64.pow(*p)).unwrap();
            if n.is_multiple_of(1000_u64.pow(p)) {
                format!("{} {}", encode(n / 1000_u64.pow(p)), units_pow6(p))
            } else {
                format!("{} {} {}", encode(n / 1000_u64.pow(p)), units_pow6(p), encode(n % 1000_u64.pow(p)))
            }
        }
    }
}
