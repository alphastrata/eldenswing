use std::io::Cursor;
use image::io::Reader;
use std::fs;
use std::path::Path;
use chrono::prelude::Utc;
use std::fs::File;
use std::io::Read;

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
        // cropped.save("souls_counter.png").unwrap(); //NOTE: This is going to continually replace the only file...
        cropped.save("souls_counter.jpg").unwrap(); //NOTE: This is going to continually replace the only file...
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


fn get_file_as_byte_vec(filename: String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
fn main() {
    println!("Hello, world!");
    let start = Utc::now();

    let paths = run("assets".into());
    let souls_box = make_box(1920, 1080);

    for p in paths.iter() {
        let current = Utc::now();
        let p = format!("{}{}", "assets/", p.clone());
        // crop_from_screengrab(p, souls_box);
        // read_text("souls_counter.png".into(), souls_box);
        read_text(p.into(), souls_box);
        let end = Utc::now() - current;
        println!("\tTime Taken:{}ms\n", end.num_milliseconds());
    }
    // crop_from_screengrab("assets/t4.png".into(),souls_box); //NOTE: something funny about the way imgs come outta the image crate -- leptess doesn't see them as valid images.
    // read_text("souls_counter.jpg".into(),souls_box);

    println!("Total Imgs {}", paths.len());
    println!("Total time {}ms", (Utc::now() - start).num_milliseconds());
    
    //NOTE: You left off here, trying to get the in-memory working.
    let img_in_memory = get_file_as_byte_vec("assets/1336.png".to_string());
    read_from_memory(&img_in_memory);  

}

fn strip_non_digits(s: &str) -> String {
    let t = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    t
}

// Make a box to cover the souls counter as a % of screen resolution (x and y)
fn make_box(x: u32, y: u32)-> (i32, i32, i32, i32){
    // let x = (x as f64 * 0.89).round() as i32;
    let x = (x as f64 * 0.98).round() as i32;
    let y = (y as f64 * 0.94).round() as i32;
    let w = (x as f64  * 0.0789).round() as i32;
    let h = (y as f64  * 0.0264).round() as i32;

    println!("BOX:\n{:#?}", (x, y, w, h));

    (x,y,w,h)

}
// // NOTE: Kinda works, but it returns memory addresses for every component that the matcher matches on...
// fn get_bounding_boxes(p: String) {
//     let mut lt = leptess::LepTess::new(None, "eng").unwrap();
//     lt.set_image(p);
//     let boxes = lt.get_component_boxes(
//         leptess::capi::TessPageIteratorLevel_RIL_WORD,
//         true,
//     ).unwrap();

//     for b in boxes.into_iter() {
//        println!("{:?}", b);
// }
// }

// Read text using leptess/tesseract, souls_box should be the output of make_box.
fn read_text(p: String, _souls_box: (i32, i32, i32, i32)) {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image(&p[..]);

    // lt.set_rectangle(
    //     &leptess::leptonica::Box::new(souls_box.0, souls_box.2, souls_box.2, souls_box.3).unwrap(),
    // );

    let text = lt.get_utf8_text().unwrap();
    println!("{} = {}", p, strip_non_digits(&text));
}


fn read_from_memory(img: &[u8]){
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image_from_mem(img);
    println!("{}", lt.get_utf8_text().unwrap());
}













