use iced::widget::{column, row, svg};
use iced::{executor, Application, Command, Element, Theme};
use crate::view::{line_scroll, config_row};

use crate::plot::generate_chart;

#[derive(Debug, Clone)]
pub struct States {
    pub svg_bytes: String,
    pub filename: String,
    pub title: String,
    pub x_desc: String,
    pub y_desc: String,
    pub x_deci: String,
    pub y_deci: String,
    pub x_init: String,
    pub x_fin: String,
    pub y_init: String,
    pub y_fin: String,
    pub show_sub: bool,
    pub legend_pos: String,
    pub legend_pos_op: Vec<String>,
    pub lines: Vec<Line>,
    pub colors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub name: String,
    pub color: String,
    pub points: Vec<(String, String)>,
    pub sub_pos: (String, String),
}

impl Application for States {
    type Theme = Theme;
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn title(&self) -> String {
        "jhplot".into()
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                filename: "0".into(),
                title: "title".into(),
                x_desc: "x_desc".into(),
                y_desc: "y_desc".into(),
                x_deci: "2".into(),
                y_deci: "2".into(),
                x_init: "-1.0".into(),
                x_fin: "1.0".into(),
                y_init: "-1.0".into(),
                y_fin: "1.0".into(),
                show_sub: true,
                legend_pos: "UpperLeft".into(),
                legend_pos_op: vec![
                    "UpperLeft",
                    "MiddleLeft",
                    "LowerLeft",
                    "UpperMiddle",
                    "MiddleMiddle",
                    "LowerMiddle",
                    "UpperRight",
                    "MiddleRight",
                    "LowerRight",
                ]
                .into_iter()
                .map(|s| s.into())
                .collect(),
                lines: vec![],
                colors: vec!["Black", "Blue", "Cyan", "Green", "Magenta", "Red", "Yellow"]
                    .into_iter()
                    .map(|s| s.into())
                    .collect(),
                svg_bytes: String::new(),
            },
            Command::none(),
        )
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let bytes: &'static [u8] =
            Box::leak(self.svg_bytes.clone().into_boxed_str().into_boxed_bytes());
        let handle = svg::Handle::from_memory(bytes);
        row![
            column![config_row(self), line_scroll(self)]
                .spacing(20)
                .width(600),
            column![svg(handle)]
        ]
        .spacing(10)
        .padding(20)
        .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Filename(s) => {
                self.filename = s;
            }
            Message::Title(s) => {
                self.title = s;
            }
            Message::XDesc(s) => {
                self.x_desc = s;
            }
            Message::YDesc(s) => {
                self.y_desc = s;
            }
            Message::XDeci(s) => {
                change_if_type::<usize>(&mut self.x_deci, &s);
            }
            Message::YDeci(s) => {
                change_if_type::<usize>(&mut self.y_deci, &s);
            }
            Message::XInit(s) => {
                change_if_type::<f64>(&mut self.x_init, &s);
            }
            Message::XFin(s) => {
                change_if_type::<f64>(&mut self.x_fin, &s);
            }
            Message::YInit(s) => {
                change_if_type::<f64>(&mut self.y_init, &s);
            }
            Message::YFin(s) => {
                change_if_type::<f64>(&mut self.y_fin, &s);
            }
            Message::ShowSub(b) => {
                self.show_sub = b;
            }
            Message::LegendPos(s) => {
                self.legend_pos.replace_range(.., &s);
            }
            Message::AddLine => {
                let line_num = self.lines.len() + 1;
                let line = Line {
                    name: format!("line {}", line_num),
                    color: self
                        .colors
                        .get((line_num - 1) % self.colors.len())
                        .unwrap()
                        .clone(),
                    sub_pos: ("12".into(), "-12".into()),
                    points: vec![],
                };
                self.lines.push(line);
            }
            Message::Linename(s) => {
                let (idx, content) = decode(s);
                self.lines
                    .get_mut(idx)
                    .unwrap()
                    .name
                    .replace_range(.., &content);
            }
            Message::LineColor(s) => {
                let (idx, content) = decode(s);
                self.lines
                    .get_mut(idx)
                    .unwrap()
                    .color
                    .replace_range(.., &content);
            }
            Message::LineSubX(s) => {
                let (idx, content) = decode(s);
                change_if_type::<i32>(&mut self.lines.get_mut(idx).unwrap().sub_pos.0, &content)
            }
            Message::LineSubY(s) => {
                let (idx, content) = decode(s);
                change_if_type::<i32>(&mut self.lines.get_mut(idx).unwrap().sub_pos.1, &content)
            }
            Message::DeleteLine(s) => {
                let idx = s.parse().unwrap();
                self.lines.remove(idx);
            }
            Message::AddPoint(s) => {
                let idx: usize = s.parse().unwrap();
                self.lines
                    .get_mut(idx)
                    .unwrap()
                    .points
                    .push(("0.0".into(), "0.0".into()));
            }
            Message::UpdateX(s) => {
                let (line_idx, point_idx, content) = decode_3(s);
                change_if_type::<f64>(
                    &mut self
                        .lines
                        .get_mut(line_idx)
                        .unwrap()
                        .points
                        .get_mut(point_idx)
                        .unwrap()
                        .0,
                    &content,
                );
            }
            Message::UpdateY(s) => {
                let (line_idx, point_idx, content) = decode_3(s);
                change_if_type::<f64>(
                    &mut self
                        .lines
                        .get_mut(line_idx)
                        .unwrap()
                        .points
                        .get_mut(point_idx)
                        .unwrap()
                        .1,
                    &content,
                );
            }
            Message::Generate => {
                generate_chart(self).unwrap();
                std::fs::write(format!("{}.svg", &self.filename), &self.svg_bytes).unwrap();
            }
        }
        Command::none()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Filename(String),
    Title(String),
    XDesc(String),
    YDesc(String),
    XInit(String),
    XFin(String),
    YInit(String),
    YFin(String),
    XDeci(String),
    YDeci(String),
    ShowSub(bool),
    LegendPos(String),
    AddLine,
    DeleteLine(String),
    AddPoint(String),
    UpdateX(String),
    UpdateY(String),
    Linename(String),
    LineColor(String),
    LineSubX(String),
    LineSubY(String),
    Generate,
}

fn change_if_type<T>(field: &mut String, input: &str)
where
    T: std::str::FromStr,
{
    if let Ok(_) = input.parse::<T>() {
        field.replace_range(.., input);
        return;
    }
}

fn decode(input: String) -> (usize, String) {
    let k_v: Vec<String> = input.split("|").map(|s| s.into()).collect();
    let line_num: usize = k_v.get(0).unwrap().parse().unwrap();
    let content = k_v.get(1).unwrap();
    (line_num, content.clone())
}

fn decode_3(input: String) -> (usize, usize, String) {
    let k_v: Vec<String> = input.split("|").map(|s| s.into()).collect();
    let line_num: usize = k_v.get(0).unwrap().parse().unwrap();
    let point_idx = k_v.get(1).unwrap().parse().unwrap();
    let content = k_v.get(2).unwrap();
    (line_num, point_idx, content.clone())
}
