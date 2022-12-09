use std::str::FromStr;
use slidy::puzzle::{
    color_scheme::Scheme, coloring::RainbowBright, label::{label::{Label, SplitFringe, Rows}, rect_partition::{Rect, RectPartition}}, puzzle::Puzzle,
    render::{Renderer, Text},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    state: String,

    #[arg(short, long)]
    label: String,
}

macro_rules! make_label {
    ($arg: expr, $($label: expr),+) => {
        match $arg {
            $(stringify!($label) => return Some(Box::new($label)),)+
            _ => {},
        }
    }
}

fn parse_label(arg: &str) -> Option<Box<dyn Label>> {
    make_label!(arg, Rows, SplitFringe);

    match arg {
        "10x10" => Some(Box::new(
            RectPartition::new(
                vec![
                    Rect::new((0,0), (5,5)).unwrap(),
                    Rect::new((5,0), (10,5)).unwrap(),
                    Rect::new((0,5), (5,10)).unwrap(),
                    Rect::new((5,5), (10,10)).unwrap(),
                ]
            ).unwrap()
        )),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let grids = Box::new(
    //     RectPartition::new(
    //         vec![
    //             Rect::new((0,0), (5,5))?,
    //             Rect::new((5,0), (10,5))?,
    //             Rect::new((0,5), (5,10))?,
    //             Rect::new((5,5), (10,10))?,
    //         ]
    //     )?
    // );

    let args = Args::parse();

    let p = Puzzle::from_str(&args.state)?;
    let label = parse_label(&args.label).expect("Invalid label");

    let svg = Renderer::new()
        .scheme(Scheme::new(
            label,
            Box::new(RainbowBright)
        ))
        .text(Text::new())
        .svg(&p)?;

    svg::save("out.svg", &svg)?;

    Ok(())
}