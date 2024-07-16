use crate::{app::{App, AppResult, CalendarView}, file_reader_writer::DataIO};
use crossterm::event::{KeyCode, KeyEvent};
use chrono::{NaiveDate, Datelike};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `q`
        KeyCode::Char('q') => {
            app.save_data();
            app.quit();
        }
        // Switch focus between chunks on Tab
        KeyCode::Char('s') => {
            app.focused_chunk = (app.focused_chunk + 1) % 2;
        }
        // Counter handlers in the first chunk
        KeyCode::Right => {
            if app.focused_chunk == 0 {
                app.increment_counter();
            }
        }
        KeyCode::Left => {
            if app.focused_chunk == 0 {
                app.decrement_counter();
            }
        }
        // Table cell navigation in the second chunk using h, j, k, l
        KeyCode::Char('l') => {
            if app.focused_chunk == 1 {
                app.table_selected_cell.1 = (app.table_selected_cell.1 + 1).min(6);
            }
        }
        KeyCode::Char('h') => {
            if app.focused_chunk == 1 {
                app.table_selected_cell.1 = app.table_selected_cell.1.saturating_sub(1);
            }
        }
        KeyCode::Char('j') => {
            if app.focused_chunk == 1 {
                app.table_selected_cell.0 = (app.table_selected_cell.0 + 1).min(5);
            }
        }
        KeyCode::Char('k') => {
            if app.focused_chunk == 1 {
                app.table_selected_cell.0 = app.table_selected_cell.0.saturating_sub(1);
            }
        }
        KeyCode::Char('o') => {
            if app.focused_chunk == 1 {
                app.navigate_previous();
            }
        }
        KeyCode::Char('p') => {
            if app.focused_chunk == 1 {
                app.navigate_next();
            }
        }
        KeyCode::Char('f') => {
            if app.focused_chunk == 1 {
                app.zoom_in();
            }
        }
        KeyCode::Char('d') => {
            if app.focused_chunk == 1 {
                app.zoom_out();
            }
        }
        _ => {}
    }
    Ok(())
}