mod cli;
mod grid;
mod renderer;
mod simulation;

use clap::Parser;
use cli::Args;
use grid::Grid;
use renderer::render;
use simulation::{Metaball, update_balls, update_grid};

use std::io::stdout;
use std::time::{Duration, Instant};

use crossterm::{
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};

use rand::{RngExt, rng};

use crossterm::cursor::{Hide, Show};

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut stdout = stdout();

    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let (width, height) = terminal::size()?;
    let mut grid = Grid::new(width as usize, height as usize * 2);

    let w = grid.width as f32;
    let h = grid.height as f32;

    let mut rng = rng();

    let mut balls = Vec::new();
    for _ in 0..args.balls {
        balls.push(Metaball {
            x: rng.random_range(0.0..w),
            y: rng.random_range(0.0..h),
            vx: rng.random_range(-args.speed..args.speed),
            vy: rng.random_range(-args.speed..args.speed),
            radius: rng.random_range(args.min_radius..args.max_radius),
        });
    }

    let frame_time = Duration::from_millis(1000 / args.fps);
    let mut time: f32 = 0.0;

    loop {
        let frame_start = Instant::now();

        if poll(Duration::from_millis(1))?
            && let Event::Key(key) = read()?
            && key.code == KeyCode::Char('q')
        {
            break;
        }

        time += 0.03;

        update_balls(&mut balls, grid.width as f32, grid.height as f32);
        update_grid(&mut grid, &balls);

        render(&grid, time, &mut stdout)?;

        let elapsed = frame_start.elapsed();

        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }
    }

    execute!(stdout, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
