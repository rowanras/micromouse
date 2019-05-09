use std::fs::File;

const MAZE_WIDTH: usize = 16;
const MAZE_HEIGHT: usize = 16;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Edge {
    Open,
    Closed,
    Unknown,
}

pub struct Maze<C: Copy> {
    horizontal_edges: [[Edge; MAZE_HEIGHT - 1]; MAZE_WIDTH],
    vertical_edges: [[Edge; MAZE_WIDTH]; MAZE_WIDTH - 1],
    cells: [[C; MAZE_HEIGHT]; MAZE_WIDTH],
}

impl<C: Copy> Maze<C> {
    pub fn new(cell: C, edge: Edge) -> Maze<C> {
        Maze {
            horizontal_edges: [[edge; MAZE_HEIGHT - 1]; MAZE_WIDTH],
            vertical_edges: [[edge; MAZE_HEIGHT]; MAZE_WIDTH - 1],
            cells: [[cell; MAZE_HEIGHT]; MAZE_WIDTH],
        }
    }

    /**
     *  Reads files in the format described by
     *  http://www.micromouseonline.com/2018/01/31/micromouse-maze-file-collection/
     */
    pub fn from_file(
        cell: C,
        bytes: [u8; MAZE_WIDTH * MAZE_HEIGHT],
    ) -> Maze<C> {
        let mut horizontal_edges =
            [[Edge::Unknown; MAZE_WIDTH - 1]; MAZE_WIDTH];
        let mut vertical_edges = [[Edge::Unknown; MAZE_WIDTH]; MAZE_HEIGHT - 1];

        for (i, byte) in bytes.iter().enumerate() {
            let y = i % MAZE_WIDTH;
            let x = i / MAZE_WIDTH;

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

            if y < MAZE_HEIGHT - 1 {
                horizontal_edges[x][y] = north;
            }

            if x < MAZE_WIDTH - 1 {
                vertical_edges[x][y] = east;
            }
        }

        Maze {
            horizontal_edges,
            vertical_edges,
            cells: [[cell; MAZE_HEIGHT]; MAZE_WIDTH],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> (C, Edge, Edge, Edge, Edge) {
        let north_edge = if y >= MAZE_HEIGHT - 1 {
            Edge::Closed
        } else {
            self.horizontal_edges[x][y]
        };

        let south_edge = if y <= 0 {
            Edge::Closed
        } else {
            self.horizontal_edges[x][y - 1]
        };

        let east_edge = if x >= MAZE_WIDTH - 1 {
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
}
