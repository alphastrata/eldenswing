use leptess::*;

fn main() {
    println!("Hello, world!");

    read_text("assets/19292901.png".to_string());// 19292901
    read_text("assets/19324292.png".to_string());// 19324292    
    read_text("assets/19352740.png".to_string());// 19352740
    
    //WRONG!!!
    read_text("assets/112072.png".to_string());// 1120727 << This one is wrong for some reason...
    //
    
    read_text("assets/112669.png".to_string());// 112669
    read_text("assets/62896.png".to_string());// 62896
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






