use crate::Models::{ColorRGB, Coords};

#[derive(Clone)]
pub enum CellState {
    Filled(RenderedCell),
    Empty(Cell),
}

#[derive(Clone)]
pub struct Cell {
    size: u8,
    coords: Coords,
}

#[derive(Clone)]
pub struct RenderedCell {
    cell: Cell,
    color: ColorRGB,
}

impl Cell {
    pub fn new(size: u8, coords: Coords) -> Self {
        Cell { size, coords }
    }
}

impl RenderedCell {
    pub fn from_cell(color: ColorRGB, cell: Cell) -> Self {
        RenderedCell { color, cell }
    }
}
