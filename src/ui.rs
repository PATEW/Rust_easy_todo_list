use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::app::{App, CalendarView};

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
    ], [ Constraint::Length(5), Constraint::Length(5)])
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

    // Calendar Table
    let (today_day, today_month, today_year) = &app.todays_day_month_year;
    let today_day: u32 = today_day.parse().unwrap_or(0);

    let view_data = app.get_current_view_data();
    let rows = match app.current_view {
        CalendarView::Year => {
            view_data.chunks(3).map(|chunk| {
                Row::new(chunk.iter().map(|(month, is_current)| {
                    let cell = Cell::from(month.clone());
                    if *is_current {
                        cell.style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                    } else {
                        cell.style(Style::default().fg(Color::Green))
                    }
                }))
            }).collect::<Vec<_>>()
        },
        CalendarView::Month => {
            (0..6).map(|i| {
                Row::new((0..7).map(|j| {
                    let index = i * 7 + j;
                    if let Some((day, is_current_month)) = view_data.get(index) {
                        let cell = Cell::from(day.clone());
                        if (i, j) == app.table_selected_cell {
                            cell.style(Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD))
                        } else if day == &today_day.to_string() && *is_current_month {
                            cell.style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                        } else if *is_current_month {
                            cell.style(Style::default().fg(Color::Green))
                        } else {
                            cell.style(Style::default().fg(Color::Gray))
                        }
                    } else {
                        Cell::from("  ").style(Style::default().fg(Color::Gray))
                    }
                }))
                .height(row_height as u16)
            }).collect()
        },
        CalendarView::Week => {
            vec![Row::new(view_data.into_iter().map(|(day, is_current_month)| {
                let cell = Cell::from(day);
                if is_current_month {
                    cell.style(Style::default().fg(Color::Green))
                } else {
                    cell.style(Style::default().fg(Color::Gray))
                }
            }))]
        },
        CalendarView::Day => {
            vec![Row::new(vec![Cell::from(view_data[0].0.clone()).style(Style::default().fg(Color::Green))])]
        },
    };

    let calendar_title = match app.current_view {
        CalendarView::Year => format!("Year View - {}", app.currently_selected_date.format("%Y")),
        CalendarView::Month => format!("Month View - {}", app.currently_selected_date.format("%B %Y")),
        CalendarView::Week => format!("Week View - Week of {}", app.currently_selected_date.format("%B %d, %Y")),
        CalendarView::Day => format!("Day View - {}", app.currently_selected_date.format("%B %d, %Y")),
    };

    let calendar_table = Table::new(rows, [ Constraint::Length(5), Constraint::Length(5)])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(calendar_title)
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
        f - zoom in\n\
        d - zoom out\n\
        o - previous (month/week/year)\n\
        p - next (month/week/year)\n\
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

    let additional_info = Paragraph::new("XYZ Requirements Document\nIntegration Testing\nAI/ML homework #2\nApp UI Overhaul")
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