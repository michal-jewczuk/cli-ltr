use crate::models::test::AnswerModel;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, BorderType, Cell, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame,
};

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

pub fn get_two_row_layout<B: Backend>(f: &mut Frame<B>, first: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(first),
            Constraint::Percentage(100 - first)
            ].as_ref())
        .split(f.size())
}

pub fn get_two_row_layout_rect(f: Rect, first: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(first),
            Constraint::Percentage(100 - first)
            ].as_ref())
        .split(f)
}

pub fn get_three_row_layout_rect(f: Rect, first: u16, second: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(first),
            Constraint::Percentage(second),
            Constraint::Percentage(100 - first - second)
            ].as_ref())
        .split(f)
}

pub fn get_three_col_layout_rect(f: Rect, middle: u16) -> Vec<Rect> {
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

pub fn create_navigable_list<'a>(items: Vec<&'a str>) -> List<'a> {
    let list_items: Vec<ListItem> = items.iter()
        .map(|&i| ListItem::new(i))
        .collect();
    List::new(list_items)
        .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
            .bg(Color::Magenta)
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

pub fn get_header(text: Vec<Spans>) -> Paragraph {
    Paragraph::new(text)
        .block(Block::default())
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
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
        .style(Style::default().bg(Color::Magenta))
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

pub fn render_question<'a>(text: &'a str) -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from(Span::raw("")),
        Spans::from(Span::from(text)),
    ])
        .block(Block::default()
            .title("Question")
            .borders(Borders::ALL)
            .border_type(BorderType::Double))
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}

pub fn render_summary_table<'a>(answers: Vec<AnswerModel>) -> Table<'a> {
    let mut idx = 0;
    let rows: Vec<Row> = answers.iter()
        .map(|a| {
            let mut correct = "No";
            if a.is_correct {
                correct = "Yes"
            }
            idx += 1;
            (format!(" #{:?}", idx), a.question.to_string(), correct.to_string())
        })
        .map(|t| {
	    let mut color = Color::Red;
	    if t.2 == "Yes" {
        	color = Color::Green;
	    }
	    let result = Spans::from(vec![Span::raw(" "), Span::raw(t.2)]);
	    Row::new(vec![
		Cell::from(t.0).style(Style::default().bg(color)),
		Cell::from(t.1).style(Style::default().bg(color)),
		Cell::from(result).style(Style::default().bg(color)),
	    ]).height(1)
	    .bottom_margin(1)
	})
        .collect();

    Table::new(rows)
        .style(Style::default())
        .header(
            Row::new(vec![" Number", " Question", " Correct"])
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
            Constraint::Percentage(65), 
            Constraint::Percentage(25)
        ])
        .column_spacing(1)
}

pub fn get_results_q_page(qidx: usize, total: usize, q_text: String, answers: Vec<Spans>) -> Paragraph {
    let header = format!("QUESTION {} out of {}", qidx, total);
    let timer = String::from("Answered in: 00:15");
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

