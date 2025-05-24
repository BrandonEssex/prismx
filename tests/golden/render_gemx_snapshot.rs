use prismx::{screen::render_gemx, state::AppState};
use ratatui::{backend::TestBackend, buffer::Buffer, prelude::*, Terminal};
use std::fs;

fn buffer_from_file(path: &str) -> Buffer {
    let contents = fs::read_to_string(path).expect("snapshot file");
    let lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
    Buffer::with_lines(lines)
}

fn buffer_to_string(buf: &Buffer) -> String {
    let mut lines = Vec::new();
    for y in 0..buf.area.height {
        let mut line = String::new();
        for x in 0..buf.area.width {
            line.push_str(buf.get(x, y).symbol.as_ref());
        }
        lines.push(line);
    }
    let text = lines.join("\n");
    text.replace('·', "✦")
}

#[test]
fn gemx_renders_correctly() {
    let area = Rect::new(0, 0, 50, 10);
    let backend = TestBackend::new(area.width, area.height);
    let mut terminal = Terminal::new(backend).unwrap();
    let state = AppState::default();
    let completed = terminal
        .draw(|f| render_gemx(f, area, &state))
        .unwrap();
    let buffer = completed.buffer.clone();
    let expected = buffer_from_file("tests/golden/gemx.snapshot");
    assert_eq!(buffer_to_string(&buffer), buffer_to_string(&expected));
}
