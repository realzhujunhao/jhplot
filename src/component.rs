use crate::gui::Message;
use iced::widget::{
    button, pick_list, row, text, text_input, toggler, Button, PickList, Row, Text, TextInput,
    Toggler,
};
use iced::{alignment, Font};
use lazy_static::lazy_static;

const TEXT_FONT_BYTES: &[u8] = include_bytes!("font/DeliciousHandrawn-Regular.ttf");
const DEFAULT_T_WIDTH: u16 = 90;
const DEFAULT_I_WIDTH: u16 = 130;

lazy_static! {
    static ref TEXT_FONT: Font = {
        Font::External {
            name: "Lobster-Regular",
            bytes: TEXT_FONT_BYTES,
        }
    };
}

pub fn input_row<'a, M>(
    label_str: &'a str,
    value: &'a str,
    hint: &'a str,
    msg: M,
) -> Row<'a, Message>
where
    M: Fn(String) -> Message + 'a,
{
    input_row_sized(
        label_str,
        value,
        hint,
        msg,
        DEFAULT_T_WIDTH,
        DEFAULT_I_WIDTH,
    )
}

pub fn input_row_sized<'a, M>(
    label_str: &'a str,
    value: &'a str,
    hint: &'a str,
    msg: M,
    t_size: u16,
    i_size: u16,
) -> Row<'a, Message>
where
    M: Fn(String) -> Message + 'a,
{
    row![
        label_sized(label_str, t_size),
        input_sized(hint, value, msg, i_size),
    ]
    .spacing(10)
}

pub fn input_select_row<'a, M>(
    label_str: &'a str,
    options: Vec<String>,
    selected: &str,
    msg: M,
    t_width: u16,
) -> Row<'a, Message>
where
    M: Fn(String) -> Message + 'a,
{
    row![
        label_sized(label_str, t_width),
        my_picklist(options, selected.into(), msg),
    ].spacing(10)
}

pub fn label_sized<'a>(label_str: &'a str, t_width: u16) -> Text<'a> {
    text(label_str)
        .font(*TEXT_FONT)
        .width(t_width)
        .size(25)
        .height(35)
        .vertical_alignment(alignment::Vertical::Center)
        .horizontal_alignment(alignment::Horizontal::Right)
}

pub fn input_sized<'a, M>(
    hint: &'a str,
    value: &'a str,
    msg: M,
    i_width: u16,
) -> TextInput<'a, Message>
where
    M: Fn(String) -> Message + 'a,
{
    text_input(hint, value, msg)
        .font(*TEXT_FONT)
        .width(i_width)
        .padding(5)
}

pub fn my_toggler<'a, M>(label: &str, value: bool, msg: M) -> Toggler<'a, Message>
where
    M: Fn(bool) -> Message + 'a,
{
    toggler(String::from(label), value, msg)
        .font(*TEXT_FONT)
        .size(30)
        .spacing(0)
        .text_size(25)
        .text_alignment(alignment::Horizontal::Center)
}

pub fn my_picklist<'a, M>(
    options: Vec<String>,
    value: String,
    msg: M,
) -> PickList<'a, String, Message>
where
    M: Fn(String) -> Message + 'a,
{
    pick_list(options, Some(value), msg).font(*TEXT_FONT)
}

pub fn my_button<'a>(content: &'a str, width: u16, msg: Message) -> Button<'a, Message> {
    let content = text(content)
        .font(*TEXT_FONT)
        .width(width)
        .size(25)
        .height(35)
        .vertical_alignment(alignment::Vertical::Center)
        .horizontal_alignment(alignment::Horizontal::Center);
    button(content).on_press(msg).height(30).width(90)
}
