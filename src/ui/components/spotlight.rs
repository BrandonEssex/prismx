use ratatui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;
use crate::ui::animate::cursor_blink;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::ui::layout::Rect;
use crate::theme::layout::spotlight_width;

use crate::state::AppState;
use crate::spotlight::{command_preview, command_suggestions_scored};
use crate::spotlight::result::command_icon;
use crate::config::theme::ThemeConfig;

pub fn render_spotlight<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let input = &state.spotlight_input;
    let tick = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 300) as u64;
    let caret = cursor_blink(tick);
    let raw_input = if input.is_empty() { "<type command>" } else { input };
    let display_input = format!("{}{}", raw_input, caret);
    let cfg = ThemeConfig::load();
    let palette = cfg.spotlight_palette();

    // Use a fixed width so the Spotlight panel doesn't resize with input length.
    let width = spotlight_width(area.width);
    let x_offset = area.x + (area.width.saturating_sub(width)) / 2;
    let y_offset = area.y + area.height / 3;

    let preview = command_preview(input);
    let matches = command_suggestions_scored(input);
    let line_capacity = width.saturating_sub(2); // account for "> " prefix
    let input_width = UnicodeWidthStr::width(display_input.as_str()) as u16;
    let mut input_lines = (input_width + line_capacity - 1) / line_capacity;
    if input_lines == 0 { input_lines = 1; }
    let mut height = 2 + input_lines; // borders + input lines
    if preview.is_some() {
        height += 1;
    }
    // Ensure Spotlight stays above the status bar
    height = height.min(area.height.saturating_sub(1));
    let spotlight_area = Rect::new(x_offset, y_offset, width, height);

    let border_color = Color::Cyan;

    let block = Block::default()
        .title("Spotlight")
        .borders(Borders::ALL)
        .style(Style::default().fg(border_color).bg(palette.background));

    let spot_style = Style::default()
        .fg(palette.foreground)
        .bg(palette.background);
    let mut lines = vec![
        Line::styled(
            format!("> {}", display_input),
            spot_style.add_modifier(Modifier::BOLD),
        ),
    ];

    if let Some((msg, known)) = preview {
        if known {
            lines.push(Line::from(vec![
                Span::styled(
                    "→ ",
                    Style::default().fg(cfg.accent_color()).bg(palette.background),
                ),
                Span::styled(
                    msg,
                    Style::default().fg(palette.foreground).bg(palette.background),
                ),
            ]));
        } else {
            let style = Style::default().fg(Color::Red).bg(palette.background);
            lines.push(Line::from(vec![
                Span::styled("⚠ ", style),
                Span::styled(msg, style),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    // Clear background area and render base layer
    let clear_height = height + 10; // always clear room for suggestions
    let bg_rect = Rect::new(x_offset, y_offset, width, clear_height);
    f.render_widget(Clear, bg_rect);
    f.render_widget(
        Block::default().style(Style::default().bg(palette.background)),
        bg_rect,
    );

    f.render_widget(paragraph, spotlight_area);

    for (i, (suggestion, score)) in matches.iter().take(5).enumerate() {
        let y = y_offset + height - 1 + (i as u16 * 2);
        if y >= area.y + area.height.saturating_sub(2) {
            break;
        }
        let mut style = spot_style;
        if Some(i) == state.spotlight_suggestion_index {
            style = Style::default()
                .fg(palette.foreground)
                .bg(palette.active_background)
                .add_modifier(Modifier::BOLD);
        }
        let icon = command_icon(suggestion);
        let mut label_spans = vec![
            Span::styled(icon, style),
            Span::raw(" "),
            Span::styled(*suggestion, style),
        ];
        #[cfg(debug_assertions)]
        {
            label_spans.push(Span::raw(" "));
            label_spans.push(Span::styled(format!("{}", score), style.add_modifier(Modifier::DIM)));
        }
        let label_line = Line::from(label_spans);
        f.render_widget(Paragraph::new(label_line).style(style), Rect::new(x_offset, y, width, 1));

        if let Some((desc, _)) = command_preview(suggestion) {
            let desc_line = Line::from(vec![
                Span::raw("  "),
                Span::styled(desc, style.add_modifier(Modifier::DIM)),
            ]);
            f.render_widget(
                Paragraph::new(desc_line)
                    .style(style)
                    .wrap(Wrap { trim: true }),
                Rect::new(x_offset, y + 1, width, 1),
            );
        }
    }
}
