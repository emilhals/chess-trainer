use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::Style,
    widgets::Widget,
};
pub const DIGITS: [[&str; 3]; 10] = [
    ["█▀█", "█ █", "▀▀▀"], // 0
    ["▀█ ", " █ ", "▀▀▀"], // 1
    ["▀▀█", "█▀▀", "▀▀▀"], // 2
    ["▀▀█", " ▀█", "▀▀▀"], // 3
    ["█ █", "▀▀█", "  ▀"], // 4
    ["█▀▀", "▀▀█", "▀▀▀"], // 5
    ["█▀▀", "█▀█", "▀▀▀"], // 6
    ["▀▀█", "  █", "  ▀"], // 7
    ["█▀█", "█▀█", "▀▀▀"], // 8
    ["█▀█", "▀▀█", "▀▀▀"], // 9
];

#[derive(Default, Setters)]
pub struct BigNumber {
    value: u32,
    style: Style,
    alignment: Alignment,
}

impl BigNumber {
    const DIGIT_WIDTH: u16 = 3;
    const GAP: u16 = 1;
    pub fn desired_width(&self) -> u16 {
        let digits = self.value.to_string().len() as u16;

        digits * Self::DIGIT_WIDTH + (digits - 1) * Self::GAP
    }
}

impl Widget for BigNumber {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let value_string = self.value.to_string();

        let area_width: u16 = self.desired_width();
        let area: Rect = match self.alignment {
            Alignment::Left => area,
            Alignment::Center => area.centered_horizontally(Constraint::Length(area_width)),

            Alignment::Right => Rect {
                x: area.right() - area_width,
                y: area.y,
                width: area_width,
                height: area.height,
            },
        };

        for (col, char) in value_string.chars().enumerate() {
            let num = char.to_digit(10).expect("value is not a number");

            for (row, line) in DIGITS[num as usize].iter().enumerate() {
                buf.set_string(
                    area.x + col as u16 * (Self::DIGIT_WIDTH + Self::GAP),
                    area.y + row as u16,
                    line,
                    self.style,
                );
            }
        }
    }
}
