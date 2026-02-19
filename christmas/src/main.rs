fn main() {
    let presents = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];
    let numerals = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelveth"
    ];
    for day in 0..=11 {
        let numeral = numerals[day];
        println!("On the {numeral} day of Christmas my true love sent to me");
        for present_idx in (0..=day).rev() {
            let present = presents[present_idx];
            if present_idx == 0 && day > 0 {
                print!("And {present}.\n");
            } else {
                let present = capitalize(present);
                print!("{present}");
                if day == 0 {
                    println!(".");
                } else {
                    println!(",");
                }
            }
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
