use std::fs;
use std::path::Path;
// use image::*;

#[allow(dead_code)]
fn crop_from_screengrab(
        p: String,
        souls_box: (i32,i32,i32,i32),

    )  {
        let mut img = image::open(p).unwrap();
        let cropped = img.crop(
            souls_box.0.try_into().unwrap(),
            souls_box.1.try_into().unwrap(),
            souls_box.2.try_into().unwrap(),
            souls_box.3.try_into().unwrap(),
            ); 
        // let filename = stringify!("souls_counter.png");
        cropped.save("souls_counter.png").unwrap();
    }

fn run(p: String) -> Vec<String> {
    let paths = fs::read_dir(&Path::new(&p)).unwrap();

    let names = paths
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
            .filter(|s| s.contains(".png"))
        })
        .collect::<Vec<String>>();
    names
}
fn main() {
    println!("Hello, world!");

    let paths = run("assets".into());
    let souls_box = make_box(1280, 720);

    for p in paths.iter() {
        println!("{}", p);
        let p = format!("{}{}", "assets/", p);
        // crop_from_screengrab(p, souls_box);
        // read_text("souls_counter.png".into(), souls_box);
        read_text(p.into(), souls_box);
    }

    // read_text("frames/f0355.jpg".into(), souls_box); // should be 19315185
    // read_text("frames/f0860.jpg".into(), souls_box); // should be 19324292




    // crop_from_screengrab("frames/f0355.jpg".into(), souls_box); 
    // read_text("souls_counter.png".into(), souls_box); 

    // crop_from_screengrab("frames/f0355.jpg".into(), souls_box); 
    // read_text("souls_counter.png".into(), souls_box); 


    println!("RUN_LEN {}", paths.len());

}

fn strip_non_digits(s: &str) -> String {
    let t = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    t
}

fn make_box(x: u32, y: u32)-> (i32, i32, i32, i32){
    let x = (x as f64 * 0.89) as i32;
    let y = (y as f64 * 0.94) as i32;
    let w = (x as f64  * 0.0789) as i32;
    let h = (y as f64  * 0.0264) as i32;

    (x,y,w,h)

}

fn read_text(p: String, souls_box: (i32, i32, i32, i32)) {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image(&p[..]);

    // lt.set_rectangle(
    //     &leptess::leptonica::Box::new(souls_box.0, souls_box.2, souls_box.2, souls_box.3).unwrap(),
    // );

    #[allow(unused_must_use)]
    // lt.set_image(&p[..]);
    let text = lt.get_utf8_text().unwrap();
    println!("{}", strip_non_digits(&text));
}





















// let x = 2280; 89.06% of 2560
// let y = 1362; 0.94% of 1440
// let width = 202; 0.0789% of 2560
// let height = 38; 0.0264% of 1440

// read_text("assets/19292901.png".to_string()); // 19292901
// read_text("assets/19324292.png".to_string()); // 19324292
// read_text("assets/19352740.png".to_string()); // 19352740

// //WRONG!!!
// read_text("assets/112072.png".to_string()); // 1120727 << This one is wrong for some reason...
//                                             //

// read_text("assets/112669.png".to_string()); // 112669
// read_text("assets/62896.png".to_string()); // 62896
