use colors_transform::Hsl;
use colors_transform::Color;

pub fn get_colors(n_colors: usize) -> Vec<String> {
    let mut v = vec![];
    for i in 0..n_colors {
        let hue = (360.0 / n_colors as f32) * i as f32;
        let hex_color = Hsl::from(hue, 100.0, 50.0).to_rgb().to_css_hex_string();
        v.push(hex_color);
    }
    return v;
}
