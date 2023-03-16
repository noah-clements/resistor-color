// Refactored to NOT use the external crates, even though that was the point of the exercise.

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ResistorColor {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    colors().iter().position(|&c| c == color).unwrap() as u32
}

pub fn value_to_color_string(value: u32) -> String {
    if value > 9 {
        return String::from("value out of range");
    }
    format!("{:?}", &colors()[value as usize])
}

pub fn colors() -> Vec<ResistorColor> {
    use ResistorColor::*;
    vec![Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White]
}
