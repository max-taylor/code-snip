use clap::{Arg, Command, Parser};
use image::{ImageBuffer, Rgba};
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
    let args = Args::parse();

    // Read the code from the input file

    // let output_file = matches.value_source("output").unwrap();

    // Load syntax definitions and theme
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    // Create an image buffer with a transparent background
    let font_size = 20;
    let line_height = font_size + 4;
    let padding = 10;
    let mut image = ImageBuffer::new(800, 600); // Adjust dimensions as needed

    // Render the code into the image
    let mut y = padding;
    for line in LinesWithEndings::from(&args.input) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let mut x = padding;
        for (style, text) in ranges {
            let color = style.foreground;
            let rgba = Rgba([color.r, color.g, color.b, 255]);
            for c in text.chars() {
                // Draw each character (this is a simplified example)
                // You can use a font rendering library like `rusttype` for better text rendering
                image.put_pixel(x, y, rgba);
                x += font_size / 2; // Adjust spacing
            }
        }
        y += line_height;
    }

    // Save the image
    image.save(&args.output).unwrap();
    println!("Image saved to {}", &args.output);

    Ok(())
}
