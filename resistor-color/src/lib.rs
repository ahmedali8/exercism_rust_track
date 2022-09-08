use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[repr(usize)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => format!("{:?}", color),
        _ => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors: Vec<ResistorColor> = all::<ResistorColor>().collect::<Vec<ResistorColor>>();
    colors.sort();
    colors
}
