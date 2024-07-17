const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let str = String::from("HELLO THERE");
    let res = vowels(str);
    println!("{}", res);
}

fn vowels(s: String) -> u32 {
    let mut count = 0;

    for char in s.chars() {
        if VOWELS.contains(&char.to_ascii_lowercase()) {
            count += 1;
        }
    }

    count
}
