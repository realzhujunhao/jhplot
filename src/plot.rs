use crate::{
    adapt::{parse_color, parse_points, parse_legend_pos, parse_sub_pos},
    gui::States,
};
use plotters::prelude::*;

pub fn generate_chart<'a>(states: &mut States) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::with_string(&mut states.svg_bytes, (640, 480))
        .into_drawing_area()
        .margin(1, 1, 1, 1);
    root.fill(&WHITE)?;
    let x_range: std::ops::Range<f64> =
        states.x_init.parse().unwrap()..states.x_fin.parse().unwrap();
    let y_range: std::ops::Range<f64> =
        states.y_init.parse().unwrap()..states.y_fin.parse().unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(&states.title, ("sans-serif", 30).into_font())
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(x_range, y_range)?;

    chart
        .configure_mesh()
        .x_desc(&states.x_desc)
        .y_desc(&states.y_desc)
        .x_label_formatter(&|x| format!("{:.1$}", x, &states.x_deci.parse().unwrap()))
        .y_label_formatter(&|y| format!("{:.1$}", y, &states.y_deci.parse().unwrap()))
        .draw()?;
    for line in &states.lines {
        let color = parse_color(&line.color);
        let points = parse_points(&line.points);
        let sub_pos = parse_sub_pos(&line.sub_pos);
        chart
            .draw_series(
                LineSeries::new(points.clone(), color))?
            .label(&line.name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        chart.draw_series(PointSeries::of_element(
            points.clone(),
            3.0,
            color,
            &|coord, size, style| {
                let subscript = if states.show_sub {
                    format!(
                        "({:.2$}, {:.3$})",
                        coord.0, coord.1, states.x_deci.parse().unwrap(), states.y_deci.parse().unwrap()
                    )
                } else {
                    String::new()
                };
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style.filled())
                    + Text::new(
                        subscript,
                        sub_pos,
                        ("sans-serif", 15).into_font(),
                    )
            },
        ))?;
    }
    let pos = parse_legend_pos(&states.legend_pos);
    chart
        .configure_series_labels()
        .position(pos)
        .label_font(("sans-serif", 15))
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
