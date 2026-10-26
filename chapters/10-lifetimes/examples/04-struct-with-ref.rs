//! A struct with a reference needs a lifetime and can't outlive its source

struct Highlight<'a> {
    text: &'a str,
}

fn main() {
    let article = String::from("a long article text");
    // &article[0..6]: slice bounds are in BYTES. Here every character is one byte;
    // with Cyrillic text (2 bytes per letter) the same range would be 3 letters.
    let h = Highlight {
        text: &article[0..12],
    };
    // h can't outlive article — the compiler checks it
    println!("{}", h.text); // a long
}
