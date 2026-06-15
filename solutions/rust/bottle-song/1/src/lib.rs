pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut lyric = String::new();
    for i in 0..take_down {
        lyric += &format!("{} hanging on the wall,\n", get_text_number(start_bottles-i));
        lyric += &format!("{} hanging on the wall,\n", get_text_number(start_bottles-i));
        lyric += &format!("And if {} should accidentally fall,\n", get_text_number(1).to_lowercase());
        lyric += &format!("There'll be {} hanging on the wall.\n", get_text_number(start_bottles-i-1).to_lowercase());
        lyric += "\n";
    }

    lyric
}

fn get_text_number(num: u32) -> String {
    let mut string = match num {
        0 => "No",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => todo!("Need implement for {num}")
    }.to_string() + " green bottles";

    if num == 1 {
        string = string.replace("bottles", "bottle");
    }
    string
}
