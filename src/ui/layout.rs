use crate::models::test::AnswerModel;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame,
};


pub fn get_adaptative_column(f: Rect) -> Rect {
    let mut x = 0;
    let mut x_w = f.width;
    if f.width > 100 {
        x = f.width / 2 - 50;
        x_w = 100;
    }

    Rect::new(x, f.y, x_w, f.height)
}

pub fn get_column_with_margin(f: Rect, margin: u16, threshold: u16) -> Rect {
    let mut x = 0;
    let mut x_w = f.width;
    if f.width > threshold {
        x_w = f.width * (100 - margin) / 100;
        x = f.width *  margin / 200;
    }

    Rect::new(x, f.y, x_w, f.height)
}

pub fn get_header_navbar_layout(f: Rect, header: u16, navbar: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header), 
            Constraint::Length(navbar), 
            Constraint::Min(0)
        ].as_ref())
        .split(f)
}

pub fn get_header_body_layout(f: Rect, header: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header), 
            Constraint::Min(0)
        ].as_ref())
        .split(f)
}

pub fn get_basic_layout<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(5),
            Constraint::Percentage(85)
            ].as_ref())
        .split(f.size())
}

pub fn get_two_row_layout(f: Rect, first: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(first),
            Constraint::Percentage(100 - first)
            ].as_ref())
        .split(f)
}

pub fn get_three_row_layout(f: Rect, first: u16, second: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(first),
            Constraint::Percentage(second),
            Constraint::Percentage(100 - first - second)
            ].as_ref())
        .split(f)
}

pub fn get_three_col_layout(f: Rect, middle: u16) -> Vec<Rect> {
    let side = (100 - middle) / 2;
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(side),
            Constraint::Percentage(middle),
            Constraint::Percentage(side)
            ].as_ref())
        .split(f)
}

pub fn get_background() -> Block<'static> {
    Block::default()
        .style(Style::default().bg(Color::Black))
}

pub fn create_navigable_list<'a>(items: Vec<String>) -> List<'a> {
    let list_items: Vec<ListItem> = items.into_iter()
        .map(|i| ListItem::new(i))
        .collect();
    List::new(list_items)
        .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD)
            )
        .highlight_symbol(">> ")
}

pub fn get_par_with_colors(text: Vec<Spans>, fg: Color, bg: Color) -> Paragraph {
    Paragraph::new(text)
        .block(Block::default())
        .style(Style::default().fg(fg).bg(bg))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

pub fn get_par_with_borders(text: Vec<Spans>) -> Paragraph {
    Paragraph::new(text)
        .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
        .style(Style::default().fg(Color::White).bg(Color::Blue))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

pub fn get_header(text: Vec<Spans>) -> Paragraph {
    Paragraph::new(text)
        .block(Block::default())
        .style(Style::default().fg(Color::Blue).bg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

pub fn get_navbar<'a>(text: Vec<(&'a str, &'a str)>) -> Paragraph<'a> {
    let mut nb = vec![
        get_navbar_element("[Navigation:]", false),
        Span::raw(" "),
    ];
    text.iter().for_each(|t| {
        nb.push(get_navbar_element(t.0, true));
        nb.push(get_navbar_element(t.1, false));
        nb.push(Span::styled(" // ", Style::default().add_modifier(Modifier::BOLD)));
    });
    nb.pop();
    let spans = vec![
        Spans::from(Span::raw("")),
        Spans::from(nb)
    ];
    Paragraph::new(spans)
        .block(Block::default())
        .style(Style::default().bg(Color::Blue))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

pub fn get_test_start_row<'a>(text: Vec<(&'a str, &'a str)>) -> Paragraph<'a> {
    let mut start: Vec<Span> = vec![];

    text.iter().for_each(|t| {
        start.push(get_navbar_element(t.0, true));
        start.push(get_navbar_element(t.1, false));
        start.push(Span::styled(" // ", Style::default().add_modifier(Modifier::BOLD)));
    });
    start.pop();
    Paragraph::new(Spans::from(start))
        .block(Block::default())
        .style(Style::default().bg(Color::Blue))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

fn get_navbar_element(text: &str, primary: bool) -> Span {
    if primary {
        Span::styled(text, Style::default().bg(Color::Black).fg(Color::White))
    } else {
        Span::styled(text, Style::default().bg(Color::White).fg(Color::Black))
    }
}

pub fn get_question_area<'a>(q_text: &'a str, qidx: usize, total: usize, q_time: u64, t_time: u64) -> Paragraph<'a> {
    let header = format!("QUESTION {} out of {}", qidx, total);
    let q_timer = format!("Question time elapsed: {}", format_time(q_time));
    let t_timer = format!("Test time elapsed: {}", format_time(t_time));
    let txt = vec![
        Spans::from(Span::raw("")),
        Spans::from(Span::styled(header, Style::default().add_modifier(Modifier::BOLD))),
        Spans::from(Span::raw("----------")),
        Spans::from(Span::styled(q_timer, Style::default().add_modifier(Modifier::ITALIC))),
        Spans::from(Span::styled(t_timer, Style::default().add_modifier(Modifier::ITALIC))),
        Spans::from(Span::raw("----------")),
        Spans::from(Span::raw("")),
        Spans::from(Span::styled(q_text, Style::default().bg(Color::White).fg(Color::Black))),
        Spans::from(Span::raw("")),
    ];

    Paragraph::new(txt)
        .block(Block::default()
            .borders(Borders::NONE)
        )
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}

pub fn render_summary_table<'a>(answers: Vec<AnswerModel>) -> Table<'a> {
    let mut idx = 0;
    let font_color = Color::White;
    let rows: Vec<Row> = answers.iter()
        .map(|a| {
            let mut correct = "No";
            if a.is_correct {
                correct = "Yes"
            }
            idx += 1;
            (format!(" #{:?}", idx), a.question.to_string(), correct.to_string(), format_time(a.time))
        })
        .map(|t| {
	    let mut color = Color::Red;
	    if t.2 == "Yes" {
        	color = Color::Green;
	    }
	    let result = Spans::from(vec![Span::raw(" "), Span::raw(t.2)]);
	    let timer = Spans::from(vec![Span::raw(" "), Span::raw(t.3)]);
	    Row::new(vec![
		Cell::from(t.0).style(Style::default().bg(color).fg(font_color)),
		Cell::from(t.1).style(Style::default().bg(color).fg(font_color)),
		Cell::from(timer).style(Style::default().bg(color).fg(font_color)),
		Cell::from(result).style(Style::default().bg(color).fg(font_color)),
	    ]).height(1)
	    .bottom_margin(1)
	})
        .collect();

    Table::new(rows)
        .style(Style::default())
        .header(
            Row::new(vec![" Number", " Question", " Time", " Correct"])
            .style(Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::White)
                .bg(Color::Blue)
                )
            .bottom_margin(1)
        ) 
        .block(Block::default())
        .widths(&[
            Constraint::Percentage(10), 
            Constraint::Percentage(45), 
            Constraint::Percentage(20),
            Constraint::Percentage(25)
        ])
        .column_spacing(1)
}

pub fn get_results_q_page(qidx: usize, total: usize, q_text: String, answers: Vec<Spans>, time: u64) -> Paragraph {
    let header = format!("QUESTION {} out of {}", qidx, total);
    let timer = format!("Answered in: {}", format_time(time));
    let mut txt = vec![
        Spans::from(Span::raw("")),
        Spans::from(Span::styled(header, Style::default().add_modifier(Modifier::BOLD))),
        Spans::from(Span::styled(timer, Style::default().add_modifier(Modifier::ITALIC))),
        Spans::from(Span::raw("")),
        Spans::from(Span::raw("----------------------------------------")),
        Spans::from(Span::styled(q_text, Style::default().bg(Color::White).fg(Color::Black))),
        Spans::from(Span::raw("----------------------------------------")),
        Spans::from(Span::raw("")),
    ];
    txt.extend(answers);

    Paragraph::new(txt)
        .block(Block::default()
            .borders(Borders::NONE)
        )
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}

fn format_time(time: u64) -> String {
    if time < 60 {
        return format!("00:{}", format_number(time));
    }

    let secs = time % 60;
    let mins = (time - secs) / 60;
    let secs_t = format_number(secs);
    let mins_t = format_number(mins);

    format!("{}:{}", mins_t, secs_t)
}

fn format_number(time: u64) -> String {
    if time < 10 {
        format!("0{}", time)
    } else {
        format!("{}", time)
    }
}


#[cfg(test)]
mod units {
    use super::*;

    #[test]
    fn test_format_timer_zero() {
        let expected = "00:00";

        let result = format_time(0);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_timer_single_digit() {
        let expected = "00:03";

        let result = format_time(3);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_timer_double_digit() {
        let expected = "00:49";

        let result = format_time(49);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_timer_with_munutes() {
        let expected = "02:28";

        let result = format_time(148);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_col_with_margin_above() {
        let given = Rect::new(0,0,120,20);
        let expected = Rect::new(6,0,108,20);

        let result = get_column_with_margin(given, 10, 50);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_col_with_margin_below() {
        let given = Rect::new(0,0,100,20);
        let expected = Rect::new(0,0,100,20);

        let result = get_column_with_margin(given, 10, 150);

        assert_eq!(result, expected);
    }
}


