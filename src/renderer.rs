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
            let bottom =
                grid.get(x, y + 1) + (sin_x + ((y + 1) as f32 * 0.15 + time * 1.3).cos()) * 0.15;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lava_color_zero_is_black() {
        let Color::Rgb { r, g, b } = lava_color(0.0) else {
            panic!("expected Rgb variant");
        };
        assert_eq!((r, g, b), (0, 0, 0));
    }

    #[test]
    fn lava_color_clamps_large_input() {
        // v=100 * 0.35 = 35, clamped to 1.0 → same as v where v*0.35 == 1.0
        let Color::Rgb { r: r1, g: g1, b: b1 } = lava_color(100.0) else {
            panic!("expected Rgb variant");
        };
        let Color::Rgb { r: r2, g: g2, b: b2 } = lava_color(3.0) else {
            panic!("expected Rgb variant");
        };
        assert_eq!((r1, g1, b1), (r2, g2, b2));
    }

    #[test]
    fn lava_color_red_dominates_at_low_values() {
        // At low (but nonzero) t, red > green > blue due to polynomial degrees
        let Color::Rgb { r, g, b } = lava_color(1.0) else {
            panic!("expected Rgb variant");
        };
        assert!(r >= g);
        assert!(g >= b);
    }

    #[test]
    fn render_writes_output() {
        let grid = Grid::new(4, 4);
        let mut buf: Vec<u8> = Vec::new();
        render(&grid, 0.0, &mut buf).unwrap();
        assert!(!buf.is_empty());
    }
}
