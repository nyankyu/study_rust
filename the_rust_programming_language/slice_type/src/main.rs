fn main() {
    let s = String::from("Hello, world!");
    let word = first_word(&s[..]);
    //s.clear();
    println!("first word is [{}]", word);

    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let a_slice = slice_2_5(&a[..]);
    for i in a_slice {
        println!("{}", i);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn slice_2_5(a: &[i32]) -> &[i32] {
    &a[2..6]
}
