use iced::{color, Background, border, Theme};
use iced::widget::container;
use iced::widget::container::Style;

#[derive(Default)]
pub enum CustomColor {
    #[default]
    Empty,
    Power1,
    Power2,
    Power3,
    Power4,
    Power5,
    Power6,
    Power7,
    Power8,
    Power9,
    Power10,
    Power11,
    Power12,
}

pub fn tile(back_color: CustomColor) -> impl Fn(&Theme) -> container::Style + 'static {
    
    let background: Option<Background> = match back_color {
        CustomColor::Empty =>   { Some(Background::Color(color!(0xcc, 0xc0, 0xb4))) },
        CustomColor::Power1 =>  { Some(Background::Color(color!(0xee, 0xe4, 0xda))) },
        CustomColor::Power2 =>  { Some(Background::Color(color!(0xed, 0xe0, 0xc8))) },
        CustomColor::Power3 =>  { Some(Background::Color(color!(0xf2, 0xb1, 0x79))) },
        CustomColor::Power4 =>  { Some(Background::Color(color!(0xf5, 0x95, 0x63))) },
        CustomColor::Power5 =>  { Some(Background::Color(color!(0xf6, 0x7c, 0x5f))) },
        CustomColor::Power6 =>  { Some(Background::Color(color!(0xf6, 0x5e, 0x3b))) },
        CustomColor::Power7 =>  { Some(Background::Color(color!(0xed, 0xcf, 0x72))) },
        CustomColor::Power8 =>  { Some(Background::Color(color!(0xed, 0xcc, 0x61))) },
        CustomColor::Power9 =>  { Some(Background::Color(color!(0xed, 0xc8, 0x50))) },
        CustomColor::Power10 => { Some(Background::Color(color!(0xed, 0xc5, 0x3f))) },
        CustomColor::Power11 => { Some(Background::Color(color!(0xed, 0xc2, 0x2e))) },
        CustomColor::Power12 => { Some(Background::Color(color!(149, 40, 169))) },
    };
    
    move |_theme: &Theme| Style {
        background,
        border: border::rounded(3),
        ..Style::default()
    }
}

