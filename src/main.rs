use std::str::FromStr;
use slidy::puzzle::{
    color_scheme::Scheme, coloring::{Coloring, Rainbow, RainbowFull, RainbowBright, RainbowBrightFull}, label::{label::{Label, Trivial, RowGrids, Rows, Fringe, FringeGrids, SquareFringe, SplitFringe, SplitSquareFringe, Diagonals, LastTwoRows, SplitLastTwoRows, ConcentricRectangles, Spiral, SpiralGrids}, rect_partition::{Rect, RectPartition}}, puzzle::Puzzle,
    render::{Renderer, Text, Borders},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = String::from("1 2 3 4/5 6 7 8/9 10 11 12/13 14 15 0"))]
    state: String,

    #[arg(short, long, default_value_t = String::from("SplitFringe"))]
    label: String,

    #[arg(short, long, default_value_t = String::from("RainbowBright"))]
    color: String,

    #[arg(short, long)]
    notext: bool,

    #[arg(short, long)]
    border: bool,

    #[arg(short, long, default_value_t = 40.0)]
    fontsize: f32,
}

macro_rules! make_label {
    ($arg: expr, $($label: expr),+) => {
        match $arg {
            $(stringify!($label) => return Some(Box::new($label)),)+
            _ => {},
        }
    }
}

fn parse_color(arg: &str) -> Option<Box<dyn Coloring>>{ //todo more color schemes....
    make_label!(arg, Rainbow, RainbowFull, RainbowBright, RainbowBrightFull);
    None
}

fn parse_label(arg: &str) -> Option<Box<dyn Label>> {
    make_label!(arg, Trivial, RowGrids, Rows, Fringe, FringeGrids, SquareFringe, SplitFringe, SplitSquareFringe, Diagonals, LastTwoRows, SplitLastTwoRows, ConcentricRectangles, Spiral, SpiralGrids);
        
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
    let args = Args::parse();
    let p = Puzzle::from_str(&args.state)?;
    let label = parse_label(&args.label).expect("Invalid label");
    let coloring = parse_color(&args.color).expect("Invalid color scheme");
    let notext = args.notext;
    let border = args.border;
    let fontsize = args.fontsize;

    let mut renderer = Renderer::new()
        .scheme(Scheme::new(
            label,
            coloring
        ));
    if border{
        let thickness = 1.5;
        renderer = renderer.borders(Borders::new().thickness(thickness)).tile_gap(-thickness);
    }
    if !notext{
        renderer = renderer.text(Text::new().font_size(fontsize));
    }
    
    let svg = renderer.svg(&p)?; 
    svg::save("out.svg", &svg)?;

    Ok(())
}