const VERSES: [&str; 3] = [
    "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n", "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"
];

pub fn verse(n: u32) -> String {
    match n {
        0 => VERSES[0].to_string(),
        1 => VERSES[1].to_string(),
        2 => VERSES[2].to_string(),
        _ => format!(
            "{0} bottles of beer on the wall, {1} bottles of beer.\nTake one down and pass it around, {2} bottles of beer on the wall.\n",
            n,
            n,
            n - 1
        ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    assert!(start > end, "start must be greater than end");

    let verses: String = (end..=start)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<String>>()
        .join("\n");

    // println!("verses: {:?}", verses);

    verses
}
