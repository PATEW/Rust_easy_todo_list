use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::{app::App, calendar};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // Define a layout with three vertical chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(50), Constraint::Percentage(20)].as_ref())
        .split(frame.size());

    // Define a layout with two horizontal chunks within the first vertical chunk
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[0]);

    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);

    // Define a layout with two horizontal chunks within the third vertical chunk
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[2]);

    // First Paragraph
    let information_paragraph = Paragraph::new(format!(
        "Press left and right to change the counter.\n\
            Counter: {}",
        app.counter
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Information")
            .title_alignment(Alignment::Center)
            .style(if app.focused_chunk == 0 {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Green)
            }),
    )
    .style(Style::default().fg(Color::Green).bg(Color::Black))
    .alignment(Alignment::Left);

    // Second Table in the second top chunk
    let kanban_table = Table::new(vec![
        Row::new(vec![Cell::from("Backburner"), Cell::from("Todo"), Cell::from("In Progress"), Cell::from("Done")])
            .style(Style::default().fg(Color::Yellow)),
        Row::new(vec![Cell::from(""), Cell::from(""), Cell::from(""), Cell::from("")]),
        Row::new(vec![Cell::from(""), Cell::from(""), Cell::from(""), Cell::from("")]),
        Row::new(vec![Cell::from(""), Cell::from(""), Cell::from(""), Cell::from("")])
    ], [Constraint::Length(5), Constraint::Length(5)])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Kanban Board")
            .title_alignment(Alignment::Center)
            .style(Style::default()),
    )
    .widths(&[
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .style(Style::default().fg(Color::Cyan).bg(Color::Black));

    // Calculate the row height to fit within the available space
    let total_available_height = middle_chunks[1].height;
    let row_height = total_available_height / 5; // 5 rows

    // Second Table
    let mut cell_counter = 1;
    let rows = (0..5).map(|i| {
        Row::new((0..7).map(|j| {
            let content = format!("{}", cell_counter);
            let cell = if (i, j) == app.table_selected_cell {
                Cell::from(format!("[{}]", content)).style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Cell::from(content).style(Style::default().fg(Color::Green).bg(Color::Black))
            };
            cell_counter += 1;
            cell
        }).collect::<Vec<_>>())
        .height(row_height as u16)
    }).collect::<Vec<_>>();

    let calendar_table = Table::new(rows, [Constraint::Length(5), Constraint::Length(5)])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Calendar")
                .title_alignment(Alignment::Center)
                .style(if app.focused_chunk == 1 {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::Green)
                }),
        )
        .widths(&[
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
        ])
        .column_spacing(1)
        .header(
            Row::new(["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"])
                .style(Style::default().fg(Color::Green))
                .bottom_margin(1),
        );

    let controls = Paragraph::new(format!(
        "h j k l - < v ^ > Movement\n\
        f - forward / accept\n\
        d - back / decline\n\
        c - create new assignment\n\
        s - switch panes\n\
        q - quit application"
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Controls")
            .title_alignment(Alignment::Center)
            .style(Style::default()),
    )
    .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    .alignment(Alignment::Left);

    let additional_info = Paragraph::new("SE530 Requirements Document\nCEC470 Compiler Design")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Assignment List")
                .title_alignment(Alignment::Center)
                .style(Style::default()),
        )
        .style(Style::default().fg(Color::Magenta).bg(Color::Black))
        .alignment(Alignment::Left);

    // Render the paragraphs in the first chunk, split horizontally
    frame.render_widget(information_paragraph, top_chunks[0]);
    frame.render_widget(kanban_table, top_chunks[1]);

    // Render the calendar_table in the second chunk
    frame.render_widget(calendar_table, chunks[1]);

    // Render the controls and additional info in the third chunk, split horizontally
    frame.render_widget(controls, bottom_chunks[0]);
    frame.render_widget(additional_info, bottom_chunks[1]);
}
