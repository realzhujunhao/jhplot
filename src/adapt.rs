use plotters::{prelude::SeriesLabelPosition, style::*};

pub fn parse_color(color: &str) -> RGBColor {
    match color {
        "Black" => BLACK,
        "Blue" => BLUE,
        "Cyan" => CYAN,
        "Green" => GREEN,
        "Magenta" => MAGENTA,
        "Red" => RED,
        "Yellow" => YELLOW,
        _ => BLACK,
    }
}

pub fn parse_points(point: &Vec<(String, String)>) -> Vec<(f64, f64)> {
    point
        .into_iter()
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}

pub fn parse_legend_pos(pos: &str) -> SeriesLabelPosition {
    use SeriesLabelPosition::*;
    match pos {
        "UpperLeft" => UpperLeft,
        "UpperMiddle" => UpperMiddle,
        "UpperRight" => UpperRight,
        "MiddleLeft" => MiddleLeft,
        "MiddleMiddle" => MiddleMiddle,
        "MiddleRight" => MiddleRight,
        "LowerLeft" => LowerLeft,
        "LowerMiddle" => LowerMiddle,
        "LowerRight" => LowerRight,
        _ => UpperLeft,
    }
}

pub fn parse_sub_pos(pos: &(String, String)) -> (i32, i32) {
    (pos.0.parse().unwrap(), pos.1.parse().unwrap()) 
}
