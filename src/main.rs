use clap::Parser;
use image::{ImageBuffer, Rgba};
use rusttype::{point, Font, Scale};
use std::io::{self};
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
    let input = "use reqwest::blocking::get;
use serde_json::Value;
use std::fs;

fn example_function() -> anyhow::Result<()> {
    let url = \"https://api.example.com/data\";
    let response = get(url)?;

    let json: Value = response.json()?;
    
    let data = json[\"data\"]
        .as_str()
        .ok_or(anyhow::anyhow!(
	        \"Failed to extract 'data' field from JSON\"
	    ))?;

    let file_path = \"output.txt\";
    fs::write(file_path, data)?;

    println!(\"Data successfully written to {}\", file_path);
    Ok(())
}";

    let output = "output.png".to_string();

    // - `base16-ocean.dark`,`base16-eighties.dark`,`base16-mocha.dark`,`base16-ocean.light`
    // Load syntax definitions and theme
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);

    // Load a font
    let font_data = include_bytes!("/System/Library/Fonts/SFNSMono.ttf"); // Replace with your font path
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // Create an image buffer with a transparent background
    let font_size = 35.0;
    let scale = Scale::uniform(font_size);
    let line_height = (font_size * 1.2) as u32; // Adjust line height
    let line_count = input.lines().count();
    let image_height = line_count as u32 * line_height;
    let max_line_width = input
        .lines()
        .map(|line| line.chars().count() as f32 * (font_size * 0.6)) // Estimate character width
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0) as u32;

    let mut image = ImageBuffer::new(max_line_width, image_height); // Adjust dimensions as needed

    // Render the code into the image
    let mut y = font_size as u32;
    for line in LinesWithEndings::from(&input) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let mut x = 0;
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
