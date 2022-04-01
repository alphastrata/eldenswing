use leptess::*;

fn main() {
    println!("Hello, world!");

    read_text("19292901.png".to_string());
    read_text("19324292.png".to_string());
    read_text("19352740.png".to_string());
}
fn strip_non_digits(s: &str) -> String {
    let t = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    t
}

fn read_text(p: String) {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image(&p[..]);
    let mut text = lt.get_utf8_text().unwrap();
    println!("{}", strip_non_digits(&text));
}
