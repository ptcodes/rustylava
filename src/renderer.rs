use std::io::Write;

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

use crate::grid::Grid;

fn lava_color(v: f32) -> Color {
    let t = (v * 0.35).clamp(0.0, 1.0);

    let r = (255.0 * t) as u8;
    let g = (200.0 * t * t) as u8;
    let b = (40.0 * t * t * t) as u8;

    Color::Rgb { r, g, b }
}

pub fn render(grid: &Grid, time: f32, stdout: &mut impl Write) -> std::io::Result<()> {
    for (term_row, y) in (0..grid.height.saturating_sub(1)).step_by(2).enumerate() {
        queue!(stdout, MoveTo(0, term_row as u16))?;

        for x in 0..grid.width {
            let sin_x = (x as f32 * 0.12 + time).sin();
            let top = grid.get(x, y) + (sin_x + (y as f32 * 0.15 + time * 1.3).cos()) * 0.15;
            let bottom = grid.get(x, y + 1)
                + (sin_x + ((y + 1) as f32 * 0.15 + time * 1.3).cos()) * 0.15;

            queue!(
                stdout,
                SetForegroundColor(lava_color(top)),
                SetBackgroundColor(lava_color(bottom)),
                Print("▀")
            )?;
        }
    }

    queue!(stdout, ResetColor)?;
    stdout.flush()?;

    Ok(())
}
