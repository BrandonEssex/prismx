use super::base::Rect;

/// Split an area into an evenly sized grid.
/// Returns a vector of Rect cells filled row by row.
pub fn split_grid(area: Rect, columns: u16, rows: u16) -> Vec<Rect> {
    if columns == 0 || rows == 0 {
        return Vec::new();
    }

    let cell_w = area.width / columns;
    let cell_h = area.height / rows;
    let mut cells = Vec::with_capacity((columns * rows) as usize);

    for r in 0..rows {
        for c in 0..columns {
            let mut w = cell_w;
            let mut h = cell_h;
            if c == columns - 1 {
                w = area.width - cell_w * (columns - 1);
            }
            if r == rows - 1 {
                h = area.height - cell_h * (rows - 1);
            }
            cells.push(Rect::new(area.x + c * cell_w, area.y + r * cell_h, w, h));
        }
    }

    cells
}
