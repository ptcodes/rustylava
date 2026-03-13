use crate::grid::Grid;

pub struct Metaball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
}

fn clamp_and_bounce(pos: &mut f32, vel: &mut f32, min: f32, max: f32) {
    if *pos < min {
        *pos = min;
        *vel = vel.abs();
    } else if *pos > max {
        *pos = max;
        *vel = -vel.abs();
    }
}

pub fn update_balls(balls: &mut [Metaball], width: f32, height: f32) {
    for b in balls {
        b.x += b.vx;
        b.y += b.vy;
        clamp_and_bounce(&mut b.x, &mut b.vx, b.radius, width - b.radius);
        clamp_and_bounce(&mut b.y, &mut b.vy, b.radius, height - b.radius);
    }
}

fn field_value(x: f32, y: f32, balls: &[Metaball]) -> f32 {
    let mut value = 0.0;

    for b in balls {
        let dx = x - b.x;
        let dy = y - b.y;

        let dist2 = dx * dx + dy * dy + 0.0001;

        value += (b.radius * b.radius) / dist2;
    }

    value - 0.8
}

pub fn update_grid(grid: &mut Grid, balls: &[Metaball]) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let v = field_value(x as f32, y as f32, balls) * 0.5;
            grid.set(x, y, v);
        }
    }
}
