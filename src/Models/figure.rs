use crate::Models::cell::Cell;
use crate::Models::{ColorRGB, RelativeCoords};
use std::iter;

pub struct AbstractFigure {
    relativeCoords: Vec<RelativeCoords>,
    centerCell: Cell,
}

pub struct Figure {
    cells: Vec<Cell>,
    color: ColorRGB,
    figure: AbstractFigure,
}

impl AbstractFigure {
    pub fn new(relativeCoords: Vec<RelativeCoords>, centerCell: Cell) -> Self {
        AbstractFigure {
            relativeCoords,
            centerCell,
        }
    }
}

impl Figure {
    pub fn from_abstr(cell: Cell, color: ColorRGB, figure: AbstractFigure) -> Self {
        Figure {
            cells: iter::repeat(cell)
                .take(figure.relativeCoords.len() + 1)
                .collect(),
            color,
            figure,
        }
    }
}
