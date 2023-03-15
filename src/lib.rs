extern crate int_enum;
extern crate enum_iterator;

use int_enum::IntEnum;
// use int_enum::IntEnumError;
use enum_iterator::{all, Sequence};

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, IntEnum, Copy, Clone, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

// Really should have more explanation for the IntEnum trait.
// I thought we would have to implement it ourselves. And the docs were very unclear.
// Instead we needed to put the values into the enum and the trait was implemented for us.
// I had to look at other people's solutions to figure this out.
/* impl IntEnum for ResistorColor {
    type Int = u32;
    fn int_value(self) -> u32 {
        match self {
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

    fn from_int(n: Self::Int) -> Result<Self, int_enum::IntEnumError<Self>>
        where
            Self: Sized {
        match n {
            0 => Ok(ResistorColor::Black),
            1 => Ok(ResistorColor::Brown),
            2 => Ok(ResistorColor::Red),
            3 => Ok(ResistorColor::Orange),
            4 => Ok(ResistorColor::Yellow),
            5 => Ok(ResistorColor::Green),
            6 => Ok(ResistorColor::Blue),
            7 => Ok(ResistorColor::Violet),
            8 => Ok(ResistorColor::Grey),
            9 => Ok(ResistorColor::White),
            _ => Err(int_enum::IntEnumError<_>),
        }
    } 
}
 */
pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
   ResistorColor::from_int(value)
        .map(|c| format!("{:?}", c))
        .unwrap_or_else(|_| "value out of range".to_owned())
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}
