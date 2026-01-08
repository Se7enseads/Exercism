pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();

   for n in (start_bottles + 1 - take_down)..=start_bottles {
        verses.push(verse(n));
    }

    verses.reverse();
    dbg!(&verses);
    verses.join("\n\n")
}

fn verse(n: u32) -> String {
    let current = format!(
        "{} green {} hanging on the wall,\n\
         {} green {} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {} green {} hanging on the wall.",
        number_word(n),
        bottle(n),
        number_word(n),
        bottle(n),
        number_word(n - 1).to_lowercase(),
        bottle(n - 1)
    );

    current
}

fn bottle(n: u32) -> &'static str {
    if n == 1 {
        "bottle"
    } else {
        "bottles"
    }
}

fn number_word(n: u32) -> &'static str {
    match n {
        0 => "no",
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
        _ => unreachable!(),
    }
}
