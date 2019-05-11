use std::fs::File;

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 16;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Edge {
    Open,
    Closed,
    Unknown,
}

pub struct Maze<C: Copy> {
    horizontal_edges: [[Edge; HEIGHT - 1]; WIDTH],
    vertical_edges: [[Edge; WIDTH]; WIDTH - 1],
    cells: [[C; HEIGHT]; WIDTH],
}

impl<C: Copy> Maze<C> {
    pub fn new(cell: C, edge: Edge) -> Maze<C> {
        Maze {
            horizontal_edges: [[edge; HEIGHT - 1]; WIDTH],
            vertical_edges: [[edge; HEIGHT]; WIDTH - 1],
            cells: [[cell; HEIGHT]; WIDTH],
        }
    }

    /**
     *  Reads files in the format described by
     *  http://www.micromouseonline.com/2018/01/31/micromouse-maze-file-collection/
     */
    pub fn from_file(cell: C, bytes: [u8; WIDTH * HEIGHT]) -> Maze<C> {
        let mut horizontal_edges = [[Edge::Unknown; WIDTH - 1]; WIDTH];
        let mut vertical_edges = [[Edge::Unknown; WIDTH]; HEIGHT - 1];

        for (i, byte) in bytes.iter().enumerate() {
            let y = i % WIDTH;
            let x = i / WIDTH;

            let north = if byte & 0x01 == 0x01 {
                Edge::Closed
            } else {
                Edge::Open
            };
            let east = if byte & 0x02 == 0x02 {
                Edge::Closed
            } else {
                Edge::Open
            };

            if y < HEIGHT - 1 {
                horizontal_edges[x][y] = north;
            }

            if x < WIDTH - 1 {
                vertical_edges[x][y] = east;
            }
        }

        Maze {
            horizontal_edges,
            vertical_edges,
            cells: [[cell; HEIGHT]; WIDTH],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> (C, Edge, Edge, Edge, Edge) {
        let north_edge = if y >= HEIGHT - 1 {
            Edge::Closed
        } else {
            self.horizontal_edges[x][y]
        };

        let south_edge = if y <= 0 {
            Edge::Closed
        } else {
            self.horizontal_edges[x][y - 1]
        };

        let east_edge = if x >= WIDTH - 1 {
            Edge::Closed
        } else {
            self.vertical_edges[x][y]
        };

        let west_edge = if x <= 0 {
            Edge::Closed
        } else {
            self.vertical_edges[x - 1][y]
        };

        let cell = self.cells[x][y];

        (cell, north_edge, south_edge, east_edge, west_edge)
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: C) {
        self.cells[x][y] = cell;
    }
}
