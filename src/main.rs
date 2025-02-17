use clap::{Arg, Command, Parser};
use image::{ImageBuffer, Rgba};
use rusttype::{point, Font, Scale};
use std::io::{self, Write};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The code input
    #[arg(short, long)]
    input: String,

    /// The output image file
    #[arg(short, long, default_value = "output.png")]
    output: String,
}

fn main() -> io::Result<()> {
    // Parse command-line arguments
    // let args = Args::parse();
    let input = "fn main() -> io::Result<()> {
    // Parse command-line arguments
    // let args = Args::parse();
    let input = \"pub enum Result<T, E> {
    Ok(T),
    Err(E),
    }\"
}";

    let output = "output.png".to_string();

    // - `base16-ocean.dark`,`base16-eighties.dark`,`base16-mocha.dark`,`base16-ocean.light`
    // Load syntax definitions and theme
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-mocha.dark"]);

    // Load a font
    let font_data = include_bytes!("/System/Library/Fonts/SFNSMono.ttf"); // Replace with your font path
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // Create an image buffer with a transparent background
    let font_size = 20.0;
    let scale = Scale::uniform(font_size);
    let line_height = (font_size * 1.2) as u32; // Adjust line height
    let padding = 20;
    let mut image = ImageBuffer::new(800, 600); // Adjust dimensions as needed

    // Render the code into the image
    let mut y = padding;
    for line in LinesWithEndings::from(&input) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let mut x = padding;
        for (style, text) in ranges {
            let color = style.foreground;
            let rgba = Rgba([color.r, color.g, color.b, 255]);
            for c in text.chars() {
                // Render the character using rusttype
                let glyph = font
                    .glyph(c)
                    .scaled(scale)
                    .positioned(point(x as f32, y as f32));
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|gx, gy, gv| {
                        let px = (gx as i32 + bounding_box.min.x) as u32;
                        let py = (gy as i32 + bounding_box.min.y) as u32;
                        if px < image.width() && py < image.height() {
                            let pixel = image.get_pixel_mut(px, py);
                            *pixel = Rgba([rgba[0], rgba[1], rgba[2], (gv * 255.0) as u8]);
                        }
                    });
                }
                x += (font_size * 0.6) as u32; // Adjust character spacing
            }
        }
        y += line_height;
    }

    // Save the image
    image.save(&output).unwrap();
    println!("Image saved to {}", &output);

    Ok(())
}
