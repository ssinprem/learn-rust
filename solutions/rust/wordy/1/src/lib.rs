use regex::Regex;

#[derive(PartialEq)]
pub enum States {
    First,
    Plus,
    Minus,
    Multiple,
    Divide,
    Number
}

pub fn answer(command: &str) -> Option<i32> {
    let mut command = command.to_string();
    if let Some(check) = command.strip_prefix("What is ") {
        command = check.to_string();
    } else {
        return None;
    }
    if let Some(check) = command.strip_suffix("?") {
        command = check.to_string();
    } else {
        return None;
    }

    let mut result = None;

    let regex = Regex::new(
        r"([-0-9]+)|((?:plus)|(?:minus)|(?:multiplied by)|(?:divided by))|(?:raised to the (\d+).. power)|(.*)"
    ).unwrap();

    let mut n = 0;
    let mut state = States::First;
    while let Some(cap) = regex.captures_at(command.as_str(), n) {
        // group 1 is number
        if let Some(matched) = cap.get(1)
            && let Ok(num) = matched.as_str().parse::<i32>()
            && state != States::Number
        {
            match state {
                States::First => result = Some(num),
                States::Plus => result = Some(result.unwrap() + num),
                States::Minus => result = Some(result.unwrap() - num),
                States::Multiple  => result = Some(result.unwrap() * num),
                States::Divide  => result = Some(result.unwrap() / num),
                _ => unreachable!(),
            }
            n += matched.as_str().len() + 1;
            state = States::Number;
        }
        // group 2 is oparand
        else if let Some(matched) = cap.get(2)
             && state == States::Number
        {
            match matched.as_str() {
                "plus" => state = States::Plus,
                "minus" => state = States::Minus,
                "multiplied by" => state = States::Multiple,
                "divided by" => state = States::Divide,
                _ => unreachable!()
            }

            n += matched.as_str().len() + 1;
        }
        // group 3 is power with nth
        else if let Some(matched) = cap.get(3)
            && let Ok(num) = matched.as_str().parse::<i32>()
            && state == States::Number
        {
            result = Some(result.unwrap().pow(num as u32));
            n += matched.as_str().len() + 1 + 22;
        }
        else
        {
            return None;
        }
    }

    if state == States::Number {
        result
    } else {
        None
    }
}
