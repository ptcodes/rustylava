use clap::Parser;

#[derive(Parser)]
#[command(about = "Terminal lava lamp simulation")]
pub struct Args {
    /// Number of metaballs
    #[arg(short, long, default_value_t = 25)]
    pub balls: usize,

    /// Minimum ball radius
    #[arg(long, default_value_t = 5.0)]
    pub min_radius: f32,

    /// Maximum ball radius
    #[arg(long, default_value_t = 20.0)]
    pub max_radius: f32,

    /// Maximum speed (balls move in range [-speed, speed])
    #[arg(short, long, default_value_t = 0.5)]
    pub speed: f32,

    /// Target frames per second
    #[arg(short, long, default_value_t = 30)]
    pub fps: u64,
}
