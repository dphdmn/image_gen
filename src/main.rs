use std::str::FromStr;
use std::env;
use slidy::puzzle::{
    color_scheme::Scheme, coloring::RainbowBrightFull, label::label::SplitFringe, puzzle::Puzzle,
    render::{Renderer, Font, Text},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("Input scramble at least!");
        return Ok(());
    }
    let p = Puzzle::from_str(&args[1])?;
    let svg = Renderer::new()
        .scheme(Scheme::new(
            Box::new(SplitFringe),
            Box::new(RainbowBrightFull)
        ))
        .text(Text::new())
        .svg(&p)?;
    svg::save("out.svg", &svg)?;
    Ok(())
}