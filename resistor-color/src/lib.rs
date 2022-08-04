#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

fn value_to_color(value: u32) -> ResistorColor {
    match value {
        0 => ResistorColor::Black,
        1 => ResistorColor::Brown,
        2 => ResistorColor::Red,
        3 => ResistorColor::Orange,
        4 => ResistorColor::Yellow,
        5 => ResistorColor::Green,
        6 => ResistorColor::Blue,
        7 => ResistorColor::Violet,
        8 => ResistorColor::Grey,
        9 => ResistorColor::White,
        _ => ResistorColor::Black
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    match _color {
        ResistorColor::Black => 0,
        ResistorColor::Brown => 1,
        ResistorColor::Red => 2,
        ResistorColor::Orange => 3,
        ResistorColor::Yellow => 4,
        ResistorColor::Green => 5,
        ResistorColor::Blue => 6,
        ResistorColor::Violet => 7,
        ResistorColor::Grey => 8,
        ResistorColor::White => 9,
    }
}

pub fn value_to_color_string(value: u32) -> String {
    match value {
        0 => String::from("Black"),
        1 => String::from("Brown"),
        2 => String::from("Red"),
        3 => String::from("Orange"),
        4 => String::from("Yellow"),
        5 => String::from("Green"),
        6 => String::from("Blue"),
        7 => String::from("Violet"),
        8 => String::from("Grey"),
        9 => String::from("White"),
        _ => String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors = vec![];
    for i in 0..=9 {
        colors.push(value_to_color(i));
    }
    colors
}
