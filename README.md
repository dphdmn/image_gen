Version: 2.0 beta

Usage: image_gen.exe [OPTIONS]

Options:
-s, --state [default: "1 2 3 4/5 6 7 8/9 10 11 12/13 14 15 0"]
-l, --label [default: SplitFringe]
-c, --color [default: RainbowBright]
-n, --notext
-b, --border
-f, --fontsize [default: 30]
-h, --help Print help information

Labels supported atm:
Trivial,
RowGrids,
Rows,
Fringe,
FringeGrids,
SquareFringe,
SplitFringe,
SplitSquareFringe,
Diagonals,
LastTwoRows,
SplitLastTwoRows,
ConcentricRectangles,
Spiral,
SpiralGrids

Colors supported atm:
Rainbow,
RainbowFull,
RainbowBright,
RainbowBrightFull

Complex input example:
image_gen.exe --state "2 4 13 17 9/8 5 3 19 16/21 15 24 0 1/7 10 23 11 18/20 22 6 14 12" -l "Rows" --color "RainbowBrightFull" -b