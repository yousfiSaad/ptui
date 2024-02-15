
use ratatui::{
    prelude::Frame,
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Widget, block::Title }, layout::{Layout, Direction, Constraint, Flex},
};

use crate::app::App;

use chrono::{Datelike, NaiveDateTime, Utc, TimeZone, Local};

fn format_time(t: u128) -> String{
    let dt = Utc.from_utc_datetime(
        &NaiveDateTime::from_timestamp_millis(t as i64)
            .unwrap()
    );
    return dt
        .with_timezone(&Local)
        .format("%H:%M:%S (%Z)").to_string();
}

fn format_duration(duration: u128) -> String{
    let mut str = "".to_string();
    let minutes = duration/(1000 * 60);
    if minutes >= 60{
        str = (minutes / 60).to_string() + " hours and "
    }
    return str + &(minutes%60).to_string() + " minutes remaining"
}

fn create_one_bar_widget(app: &App, pos: Option<usize>) -> impl Widget{
    let posi = pos.unwrap_or(app.ipos);
    let t1 = if app.show_fasting_time {app.prayer_times[app.previous_icha + 1].time}
                else {app.prayer_times[posi].time};
    let t2 = if app.show_fasting_time {app.prayer_times[app.previous_icha + 6].time} 
                else {app.prayer_times[posi + 1].time};
    // let t2 = app.prayer_times[posi + 1].time;

    let title = if app.show_fasting_time {" Fasting time ".to_owned()}
                else { " ".to_string() + &app.prayer_times[posi + 1].name + " "};

    let r: f64 = if app.now < t1 {
        0.0
    } else if app.now > t2 {
        1.0
    } else {
        ((app.now - t1) as f64)/(t2 - t1) as f64
    };

    let color = if is_friday() {Color::Green} else {Color::Cyan};

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(
                    Title::from(title)
                        .alignment(ratatui::layout::Alignment::Center)
                )
        )
        .gauge_style(
            Style::default()
                .fg(color)
                .bg(Color::Black)
        )
        .ratio(r)
        .label(if t2 > app.now {
            if app.show_remaining_time {
                format_duration(t2- app.now)
            } else {
                format_time(t2)
            }
        } else {format_time(t2)});
        // .percent(20);
    return gauge;
}

fn is_friday() -> bool{
    let now = chrono::offset::Local::now();
    return now.weekday() == chrono::Weekday::Fri;
}

pub fn render(app: &mut App, f: &mut Frame) {
    if app.ipos < 1 {
        return;
    }

    let show_full = f.size().height > 35 && !app.show_fasting_time;
    let layout_1 = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(0),
            Constraint::Length(5),
            Constraint::Min(0),
        ],
    ).flex(Flex::Center).split(f.size());

    let layout_7 = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
            ],
        ).flex(Flex::Center).split(f.size());

    if show_full {
        let mut i = 0;
        while i < 7 {
            f.render_widget(
                create_one_bar_widget(app, Some(app.previous_icha + i)),
                layout_7[i]
            );
            i += 1;
        }
    } else {
        f.render_widget(
            create_one_bar_widget(app, None),
            layout_1[1]
        );
    }

}
