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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ball(x: f32, y: f32, vx: f32, vy: f32, radius: f32) -> Metaball {
        Metaball { x, y, vx, vy, radius }
    }

    // clamp_and_bounce tests (tested via update_balls)

    #[test]
    fn ball_moves_by_velocity() {
        let mut balls = vec![make_ball(10.0, 10.0, 2.0, -3.0, 1.0)];
        update_balls(&mut balls, 100.0, 100.0);
        assert_eq!(balls[0].x, 12.0);
        assert_eq!(balls[0].y, 7.0);
    }

    #[test]
    fn ball_bounces_off_right_wall() {
        // Place ball near right wall with positive vx so it would exceed max
        let mut balls = vec![make_ball(98.0, 50.0, 5.0, 0.0, 2.0)];
        update_balls(&mut balls, 100.0, 100.0);
        // After update: x = 103 > 98 (100 - radius), so clamp to 98 and reverse vx
        assert_eq!(balls[0].x, 98.0);
        assert!(balls[0].vx < 0.0);
    }

    #[test]
    fn ball_bounces_off_left_wall() {
        let mut balls = vec![make_ball(3.0, 50.0, -5.0, 0.0, 4.0)];
        update_balls(&mut balls, 100.0, 100.0);
        // After update: x = -2 < 4 (radius), clamp to 4 and flip vx positive
        assert_eq!(balls[0].x, 4.0);
        assert!(balls[0].vx > 0.0);
    }

    #[test]
    fn ball_bounces_off_bottom_wall() {
        let mut balls = vec![make_ball(50.0, 97.0, 0.0, 5.0, 3.0)];
        update_balls(&mut balls, 100.0, 100.0);
        assert_eq!(balls[0].y, 97.0);
        assert!(balls[0].vy < 0.0);
    }

    #[test]
    fn ball_bounces_off_top_wall() {
        let mut balls = vec![make_ball(50.0, 2.0, 0.0, -5.0, 3.0)];
        update_balls(&mut balls, 100.0, 100.0);
        assert_eq!(balls[0].y, 3.0);
        assert!(balls[0].vy > 0.0);
    }

    #[test]
    fn field_value_is_higher_near_ball_center() {
        let balls = vec![make_ball(5.0, 5.0, 0.0, 0.0, 10.0)];
        // Cell directly on ball center should have high field value
        let near = field_value(5.0, 5.0, &balls);
        // Cell far from ball
        let far = field_value(50.0, 50.0, &balls);
        assert!(near > far);
    }

    #[test]
    fn field_value_no_balls_returns_negative_threshold() {
        let value = field_value(0.0, 0.0, &[]);
        assert_eq!(value, -0.8);
    }

    #[test]
    fn update_grid_populates_all_cells() {
        let mut grid = Grid::new(5, 5);
        let balls = vec![make_ball(2.0, 2.0, 0.0, 0.0, 5.0)];
        update_grid(&mut grid, &balls);
        // At least some cells should be non-zero
        assert!(grid.cells.iter().any(|&v| v != 0.0));
    }

    #[test]
    fn update_grid_center_cell_higher_than_edge() {
        let mut grid = Grid::new(10, 10);
        let balls = vec![make_ball(5.0, 5.0, 0.0, 0.0, 10.0)];
        update_grid(&mut grid, &balls);
        let center = grid.get(5, 5);
        let edge = grid.get(0, 0);
        assert!(center > edge);
    }
}
