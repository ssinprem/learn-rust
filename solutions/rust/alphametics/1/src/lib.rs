use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u32>> {
    // Split the input into left and right sides, handling both '=' and '=='
    let sides: Vec<&str> = input.split('=').filter(|s| !s.is_empty()).collect();
    if sides.len() != 2 {
        return None;
    }

    let mut weights = HashMap::new();
    let mut leading_chars = HashSet::new();

    // Process the left side (positive weights)
    for word in sides[0].split('+').map(|s| s.trim()) {
        if word.is_empty() { continue; }
        let chars: Vec<char> = word.chars().collect();
        
        // The leading digit of a multi-digit number cannot be zero
        if chars.len() > 1 {
            leading_chars.insert(chars[0]);
        }
        
        for (i, &ch) in chars.iter().rev().enumerate() {
            *weights.entry(ch).or_insert(0i64) += 10i64.pow(i as u32);
        }
    }

    // Process the right side (negative weights)
    for word in sides[1].split('+').map(|s| s.trim()) {
        if word.is_empty() { continue; }
        let chars: Vec<char> = word.chars().collect();
        
        if chars.len() > 1 {
            leading_chars.insert(chars[0]);
        }
        
        for (i, &ch) in chars.iter().rev().enumerate() {
            *weights.entry(ch).or_insert(0i64) -= 10i64.pow(i as u32);
        }
    }

    let chars: Vec<char> = weights.keys().cloned().collect();
    // A puzzle cannot have more than 10 unique letters since there are only 10 digits
    if chars.len() > 10 || chars.is_empty() {
        return None;
    }

    let mut used_digits = [false; 10];
    let mut assignment = HashMap::new();

    if backtrack(0, 0, &chars, &weights, &leading_chars, &mut used_digits, &mut assignment) {
        Some(assignment)
    } else {
        None
    }
}

fn backtrack(
    idx: usize,
    current_sum: i64,
    chars: &[char],
    weights: &HashMap<char, i64>,
    leading_chars: &HashSet<char>,
    used_digits: &mut [bool; 10],
    assignment: &mut HashMap<char, u32>,
) -> bool {
    // Base case: All characters have been assigned a unique digit
    if idx == chars.len() {
        return current_sum == 0;
    }

    let ch = chars[idx];
    let w = weights[&ch];
    let can_be_zero = !leading_chars.contains(&ch);

    for digit in 0..=9 {
        if digit == 0 && !can_be_zero {
            continue;
        }
        if used_digits[digit] {
            continue;
        }

        // Tentatively assign the digit
        used_digits[digit] = true;
        assignment.insert(ch, digit as u32);

        // Recurse to the next character
        if backtrack(
            idx + 1,
            current_sum + (w * digit as i64),
            chars,
            weights,
            leading_chars,
            used_digits,
            assignment,
        ) {
            return true;
        }

        // Backtrack if the assignment didn't lead to a solution
        assignment.remove(&ch);
        used_digits[digit] = false;
    }

    false
}