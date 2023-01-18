use tui::backend::Backend;
use tui::Frame;
use tui_logger::TuiLoggerWidget;
use tui::layout::{
    Alignment,
    Constraint,
    Direction,
    Layout,
    Rect
};
use tui::text::{
    Span,
    Spans, Text
};
use tui::widgets::{
    Block,
    BorderType,
    Borders,
    Paragraph,
    List,
    ListItem,
    ListState,
    Gauge, Table, Cell, Row, TableState, Clear,
};
use crate::constants::{
    APP_TITLE,
    MIN_TERM_WIDTH,
    MIN_TERM_HEIGHT,
    NO_OF_BOARDS_PER_PAGE,
    DEFAULT_BOARD_TITLE_LENGTH,
    DEFAULT_CARD_TITLE_LENGTH,
    NO_OF_CARDS_PER_BOARD,
    LIST_SELECT_STYLE,
    LIST_SELECTED_SYMBOL,
    CARD_DUE_DATE_STYLE,
    CARD_ACTIVE_STATUS_STYLE,
    FOCUSED_ELEMENT_STYLE,
    NON_FOCUSED_ELEMENT_STYLE,
    HELP_KEY_STYLE,
    HELP_DESCRIPTION_STYLE,
    LOG_ERROR_STYLE,
    LOG_DEBUG_STYLE,
    LOG_WARN_STYLE,
    LOG_TRACE_STYLE,
    LOG_INFO_STYLE,
    DEFAULT_STYLE,
    PROGRESS_BAR_STYLE,
    ERROR_TEXT_STYLE,
    INACTIVE_TEXT_STYLE,
    VERTICAL_SCROLL_BAR_SYMBOL,
    CARD_COMPLETED_STATUS_STYLE,
    CARD_STALE_STATUS_STYLE
};

use super::{
    MainMenuItem,
    App,
    MainMenu
};
use super::state::{
    Focus,
    AppStatus,
};
use crate::io::data_handler::{
    get_config,
    get_available_local_savefiles
};

/// Draws main screen with kanban boards
pub fn render_zen_mode<'a,B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(rect.size());

    render_body(rect, chunks[0], app,);
}

pub fn render_title_body<'a,B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(80),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let title = draw_title(&app.focus, false);
    rect.render_widget(title, chunks[0]);
    
    render_body(rect, chunks[1], app);
}

pub fn render_body_help<'a,B>(rect: &mut Frame<B>, app: &App, help_state: &mut TableState, keybind_store: Vec<Vec<String>>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(85),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[1]);
    
    render_body(rect, chunks[0], app);

    let help = draw_help(&app.focus, false, keybind_store);
    let help_separator = Block::default().borders(Borders::LEFT);
    rect.render_widget(help.0, chunks[1]);
    rect.render_stateful_widget(help.1, help_chunks[0], help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(help.2, help_chunks[2], help_state);
}

pub fn render_body_log<'a,B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(rect.size());

    render_body(rect, chunks[0], app);

    let log = draw_logs(&app.focus, true, false);
    rect.render_widget(log, chunks[1]);
}

pub fn render_title_body_help<'a,B>(rect: &mut Frame<B>, app: &App, help_state: &mut TableState, keybind_store: Vec<Vec<String>>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(75),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(rect.size());

        let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[2]);

    let title = draw_title(&app.focus, false);
    rect.render_widget(title, chunks[0]);

    render_body(rect, chunks[1], app);

    let help = draw_help(&app.focus, false, keybind_store);
    let help_separator = Block::default().borders(Borders::LEFT);
    rect.render_widget(help.0, chunks[2]);
    rect.render_stateful_widget(help.1, help_chunks[0], help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(help.2, help_chunks[2], help_state);
}

pub fn render_title_body_log<'a,B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(75),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let title = draw_title(&app.focus, false);
    rect.render_widget(title, chunks[0]);

    render_body(rect, chunks[1], app);

    let log = draw_logs(&app.focus, true, false);
    rect.render_widget(log, chunks[2]);
}

pub fn render_body_help_log<'a,B>(rect: &mut Frame<B>, app: &App, help_state: &mut TableState, keybind_store: Vec<Vec<String>>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Length(5),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(rect.size());

        let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[1]);

    render_body(rect, chunks[0], app);

    let help = draw_help(&app.focus, false, keybind_store);
    let help_separator = Block::default().borders(Borders::LEFT);
    rect.render_widget(help.0, chunks[1]);
    rect.render_stateful_widget(help.1, help_chunks[0], help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(help.2, help_chunks[2], help_state);

    let log = draw_logs(&app.focus, true, false);
    rect.render_widget(log, chunks[2]);
}

pub fn render_title_body_help_log<'a,B>(rect: &mut Frame<B>, app: &App, help_state: &mut TableState, keybind_store: Vec<Vec<String>>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(60),
                Constraint::Length(5),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(rect.size());

        let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[2]);

    let title = draw_title(&app.focus, false);
    rect.render_widget(title, chunks[0]);

    render_body(rect, chunks[1], app);

    let help = draw_help(&app.focus, false, keybind_store);
    let help_separator = Block::default().borders(Borders::LEFT);
    rect.render_widget(help.0, chunks[2]);
    rect.render_stateful_widget(help.1, help_chunks[0], help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(help.2, help_chunks[2], help_state);

    let log = draw_logs(&app.focus, true, false);
    rect.render_widget(log, chunks[3]);
}

pub fn render_config<'a,B>(rect: &mut Frame<B>, app: &App, config_state: &mut TableState, popup_mode: bool)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(60),
                Constraint::Length(3),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(rect.size());
    
    let title = draw_title(&app.focus, popup_mode);
    rect.render_widget(title, chunks[0]);
    
    let config = draw_config_table_selector(popup_mode);
    rect.render_stateful_widget(config, chunks[1], config_state);

    let config_help = draw_config_help(&app.focus, popup_mode);
    rect.render_widget(config_help, chunks[2]);

    let log = draw_logs(&app.focus, true, popup_mode);
    rect.render_widget(log, chunks[3]);
}

pub fn render_edit_config<'a,B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    
    let edit_box_style = if app.state.status == AppStatus::UserInput {
        FOCUSED_ELEMENT_STYLE
    } else {
        DEFAULT_STYLE
    };
    
    let area = centered_rect(70, 70, rect.size());
    let clear_area = centered_rect(80, 80, rect.size());
    let clear_area_border = Block::default()
        .borders(Borders::ALL)
        .border_style(FOCUSED_ELEMENT_STYLE)
        .title("Config Editor");
    rect.render_widget(Clear, clear_area);
    rect.render_widget(clear_area_border, clear_area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Length(4),
            ]
            .as_ref(),
        )
        .split(area);

    let config_item_index = &app.config_item_being_edited.unwrap_or(0);
    let list_items = get_config_items();
    let config_item_name = list_items[*config_item_index].first().unwrap();
    let config_item_value = list_items.iter()
        .find(|x| x.first().unwrap() == config_item_name).unwrap()
        .get(1).unwrap();
    let paragraph_text = format!("Current Value is {}\n\n{}",config_item_value,
        "Press 'i' to edit, or 'Esc' to cancel, Press 'Enter' to stop editing and press 'Enter' again to save");
    let paragraph_title = Spans::from(vec![Span::raw(config_item_name)]);
    let config_item = Paragraph::new(paragraph_text)
        .block(Block::default().borders(Borders::ALL).title(paragraph_title))
        .wrap(tui::widgets::Wrap { trim: false });
    let edit_item = Paragraph::new(&*app.current_user_input)
        .block(Block::default().borders(Borders::ALL).title("Edit").border_style(edit_box_style))
        .wrap(tui::widgets::Wrap { trim: false });

    let log = draw_logs(&app.focus, true, false);
    
    if app.state.status == AppStatus::UserInput {
        rect.set_cursor(
            // Put cursor past the end of the input text
            chunks[1].x + app.current_user_input.len() as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[1].y + 1,
        );
    }
    rect.render_widget(config_item, chunks[0]);
    rect.render_widget(edit_item, chunks[1]);
    rect.render_widget(log, chunks[2]);
}

pub fn render_edit_keybindings<'a,B>(rect: &mut Frame<B>, app: &App, edit_keybindings_state: &mut TableState, popup_mode: bool)
where
    B: Backend,
{
    let default_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let reset_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if matches!(app.focus, Focus::SubmitButton) {
            ERROR_TEXT_STYLE
        } else {
            DEFAULT_STYLE
        }
    };
    let current_element_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        FOCUSED_ELEMENT_STYLE
    };

    let title_bar = draw_title(&app.focus, popup_mode);

    let mut table_items: Vec<Vec<String>> = Vec::new();
    // app.config.keybindings
    let keybindings = app.config.keybindings.clone();
    for (key, value) in keybindings.iter() {
        let mut row: Vec<String> = Vec::new();
        row.push(key.to_string());
        let mut row_value = String::new();
        for v in value.iter() {
            row_value.push_str(&v.to_string());
            row_value.push_str(" ");
        }
        row.push(row_value);
        table_items.push(row);
    }

    let rects = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(65),
            Constraint::Length(5),
            Constraint::Length(3)
            ].as_ref())
        .split(rect.size());

    let rows = table_items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.to_string()));
        Row::new(cells).height(height as u16)
    });
    let t = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Edit Keybindings").style(default_style))
        .highlight_style(current_element_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);

    let next_focus_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Focus next")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let prev_focus_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Focus previous")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    let edit_keybind_help_spans = Spans::from(vec![
        Span::raw("Use "),
        Span::styled("<Up>", current_element_style),
        Span::raw(" and "),
        Span::styled("<Down>", current_element_style),
        Span::raw(" to select a keybinding, "),
        Span::styled("<Enter>", current_element_style),
        Span::raw(" to edit, "),
        Span::styled("<Esc>", current_element_style),
        Span::raw(" to cancel, To Reset Keybindings to Default, Press "),
        Span::styled([next_focus_key, prev_focus_key].join(" or "), current_element_style),
        Span::raw("to highlight Reset Button and Press "),
        Span::styled("<Enter>", current_element_style),
        Span::raw(" on the Reset Keybindings Button"),
    ]);
    
    let edit_keybind_help = Paragraph::new(edit_keybind_help_spans)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .style(default_style)
        .wrap(tui::widgets::Wrap { trim: false });
        
    let reset_button = Paragraph::new("Reset Keybindings to Default")
        .block(Block::default().borders(Borders::ALL).title("Reset"))
        .style(reset_style)
        .alignment(Alignment::Center);
        
    rect.render_widget(title_bar, rects[0]);
    rect.render_stateful_widget(t, rects[1], edit_keybindings_state);
    rect.render_widget(edit_keybind_help, rects[2]);
    rect.render_widget(reset_button, rects[3]);
}

pub fn render_edit_specific_keybinding<'a,B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let edit_box_style = if app.state.status == AppStatus::KeyBindMode {
        FOCUSED_ELEMENT_STYLE
    } else {
        DEFAULT_STYLE
    };

    let area = centered_rect(70, 70, rect.size());
    let clear_area = centered_rect(80, 80, rect.size());
    let clear_area_border = Block::default()
        .borders(Borders::ALL)
        .border_style(FOCUSED_ELEMENT_STYLE)
        .title("Edit Keybindings");
    rect.render_widget(Clear, clear_area);
    rect.render_widget(clear_area_border, clear_area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Length(4),
            ]
            .as_ref(),
        )
        .split(area);

    let key_id = app.state.edit_keybindings_state.selected().unwrap_or(0);
    let current_bindings = app.config.keybindings.clone();
    let mut key_list = vec![];
    
    for (k, v) in current_bindings.iter() {
        key_list.push((k, v));
    }

    if key_id > key_list.len() {
        return;
    } else {
        let key = key_list[key_id].0;
        let value = key_list[key_id].1;
        let mut key_value = String::new();
        for v in value.iter() {
            key_value.push_str(&v.to_string());
            key_value.push_str(" ");
        }
        let paragraph_text = format!("Current Value for {} \n\n{}",key,
            "Press 'i' to edit, or 'Esc' to cancel, Press 'Enter' to stop editing and press 'Enter' again to save");
        let paragraph_title = key.to_uppercase();
        let config_item = Paragraph::new(paragraph_text)
        .block(Block::default().borders(Borders::ALL).title(paragraph_title))
        .wrap(tui::widgets::Wrap { trim: false });
        let current_edited_keybinding = app.state.edited_keybinding.clone();
        let mut current_edited_keybinding_string = String::new();
        if current_edited_keybinding.is_some() {
            for key in current_edited_keybinding.unwrap() {
                current_edited_keybinding_string.push_str(&key.to_string());
                current_edited_keybinding_string.push_str(" ");
            }
        }
        let edit_item = Paragraph::new(current_edited_keybinding_string.clone())
        .block(Block::default().borders(Borders::ALL).title("Edit").border_style(edit_box_style))
        .wrap(tui::widgets::Wrap { trim: false });
    
        let log = draw_logs(&app.focus, true, false);
        
        if app.state.status == AppStatus::KeyBindMode {
            rect.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + current_edited_keybinding_string.len() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            );
        }
        rect.render_widget(config_item, chunks[0]);
        rect.render_widget(edit_item, chunks[1]);
        rect.render_widget(log, chunks[2]);
    }
}

pub fn render_main_menu<'a,B>(rect: &mut Frame<B>, app: &App, main_menu_state: &mut ListState, help_state: &mut TableState, keybind_store: Vec<Vec<String>>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(16),
                Constraint::Min(8),
                Constraint::Length(8)
            ]
            .as_ref(),
        )
        .split(rect.size());

    let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[2]);
    
    let title = draw_title(&app.focus, false);
    rect.render_widget(title, chunks[0]);
    
    let main_menu = draw_main_menu(&app.focus, MainMenu::all());
    rect.render_stateful_widget(main_menu, chunks[1], main_menu_state);

    let main_menu_help = draw_help(&app.focus, false, keybind_store);
    let help_separator = Block::default().borders(Borders::LEFT);
    rect.render_widget(main_menu_help.0, chunks[2]);
    rect.render_stateful_widget(main_menu_help.1, help_chunks[0], help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(main_menu_help.2, help_chunks[2], help_state);

    let log = draw_logs(&app.focus, true, false);
    rect.render_widget(log, chunks[3]);
}

pub fn render_help_menu<'a,B>(rect: &mut Frame<B>, app: &App, help_state: &mut TableState, keybind_store: Vec<Vec<String>>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Length(4)
            ]
            .as_ref(),
        )
        .split(rect.size());

    let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[0]);

    let help_menu = draw_help(&app.focus, false, keybind_store);
    let help_separator = Block::default().borders(Borders::LEFT);
    rect.render_widget(help_menu.0, chunks[0]);
    rect.render_stateful_widget(help_menu.1, help_chunks[0], help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(help_menu.2, help_chunks[2], help_state);

    let log = draw_logs(&app.focus, true, false);
    rect.render_widget(log, chunks[1]);
}

pub fn render_logs_only<'a,B>(rect: &mut Frame<B>, focus: &Focus)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(rect.size());
    let log = draw_logs(focus, false, false);
    rect.render_widget(log, chunks[0]);
}

/// Draws Help section for normal mode
fn draw_help<'a>(focus: &Focus, popup_mode: bool, keybind_store: Vec<Vec<String>>) -> (Block<'a>,Table<'a>,Table<'a>) {
    
    let default_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if *focus == Focus::Help {
            FOCUSED_ELEMENT_STYLE
        } else {
            DEFAULT_STYLE
        }
    };

    let current_element_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        FOCUSED_ELEMENT_STYLE
    };

    let rows = keybind_store.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.to_string()));
        Row::new(cells).height(height as u16)
    });

    // split the rows into two tables
    let left_rows = rows.clone().take(rows.clone().count() / 2);
    let right_rows = rows.clone().skip(rows.clone().count() / 2);

    let left_table = Table::new(left_rows)
        .block(Block::default())
        .highlight_style(current_element_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);

    let right_table = Table::new(right_rows)
        .block(Block::default())
        .highlight_style(current_element_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);

    let border_block = Block::default().borders(Borders::ALL).border_style(default_style).title("Help");

    (border_block, left_table, right_table)
}

/// Draws help section for config mode
fn draw_config_help(focus: &Focus, popup_mode: bool) -> Paragraph {
    let helpbox_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if matches!(focus, Focus::ConfigHelp) {
            FOCUSED_ELEMENT_STYLE
        } else {
            NON_FOCUSED_ELEMENT_STYLE
        }
    };
    let key_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        HELP_KEY_STYLE
    };
    let description_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        HELP_DESCRIPTION_STYLE
    };

    let mut help_spans = vec![];
    let keys_span = Span::styled("<Up>, <Down>", key_style);
    let action_span = Span::styled("Select config option", description_style);
    help_spans.push(keys_span);
    help_spans.push(Span::raw(" - "));
    help_spans.push(action_span);
    help_spans.push(Span::raw(" ; "));
    let keys_span = Span::styled("<Enter>", key_style);
    let action_span = Span::styled("Edit config option", description_style);
    help_spans.push(keys_span);
    help_spans.push(Span::raw(" - "));
    help_spans.push(action_span);
    let keys_span = Span::styled("<Esc>", key_style);
    let action_span = Span::styled("Exit config mode", description_style);
    help_spans.push(Span::raw(" ; "));
    help_spans.push(keys_span);
    help_spans.push(Span::raw(" - "));
    help_spans.push(action_span);

    let help_span = Spans::from(help_spans);

    Paragraph::new(help_span)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .style(helpbox_style)
                .border_type(BorderType::Plain),
        )
        .wrap(tui::widgets::Wrap { trim: true })
}

/// Draws logs
fn draw_logs<'a>(focus: &Focus, enable_focus_highlight: bool, popup_mode: bool) -> TuiLoggerWidget<'a> {
    let logbox_style = if matches!(focus, Focus::Log) && enable_focus_highlight {
            FOCUSED_ELEMENT_STYLE
        } else {
            NON_FOCUSED_ELEMENT_STYLE
        };
    if popup_mode {
        TuiLoggerWidget::default()
            .style_error(INACTIVE_TEXT_STYLE)
            .style_debug(INACTIVE_TEXT_STYLE)
            .style_warn(INACTIVE_TEXT_STYLE)
            .style_trace(INACTIVE_TEXT_STYLE)
            .style_info(INACTIVE_TEXT_STYLE)
            .output_file(false)
            .output_line(false)
            .output_target(false)
            .block(
                Block::default()
                    .title("Logs")
                    .border_style(INACTIVE_TEXT_STYLE)
                    .borders(Borders::ALL),
            )
    } else {
        TuiLoggerWidget::default()
            .style_error(LOG_ERROR_STYLE)
            .style_debug(LOG_DEBUG_STYLE)
            .style_warn(LOG_WARN_STYLE)
            .style_trace(LOG_TRACE_STYLE)
            .style_info(LOG_INFO_STYLE)
            .output_file(false)
            .output_line(false)
            .output_target(false)
            .block(
                Block::default()
                    .title("Logs")
                    .border_style(logbox_style)
                    .borders(Borders::ALL),
            )
        }
}

/// Draws Main menu
fn draw_main_menu<'a>(focus: &Focus, main_menu_items: Vec<MainMenuItem>) -> List<'a> {
    let menu_style = if matches!(focus, Focus::MainMenu) {
        FOCUSED_ELEMENT_STYLE
    } else {
        NON_FOCUSED_ELEMENT_STYLE
    };
    let list_items = main_menu_items
        .iter()
        .map(|i| ListItem::new(i.to_string()))
        .collect::<Vec<ListItem>>();
    List::new(list_items)
        .block(
            Block::default()
                .title("Main menu")
                .borders(Borders::ALL)
                .border_style(menu_style)
                .border_type(BorderType::Plain),
        )
        .highlight_style(LIST_SELECT_STYLE)
        .highlight_symbol(LIST_SELECTED_SYMBOL)
}

/// Draws config list selector
fn draw_config_table_selector(popup_mode: bool) -> Table<'static> {
    let default_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let current_element_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        FOCUSED_ELEMENT_STYLE
    };

    let config_list = get_config_items();
    let rows = config_list.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.to_string()));
        Row::new(cells).height(height as u16)
    });
    Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Config Editor").style(default_style))
        .highlight_style(current_element_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ])
}

/// returns a list of all config items as a vector of strings
fn get_config_items() -> Vec<Vec<String>>
{
    let config = get_config();
    let config_list = config.to_list();
    return config_list;
}

/// Draws Kanban boards
pub fn render_body<'a,B>(rect: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let focus = &app.focus;
    let boards = &app.boards;
    let current_board = &app.state.current_board_id.unwrap_or(0);

    let add_board_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Create new board")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    // check if self.visible_boards_and_cards is empty
    if app.visible_boards_and_cards.is_empty() {
        let empty_paragraph = Paragraph::new(
            ["No boards found, press ".to_string(), add_board_key, " to add a new board".to_string()]
            .concat())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Boards")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain),
            )
            .wrap(tui::widgets::Wrap { trim: true });
        rect.render_widget(empty_paragraph, area);
        return;
    }
    
    // make a list of constraints depending on NO_OF_BOARDS_PER_PAGE constant
    let chunks = if app.config.disable_scrollbars {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(100),
                    ]
                    .as_ref(),
                )
                .split(area)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(99),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(area)
        };
    let mut constraints = vec![];
    // check if length of boards is more than NO_OF_BOARDS_PER_PAGE
    if boards.len() > NO_OF_BOARDS_PER_PAGE.into() {
        for _i in 0..NO_OF_BOARDS_PER_PAGE {
            constraints.push(Constraint::Percentage(100 / NO_OF_BOARDS_PER_PAGE as u16));
        }
    } else {
        for _i in 0..boards.len() {
            constraints.push(Constraint::Percentage(100 / boards.len() as u16));
        }
    }
    let board_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(chunks[0]);
    // visible_boards_and_cards: Vec<LinkedHashMap<String, Vec<String>>>
    let visible_boards_and_cards = app.visible_boards_and_cards.clone();
    for (board_index, board_and_card_tuple) in visible_boards_and_cards.iter().enumerate() {
        // render board with title in board chunks alongside with cards in card chunks of the board
        // break if board_index is more than NO_OF_BOARDS_PER_PAGE
        if board_index >= NO_OF_BOARDS_PER_PAGE.into() {
            break;
        }
        let board_id = board_and_card_tuple.0;
        // find index of board with board_id in boards
        let board = app.boards.iter().find(|&b| b.id == *board_id);
        // check if board is found if not continue
        if board.is_none() {
            continue;
        }
        let board = board.unwrap();
        let board_title = board.name.clone();
        let board_cards = board_and_card_tuple.1;
        // if board title is longer than DEFAULT_BOARD_TITLE_LENGTH, truncate it and add ... at the end
        let board_title = if board_title.len() > DEFAULT_BOARD_TITLE_LENGTH.into() {
            format!("{}...", &board_title[0..DEFAULT_BOARD_TITLE_LENGTH as usize])
        } else {
            board_title
        };
        let board_title = format!("{} ({})", board_title, board.cards.len());
        let board_title = if *board_id as u128 == *current_board {
            format!("{} {}", ">>", board_title)
        } else {
            board_title
        };

        // check if length of cards is more than NO_OF_CARDS_PER_BOARD constant
        let mut card_constraints = vec![];
        if board_cards.len() > NO_OF_CARDS_PER_BOARD.into() {
            for _i in 0..NO_OF_CARDS_PER_BOARD {
                card_constraints.push(Constraint::Percentage(90 / NO_OF_CARDS_PER_BOARD as u16));
            }
        } else {
            for _i in 0..board_cards.len() {
                card_constraints.push(Constraint::Percentage(100 / board_cards.len() as u16));
            }
        }

        // check if board_index is >= board_chunks.len() if yes continue
        if board_index >= board_chunks.len() {
            continue;
        }

        let board_style = if *board_id == *current_board && matches!(focus, Focus::Body) && app.state.current_card_id == None {
            FOCUSED_ELEMENT_STYLE
        } else {
            NON_FOCUSED_ELEMENT_STYLE
        };
        
        let board_block = Block::default()
            .title(&*board_title)
            .borders(Borders::ALL)
            .style(board_style)
            .border_type(BorderType::Plain);
        rect.render_widget(board_block, board_chunks[board_index]);

        let card_area_chunks = if app.config.disable_scrollbars {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(100),
                    ]
                    .as_ref(),
                ).split(board_chunks[board_index])
        } else {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Percentage(99),
                    ]
                    .as_ref(),
                ).split(board_chunks[board_index])
            };
        
        let card_chunks = if app.config.disable_scrollbars {
                Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(card_constraints.as_ref())
                    .split(card_area_chunks[0])
            } else {
                Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(card_constraints.as_ref())
                    .split(card_area_chunks[1])
            };

        if !app.config.disable_scrollbars {
            // calculate the current card scroll percentage
            // get the index of current card in board_cards
            let all_board_cards = boards.iter().find(|&b| b.id == *board_id).unwrap().cards.clone();
            let current_card_index = all_board_cards.iter().position(|c| c.id == app.state.current_card_id.unwrap_or(0));
            let cards_scroll_percentage = (current_card_index.unwrap_or(0) + 1) as f64 / all_board_cards.len() as f64;
            let cards_scroll_percentage = cards_scroll_percentage.clamp(0.0, 1.0);
            let available_height = if card_area_chunks[0].height >= 2 {
                (card_area_chunks[0].height - 2) as f64
            } else {
                0.0
            };
            // calculate number of blocks to render
            let blocks_to_render = (available_height * cards_scroll_percentage) as u16;
            // render blocks VERTICAL_SCROLL_BAR_SYMBOL
            if all_board_cards.len() > 0 {
                for i in 0..blocks_to_render {
                    let block = Paragraph::new(VERTICAL_SCROLL_BAR_SYMBOL)
                        .style(PROGRESS_BAR_STYLE)
                        .block(Block::default().borders(Borders::NONE));
                    rect.render_widget(block, Rect::new(card_area_chunks[0].x, card_area_chunks[0].y + i + 1, card_area_chunks[0].width, 1));
                }
            }
        };
        for (card_index, card_id) in board_cards.iter().enumerate() {
            if card_index >= NO_OF_CARDS_PER_BOARD.into() {
                break;
            }
            // unwrap card if panic skip it and log it
            let mut card = board.get_card(*card_id);
            // check if card is None, if so skip it and log it
            if card.is_none() {
                continue;
            } else {
                card = Some(card.unwrap());
            }
            let card_title = card.unwrap().name.clone();
            let card_title = if card_title.len() > DEFAULT_CARD_TITLE_LENGTH.into() {
                format!("{}...", &card_title[0..DEFAULT_CARD_TITLE_LENGTH as usize])
            } else {
                card_title
            };

            let card_title = if app.state.current_card_id.unwrap_or(0) == *card_id {
                format!("{} {}", ">>", card_title)
            } else {
                card_title
            };

            let mut card_description = Text::from(card.unwrap().description.clone());
            let card_due_date = card.unwrap().date_due.clone();
            if !card_due_date.is_empty() {
                let card_due_date_styled = Text::styled(
                    format!("Due: {}",card_due_date), CARD_DUE_DATE_STYLE);
                card_description.extend(card_due_date_styled);
            }
            let card_status = format!("Status: {}",card.unwrap().card_status.clone().to_string());
            let card_status = if card_status == "Status: Active" {
                Text::styled(card_status, CARD_ACTIVE_STATUS_STYLE)
            } else if card_status == "Status: Complete" {
                Text::styled(card_status, CARD_COMPLETED_STATUS_STYLE)
            } else {
                Text::styled(card_status, CARD_STALE_STATUS_STYLE)
            };
            card_description.extend(card_status);

            // if card id is same as current_card, highlight it
            let card_style = if app.state.current_card_id.unwrap_or(0) == *card_id && matches!(focus, Focus::Body) && *board_id == *current_board {
                FOCUSED_ELEMENT_STYLE
            } else {
                NON_FOCUSED_ELEMENT_STYLE
            };

            let card_paragraph = Paragraph::new(card_description)
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .title(&*card_title)
                        .borders(Borders::ALL)
                        .style(card_style)
                        .border_type(BorderType::Plain),
                )
                .wrap(tui::widgets::Wrap { trim: false });

            rect.render_widget(card_paragraph, card_chunks[card_index]);

        }
    }

    if !app.config.disable_scrollbars {
        // draw line_gauge in chunks[1]
        // get the index of the current board in boards and set percentage
        let current_board_id = app.state.current_board_id.unwrap_or(0);
        // get the index of the board with the id
        let current_board_index = boards
            .iter()
            .position(|board| board.id == current_board_id)
            .unwrap_or(0) + 1;
        let percentage = (current_board_index as f64 / boards.len() as f64) * 100.0;
        let line_gauge = Gauge::default()
            .block(Block::default())
            .gauge_style(PROGRESS_BAR_STYLE)
            .percent(percentage as u16);
        rect.render_widget(line_gauge, chunks[1]);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);
    
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
    }
    
/// Draws size error screen if the terminal is too small
pub fn draw_size_error<B>(rect: &mut Frame<B>, size: &Rect, msg: String)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(*size);

    let title = draw_title(&Focus::default(), false);
    rect.render_widget(title, chunks[0]);

    let mut text = vec![Spans::from(Span::styled(msg, ERROR_TEXT_STYLE))];
    text.append(&mut vec![Spans::from(Span::raw("Resize the window to continue, or press 'q' to quit."))]);
    let body = Paragraph::new(text)
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center);
    rect.render_widget(body, chunks[1]);
}

pub fn draw_loading_screen<B>(rect: &mut Frame<B>, size: &Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(*size);

    let title = draw_title(&Focus::default(), false);
    rect.render_widget(title, chunks[0]);

    let text = vec![Spans::from(Span::styled(
        "Loading...... \n\n 
        `(*>﹏<*)′
        \n\nPlease wait",FOCUSED_ELEMENT_STYLE))];
    let body = Paragraph::new(text)
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center);
    rect.render_widget(body, chunks[1]);
}

/// Draws the title bar
pub fn draw_title<'a>(focus: &Focus, popup_mode: bool) -> Paragraph<'a> {
    // check if focus is on title
    let title_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if matches!(focus, Focus::Title) {
            FOCUSED_ELEMENT_STYLE
        } else {
            NON_FOCUSED_ELEMENT_STYLE
        }
    };
    Paragraph::new(APP_TITLE)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(title_style)
                .border_type(BorderType::Plain),
        )
}

/// Helper function to check terminal size
pub fn check_size(rect: &Rect) -> String {
    let mut msg = String::new();
    if rect.width < MIN_TERM_WIDTH {
        msg.push_str(&format!("For optimal viewing experience, Terminal width should be >= {}, (current width {})",MIN_TERM_WIDTH, rect.width));
    }
    else if rect.height < MIN_TERM_HEIGHT {
        msg.push_str(&format!("For optimal viewing experience, Terminal height should be >= {}, (current height {})",MIN_TERM_HEIGHT, rect.height));
    }
    else {
        msg.push_str("Size OK");
    }
    msg
}

pub fn render_new_board_form<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    // make a form for the Board struct
    // take name and description where description is optional
    // submit button

    let name_style = if matches!(app.focus, Focus::NewBoardName) {
        FOCUSED_ELEMENT_STYLE
    } else {
        NON_FOCUSED_ELEMENT_STYLE
    };
    let description_style = if matches!(app.focus, Focus::NewBoardDescription) {
        FOCUSED_ELEMENT_STYLE
    } else {
        NON_FOCUSED_ELEMENT_STYLE
    };
    let submit_style = if matches!(app.focus, Focus::SubmitButton) {
        FOCUSED_ELEMENT_STYLE
    } else {
        NON_FOCUSED_ELEMENT_STYLE
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(4),
            Constraint::Length(3),
            ].as_ref())
        .split(rect.size());

    let title_paragraph = Paragraph::new("Create a new Board")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        );
    rect.render_widget(title_paragraph, chunks[0]);

    let board_name_field = app.state.new_board_form[0].clone();
    let board_description_field = app.state.new_board_form[1].clone();
    let board_name = Paragraph::new(board_name_field)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(name_style)
                .border_type(BorderType::Plain)
                .title("Board Name (required)")
        );
    rect.render_widget(board_name, chunks[1]);

    let board_description = Paragraph::new(board_description_field)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(description_style)
                .border_type(BorderType::Plain)
                .title("Board Description")
        );
    rect.render_widget(board_description, chunks[2]);

    let input_mode_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Enter input mode")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let next_focus_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Focus next")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let prev_focus_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Focus previous")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    
    let help_text = Spans::from(vec![
        Span::styled("Press ", HELP_DESCRIPTION_STYLE),
        Span::styled(input_mode_key, HELP_KEY_STYLE),
        Span::styled("to start typing", HELP_DESCRIPTION_STYLE),
        Span::raw("; "),
        Span::styled("<Esc>", HELP_KEY_STYLE),
        Span::styled(" to stop typing", HELP_DESCRIPTION_STYLE),
        Span::raw("; "),
        Span::styled("Press ", HELP_DESCRIPTION_STYLE),
        Span::styled([next_focus_key, prev_focus_key].join(" or "), HELP_KEY_STYLE),
        Span::styled("to switch focus", HELP_DESCRIPTION_STYLE),
        Span::raw("; "),
        Span::styled("<Enter>", HELP_KEY_STYLE),
        Span::styled(" to submit", HELP_DESCRIPTION_STYLE),
        Span::raw("; "),
        Span::styled("<Esc>", HELP_KEY_STYLE),
        Span::styled(" to cancel", HELP_DESCRIPTION_STYLE),
    ]);
    let help_paragraph = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .wrap(tui::widgets::Wrap { trim: true });
    rect.render_widget(help_paragraph, chunks[3]);

    let submit_button = Paragraph::new("Submit")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(submit_style)
                .border_type(BorderType::Plain),
        );
    rect.render_widget(submit_button, chunks[4]);

    if app.focus == Focus::NewBoardName && app.state.status == AppStatus::UserInput{
        rect.set_cursor(
            // Put cursor past the end of the input text
            chunks[1].x + app.state.new_board_form[0].len() as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[1].y + 1,
        );
    } else if app.focus == Focus::NewBoardDescription && app.state.status == AppStatus::UserInput{
        rect.set_cursor(
            // Put cursor past the end of the input text
            chunks[2].x + app.state.new_board_form[1].len() as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[2].y + 1,
        );
    }
}

pub fn render_new_card_form<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let name_style = if matches!(app.focus, Focus::NewCardName) {
        FOCUSED_ELEMENT_STYLE
    } else {
        NON_FOCUSED_ELEMENT_STYLE
    };
    let description_style = if matches!(app.focus, Focus::NewCardDescription) {
        FOCUSED_ELEMENT_STYLE
    } else {
        NON_FOCUSED_ELEMENT_STYLE
    };
    let due_date_style = if matches!(app.focus, Focus::NewCardDueDate) {
        FOCUSED_ELEMENT_STYLE
    } else {
        NON_FOCUSED_ELEMENT_STYLE
    };
    let submit_style = if matches!(app.focus, Focus::SubmitButton) {
        FOCUSED_ELEMENT_STYLE
    } else {
        NON_FOCUSED_ELEMENT_STYLE
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Length(3),
            ].as_ref())
        .split(rect.size());

    let title_paragraph = Paragraph::new("Create a new Card")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        );
    rect.render_widget(title_paragraph, chunks[0]);

    let card_name_field = app.state.new_card_form[0].clone();
    let card_description_field = app.state.new_card_form[1].clone();
    let card_due_date_field = app.state.new_card_form[2].clone();
    let card_name = Paragraph::new(card_name_field)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(name_style)
                .border_type(BorderType::Plain)
                .title("Card Name (required)")
        );
    rect.render_widget(card_name, chunks[1]);

    let card_description = Paragraph::new(card_description_field)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(description_style)
                .border_type(BorderType::Plain)
                .title("Card Description")
        );
    rect.render_widget(card_description, chunks[2]);

    let card_due_date = Paragraph::new(card_due_date_field)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(due_date_style)
                .border_type(BorderType::Plain)
                .title("Card Due Date (DD/MM/YYYY)")
        );
    rect.render_widget(card_due_date, chunks[3]);

    let input_mode_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Enter input mode")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let next_focus_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Focus next")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let prev_focus_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Focus previous")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    
    let help_text = Spans::from(vec![
        Span::styled("Press ", HELP_DESCRIPTION_STYLE),
        Span::styled(input_mode_key, HELP_KEY_STYLE),
        Span::styled("to start typing", HELP_DESCRIPTION_STYLE),
        Span::raw("; "),
        Span::styled("<Esc>", HELP_KEY_STYLE),
        Span::styled(" to stop typing", HELP_DESCRIPTION_STYLE),
        Span::raw("; "),
        Span::styled("Press ", HELP_DESCRIPTION_STYLE),
        Span::styled([next_focus_key, prev_focus_key].join(" or "), HELP_KEY_STYLE),
        Span::styled("to switch focus", HELP_DESCRIPTION_STYLE),
        Span::raw("; "),
        Span::styled("<Enter>", HELP_KEY_STYLE),
        Span::styled(" to submit", HELP_DESCRIPTION_STYLE),
        Span::raw("; "),
        Span::styled("<Esc>", HELP_KEY_STYLE),
        Span::styled(" to cancel", HELP_DESCRIPTION_STYLE),
    ]);

    let help_paragraph = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .wrap(tui::widgets::Wrap { trim: true });
    rect.render_widget(help_paragraph, chunks[4]);

    let submit_button = Paragraph::new("Submit")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(submit_style)
                .border_type(BorderType::Plain),
        );
    rect.render_widget(submit_button, chunks[5]);

    if app.focus == Focus::NewCardName && app.state.status == AppStatus::UserInput{
        rect.set_cursor(
            // Put cursor past the end of the input text
            chunks[1].x + app.state.new_card_form[0].len() as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[1].y + 1,
        );
    } else if app.focus == Focus::NewCardDescription && app.state.status == AppStatus::UserInput{
        rect.set_cursor(
            // Put cursor past the end of the input text
            chunks[2].x + app.state.new_card_form[1].len() as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[2].y + 1,
        );
    } else if app.focus == Focus::NewCardDueDate && app.state.status == AppStatus::UserInput{
        rect.set_cursor(
            // Put cursor past the end of the input text
            chunks[3].x + app.state.new_card_form[2].len() as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[3].y + 1,
        );
    }
}

pub fn render_load_save<B>(rect: &mut Frame<B>, load_save_state: &mut ListState, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(70),
            Constraint::Length(3),
            ].as_ref())
        .split(rect.size());

    let title_paragraph = Paragraph::new("Load a Save")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        );
    rect.render_widget(title_paragraph, chunks[0]);

    let item_list = get_available_local_savefiles();
    if item_list.len() > 0 {
        // make a list from the Vec<string> of savefiles
        let items: Vec<ListItem> = item_list
            .iter()
            .map(|i| ListItem::new(i.to_string()))
            .collect();
        let choice_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Available Saves"))
            .highlight_style(LIST_SELECT_STYLE)
            .highlight_symbol(LIST_SELECTED_SYMBOL);
        rect.render_stateful_widget(choice_list, chunks[1], load_save_state);
    } else {
        let no_saves_paragraph = Paragraph::new("No saves found")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain),
            )
            .style(LOG_ERROR_STYLE);
        rect.render_widget(no_saves_paragraph, chunks[1]);
    }

    let delete_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Delete focused element")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    let up_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Go up")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let down_key = app.state.keybind_store.iter()
        .find(|x| x[1] == "Go down")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    let help_text = Spans::from(vec![
        Span::styled("Use ", HELP_DESCRIPTION_STYLE),
        Span::styled(up_key, HELP_KEY_STYLE),
        Span::styled(" and ", HELP_DESCRIPTION_STYLE),
        Span::styled(down_key, HELP_KEY_STYLE),
        Span::styled("to navigate", HELP_DESCRIPTION_STYLE),
        Span::raw("; "),
        Span::styled("<Enter>", HELP_KEY_STYLE),
        Span::styled(" to Load the save file", HELP_DESCRIPTION_STYLE),
        Span::raw("; "),
        Span::styled("<Esc>", HELP_KEY_STYLE),
        Span::styled(" to cancel", HELP_DESCRIPTION_STYLE),
        Span::raw("; "),
        Span::styled(delete_key, HELP_KEY_STYLE),
        Span::styled("to delete a save file", HELP_DESCRIPTION_STYLE),
    ]);
    let help_paragraph = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        );
    rect.render_widget(help_paragraph, chunks[2]);
}