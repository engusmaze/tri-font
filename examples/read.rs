use std::time::Instant;

use tri_font::Font;

fn main() {
    let start = Instant::now();
    let font = Font::read(include_bytes!("Roboto-Regular.ttf")).unwrap();
    let end = Instant::now();
    println!("Characters: {}", font.map.len());
    println!("Read in: {:?}", end - start);
}
