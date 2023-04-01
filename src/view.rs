use crate::component::{
    input_row, input_row_sized, input_select_row, label_sized, my_button, my_picklist, my_toggler
};
use crate::gui::{Message, States};
use iced::widget::scrollable::Properties;
use iced::widget::{column, row, scrollable, Column, Row, Scrollable};

pub fn config_row<'a>(states: &'a States) -> Row<'a, Message> {
    row![
        column![
            input_row("filename", &states.filename, "text", Message::Filename),
            input_row("title", &states.title, "text", Message::Title),
            input_row("x description", &states.x_desc, "text", Message::XDesc),
            input_row("y description", &states.y_desc, "text", Message::YDesc),
            row![
                my_button("Add Line", 70, Message::AddLine),
                my_button("Generate", 70, Message::Generate),
            ]
            .spacing(10)
        ]
        .spacing(10)
        .width(200),
        column![
            input_row_sized(
                "x decimal",
                &states.x_deci,
                "integer",
                Message::XDeci,
                82,
                50
            ),
            input_row_sized(
                "x leftmost",
                &states.x_init,
                "decimal",
                Message::XInit,
                82,
                50
            ),
            input_row_sized(
                "y leftmost",
                &states.y_init,
                "decimal",
                Message::YInit,
                82,
                50
            ),
            my_toggler("subscript", states.show_sub, Message::ShowSub),
        ]
        .spacing(10)
        .width(150),
        column![
            input_row_sized(
                "y decimal",
                &states.y_deci,
                "integer",
                Message::YDeci,
                82,
                50
            ),
            input_row_sized(
                "x rightmost",
                &states.x_fin,
                "decimal",
                Message::XFin,
                82,
                50
            ),
            input_row_sized(
                "y rightmost",
                &states.y_fin,
                "decimal",
                Message::YFin,
                82,
                50
            ),
            row![
                label_sized("legend_pos", 82),
                my_picklist(
                    states.legend_pos_op.clone(),
                    states.legend_pos.clone(),
                    Message::LegendPos
                ),
            ]
            .spacing(10),
        ]
        .spacing(10)
        .width(190),
    ]
    .spacing(10)
}

pub fn line_scroll<'a>(states: &'a States) -> Scrollable<'a, Message> {
    let mut container = Row::new();
    for (i, line) in states.lines.iter().enumerate() {
        let mut line_col = Column::new();
        line_col = line_col.push(input_row_sized(
            "line name",
            &line.name,
            "text",
            move |s| Message::Linename(format!("{}|{}", i, s)),
            82,
            135,
        ));
        line_col = line_col.push(input_select_row(
            "color",
            states.colors.clone(),
            &line.color,
            move |s| Message::LineColor(format!("{}|{}", i, s)),
            82,
        ));
        line_col = line_col.push(row![
            column![input_row_sized(
                "x sub pos",
                &line.sub_pos.0,
                "integer",
                move |s| { Message::LineSubX(format!("{}|{}", i, s)) },
                82,
                50
            ),],
            column![input_row_sized(
                "y sub pos",
                &line.sub_pos.1,
                "integer",
                move |s| Message::LineSubY(format!("{}|{}", i, s)),
                82,
                50
            ),],
        ]);
        line_col = line_col.push(
            row![
                my_button("add point", 70, Message::AddPoint(i.to_string())),
                my_button("delete", 70, Message::DeleteLine(i.to_string()))
            ]
            .spacing(10)
            .width(280),
        );
        for (j, (x, y)) in line.points.iter().enumerate() {
            line_col = line_col.push(
                row![
                    column![input_row_sized(
                        "x",
                        &x,
                        "decimal",
                        move |s| Message::UpdateX(format!("{}|{}|{}", i, j, s)),
                        50,
                        50
                    )]
                    .spacing(5),
                    column![input_row_sized(
                        "y",
                        &y,
                        "decimal",
                        move |s| Message::UpdateY(format!("{}|{}|{}", i, j, s)),
                        50,
                        50
                    )]
                    .spacing(5),
                ]
                .spacing(5)
                .padding(10),
            )
        }
        container = container.push(line_col);
    }
    let container = container.spacing(12).padding(10);
    scrollable(container)
        .horizontal_scroll(Properties::new())
        .vertical_scroll(Properties::new())
}
