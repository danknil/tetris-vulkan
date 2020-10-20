use crate::Models::{
    cell::{Cell, CellState},
    figure::{AbstractFigure, Figure},
};
use std::iter;

pub(crate) struct TetrisMap {
    map: Vec<Vec<CellState>>,
    activeFigure: Option<Figure>,
    figures: Vec<AbstractFigure>,
}

impl TetrisMap {
    pub fn new(figures: Vec<AbstractFigure>, cellsize: u8, width: u8, height: u8) -> Self {
        let map = iter::repeat(CellState::Empty(Cell::new(cellsize, (0, 0))));
        let map: Vec<Vec<CellState>> = iter::repeat(map.take(height as usize).collect())
            .take(width as usize)
            .collect();
        TetrisMap {
            map,
            activeFigure: None,
            figures,
        }
    }
}
