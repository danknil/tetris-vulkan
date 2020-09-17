use std::convert::TryInto;
use std::iter;
use std::iter::{IntoIterator, Iterator};

type Coords = (i16, i16);
type Color = Vec<u8>;
type FigureCoords = Vec<(i16, i16)>;
type FigureState = dyn Fn(&mut FigureCoords) -> FigureCoords;

#[derive(Clone)]
pub struct Block {
    state: Option<Color>,
    coords: Coords,
    side: u8,
}

pub struct Figure {
    blocks: Vec<Block>,
    coords: Coords,
    figureCoords: FigureCoords,
}

pub struct TetrisMap {
    blocks: Vec<Vec<Block>>,
    figures: Vec<FigureCoords>,
    figureStates: Vec<Box<FigureState>>,
    figureOnMap: Option<Figure>,
    height: u16,
    width: u16,
    score: u128,
}

impl Block {
    fn new(color: Option<Color>, side: u8, coords: Coords) -> Self {
        Block {
            state: color,
            side,
            coords,
        }
    }
}

impl Figure {
    fn new(color: Color, coords: FigureCoords, blockSide: u8, startCoords: Coords) -> Self {
        let blocks: Vec<_> = iter::repeat(Block::new(Some(color), blockSide, startCoords))
            .take(coords.len())
            .collect();

        for i in 0..coords.len() {
            blocks[i].coords.0 += coords[i].0;
            blocks[i].coords.1 += coords[i].1;
        }

        Figure {
            blocks,
            coords: startCoords,
            figureCoords: coords,
        }
    }

    pub fn to_blocks(&self) -> Vec<Block> {
        self.blocks
    }

    // TODO make figure movement
}

impl TetrisMap {
    pub fn new(sizePerBlock: u8, figures: Vec<FigureCoords>, height: u16, width: u16) -> Self {
        assert!(height > 32767);
        assert!(width > 32767);
        let block = Block::new(None, sizePerBlock, (0, 0));

        let blocks = iter::repeat(iter::repeat(block).take(height.into()).collect())
            .take(width.into())
            .collect();

        for x in 0..blocks.len() {
            for y in 0..blocks[x].len() {
                blocks[x][y].coords = (x, y);
            }
        }
        // States for figures
        let states: Vec<Box<dyn Fn(&Coords) -> Coords>> = [
            |(x, y)| (x, y),
            |(x, y)| (y, x),
            |(x, y)| (-x, -y),
            |(x, y)| (-y, -x),
        ];

        // Transform states to get it for every coord
        let states: Vec<Box<dyn Fn(&mut FigureCoords) -> FigureCoords>> = {
            let mut temp = Vec::new();
            for i in 0..4 {
                temp.push(|u| u.iter().for_each(|x| states[i](x)).collect());
            }
            temp
        };

        TetrisMap {
            blocks,
            figures,
            figureStates: states,
            figureOnMap: None,
            width,
            height,
            score: 0,
        }
    }

    pub fn gen_figure(&self, color: Color, figureNumb: usize) -> Figure {
        // TODO random figure placement
        Figure::new(
            color,
            self.figures[figureNumb - 1],
            self.blocks[0][0].side,
            ((self.width + 1).try_into().unwrap(), 0),
        )
    }

    pub fn remove_lines(&mut self) {
        let counter: u8 = 0;
        for x in &mut self.blocks {
            if x.iter()
                .all(|y| if let None = y.state { false } else { true })
            {
                x.iter().map(|x| x.state = None);
                counter += 1;
            }
        }
        match counter {
            1 => self.score += 100,
            2 => self.score += 300,
            3 => self.score += 700,
            4 => self.score += 2500,
            _ => panic!("Strange remover behavior"),
        }
    }
}
