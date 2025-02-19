use image::Rgba;

pub fn boost_saturation(color: Rgba<u8>, boost: f32) -> Rgba<u8> {
    let (h, s, l) = rgb_to_hsl(color.0[0], color.0[1], color.0[2]);

    // Increase saturation but clamp between 0.0 and 1.0
    let new_s = (s * boost).min(1.0);

    let (r, g, b) = hsl_to_rgb(h, new_s, l);
    Rgba([r, g, b, color.0[3]])
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if max == min {
        return (0.0, 0.0, l); // Grayscale
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if max == r {
        ((g - b) / d) % 6.0
    } else if max == g {
        ((b - r) / d) + 2.0
    } else {
        ((r - g) / d) + 4.0
    } * 60.0;

    (if h < 0.0 { h + 360.0 } else { h }, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h {
        0.0..=60.0 => (c, x, 0.0),
        60.0..=120.0 => (x, c, 0.0),
        120.0..=180.0 => (0.0, c, x),
        180.0..=240.0 => (0.0, x, c),
        240.0..=300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}
