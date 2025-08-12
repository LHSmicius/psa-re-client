use ratatui::{Frame, layout, style, text, widgets};

use crate::app;

pub fn ui(frame: &mut Frame, app: &app::App) {
    let chunks = layout::Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints([
            layout::Constraint::Length(3),
            layout::Constraint::Min(1),
            layout::Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = widgets::Block::default()
        .borders(widgets::Borders::ALL)
        .style(style::Style::default());

    let title_str = String::from("PSA-RE-TUI    ") + app.app_config.database_dir.as_str();
    let title = widgets::Paragraph::new(text::Text::styled(
        title_str,
        style::Style::default().fg(style::Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    // Center chunk
    let mut list_items = Vec::<widgets::ListItem>::new();

    for item in app.can_messages.clone() {
        list_items.push(widgets::ListItem::new(text::Line::from(
            text::Span::styled(
                format!(
                    "{: <10}|{: <30}|{: >4}|{: >2}",
                    item.id.unwrap(),
                    item.name.unwrap().chars().take(30).collect::<String>(),
                    item.periodicity.unwrap(),
                    item.length.unwrap(),
                ),
                style::Style::default().fg(style::Color::Yellow),
            ),
        )));
    }

    let list = widgets::List::new(list_items);

    frame.render_widget(list, chunks[1]);

    let current_keys_hint = {
        match app.active_screen {
            app::ActiveScreen::CanBus => text::Span::styled(
                "Quit[q] Nav[↑↓] New[n] Edit[e]",
                style::Style::default().fg(style::Color::Green),
            ),
            app::ActiveScreen::Editing => text::Span::styled(
                "Quit[q] Nav[↑↓] Select[s]",
                style::Style::default().fg(style::Color::Green),
            ),
        }
    };

    let key_notes_footer = widgets::Paragraph::new(text::Line::from(current_keys_hint))
        .block(widgets::Block::default().borders(widgets::Borders::ALL));

    frame.render_widget(key_notes_footer, chunks[2]);

    if let Some(editing) = &app.edit_window {
        let popup_block = widgets::Block::default()
            .title("Edit window")
            .borders(widgets::Borders::NONE)
            .style(style::Style::default().bg(style::Color::DarkGray));

        let area = centered_rect(80, 60, frame.area());
        frame.render_widget(popup_block, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: layout::Rect) -> layout::Rect {
    let popup_layout = layout::Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints([
            layout::Constraint::Percentage((100 - percent_y) / 2),
            layout::Constraint::Percentage(percent_y),
            layout::Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    layout::Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints([
            layout::Constraint::Percentage((100 - percent_x) / 2),
            layout::Constraint::Percentage(percent_x),
            layout::Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
