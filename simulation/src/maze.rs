// Constants for now until Rust gets const generics
// https://github.com/rust-lang/rfcs/pull/2000
const MAZE_WIDTH: usize = 16;
const MAZE_HEGIHT: usize = 16;

pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone)]
pub enum Edge {
    Closed,
    Open,
    Unknown,
}

#[derive(Copy, Clone)]
struct SouthWestCellEdges {
    pub north_edge: Edge,
    pub east_edge: Edge,
}

impl SouthWestCellEdges {
    pub fn from_edge(edge: Edge) -> SouthWestCellEdges {
        SouthWestCellEdges {
            north_edge: edge,
            east_edge: edge,
        }
    }
}

#[derive(Copy, Clone)]
struct NorthCellEdges {
    pub east_edge: Edge,
}

impl NorthCellEdges {
    pub fn from_edge(edge: Edge) -> NorthCellEdges {
        NorthCellEdges { east_edge: edge }
    }
}

#[derive(Copy, Clone)]
struct EastCellEdges {
    pub north_edge: Edge,
}

impl EastCellEdges {
    pub fn from_edge(edge: Edge) -> EastCellEdges {
        EastCellEdges { north_edge: edge }
    }
}

#[derive(Copy, Clone)]
struct NorthEastCellEdges {}

impl NorthEastCellEdges {
    pub fn from_edge(_edge: Edge) -> NorthEastCellEdges {
        NorthEastCellEdges {}
    }
}

#[derive(Copy, Clone)]
pub struct FullCellEdges {
    pub north_edge: Edge,
    pub east_edge: Edge,
    pub south_edge: Edge,
    pub west_edge: Edge,
}

#[derive(Copy, Clone)]
pub struct PartialCellEdges {
    pub north_edge: Option<Edge>,
    pub east_edge: Option<Edge>,
    pub south_edge: Option<Edge>,
    pub west_edge: Option<Edge>,
}

// 1x1: 0
// 1x2: 1
// 1x3: 2
// 1x4: 3
// 2x2: 4
// 2x3: 7
// 2x4: 10
// nxm: (n-1)*m + (m-1)*n

pub struct Maze<C: Sized + Copy> {
    /**
     * All the cells that are not on the north or east edge
     * ```
     * ---
     * ##-
     * ##-
     * ```
     */
    south_west_cells:
        [[(C, SouthWestCellEdges); MAZE_HEGIHT - 1]; MAZE_WIDTH - 1],

    /**
     * All the cells that are on the east edge, but not the most northern
     * ```
     * ---
     * --#
     * --#
     * ```
     */
    east_cells: [(C, EastCellEdges); MAZE_WIDTH - 1],

    /**
     * All the cells that are on the north edge, but not the most eastern
     * ```
     * ##-
     * ---
     * ---
     * ```
     */
    north_cells: [(C, NorthCellEdges); MAZE_WIDTH - 1],

    /**
     * The most north eastern cell
     * ```
     * --#
     * ---
     * ---
     */
    north_east_cell: (C, NorthEastCellEdges),
}

impl<C: Sized + Copy> Maze<C> {
    pub fn new(default_cell: C, default_edge: Edge) -> Maze<C> {
        Maze {
            south_west_cells: [[(
                default_cell,
                SouthWestCellEdges::from_edge(default_edge),
            ); MAZE_HEGIHT - 1]; MAZE_WIDTH - 1],
            north_cells: [(
                default_cell,
                NorthCellEdges::from_edge(default_edge),
            ); MAZE_WIDTH - 1],
            east_cells: [(default_cell, EastCellEdges::from_edge(default_edge));
                MAZE_WIDTH - 1],
            north_east_cell: (
                default_cell,
                NorthEastCellEdges::from_edge(default_edge),
            ),
        }
    }

    pub fn cell(
        &self,
        Location { x, y }: Location,
    ) -> Option<(C, FullCellEdges)> {
        let (cell, north_edge, east_edge) = match (x, y) {
            // ---
            // ##-
            // ##-
            (x, y) if x < MAZE_WIDTH - 1 && y < MAZE_HEGIHT - 1 => Some((
                self.south_west_cells[x][y].0,
                self.south_west_cells[x][y].1.north_edge,
                self.south_west_cells[x][y].1.east_edge,
            )),

            // ---
            // --#
            // --#
            (x, y) if x == MAZE_WIDTH - 1 && y < MAZE_HEGIHT - 1 => Some((
                self.east_cells[y].0,
                self.east_cells[y].1.north_edge,
                Edge::Closed,
            )),

            // ##-
            // ---
            // ---
            (x, y) if x < MAZE_WIDTH - 1 && y == MAZE_HEGIHT - 1 => Some((
                self.north_cells[x].0,
                Edge::Closed,
                self.north_cells[x].1.east_edge,
            )),

            // --#
            // ---
            // ---
            (x, y) if x == MAZE_WIDTH - 1 && y == MAZE_HEGIHT - 1 => {
                Some((self.north_east_cell.0, Edge::Closed, Edge::Closed))
            }

            (_, _) => {
                dbg!("Failed to get north or east edge!");
                None
            }
        }?;

        let west_edge = match (x, y) {
            // ---
            // -##
            // -##
            (x, y) if x > 0 && x < MAZE_WIDTH && y < MAZE_HEGIHT - 1 => {
                Some(self.south_west_cells[x - 1][y].1.east_edge)
            }

            // -##
            // ---
            // ---
            (x, y) if x > 0 && x < MAZE_WIDTH && y == MAZE_HEGIHT - 1 => {
                Some(self.north_cells[x - 1].1.east_edge)
            }

            // #--
            // #--
            // #--
            (x, _y) if x == 0 => Some(Edge::Closed),

            (_, _) => {
                dbg!("Failed to get west edge!");
                None
            }
        }?;

        let south_edge = match (x, y) {
            // ##-
            // ##-
            // ---
            (x, y) if x < MAZE_WIDTH - 1 && y > 0 && y < MAZE_HEGIHT => {
                Some(self.south_west_cells[x][y - 1].1.north_edge)
            }

            // --#
            // --#
            // ---
            (x, y) if x == MAZE_WIDTH - 1 && y > 0 && y < MAZE_HEGIHT => {
                Some(self.east_cells[y - 1].1.north_edge)
            }

            // ---
            // ---
            // ###
            (_x, y) if y == 0 => Some(Edge::Closed),

            (_, _) => {
                dbg!("Failed to get south edge!");
                None
            }
        }?;

        Some((
            cell,
            FullCellEdges {
                north_edge,
                east_edge,
                south_edge,
                west_edge,
            },
        ))
    }

    fn update_cell(
        &mut self,
        Location { x, y }: Location,
        (
            cell_data,
            PartialCellEdges {
                north_edge,
                east_edge,
                south_edge,
                west_edge,
            },
        ): (Option<C>, PartialCellEdges),
    ) {
        match (x, y) {
            // ---
            // ##-
            // ##-
            (x, y) if x < MAZE_WIDTH - 1 && y < MAZE_HEGIHT - 1 => {
                if let Some(cell_data) = cell_data {
                    self.south_west_cells[x][y].0 = cell_data;
                }
                if let Some(north_edge) = north_edge {
                    self.south_west_cells[x][y].1.north_edge = north_edge;
                }
                if let Some(east_edge) = east_edge {
                    self.south_west_cells[x][y].1.north_edge = east_edge;
                }
            }

            // ---
            // --#
            // --#
            (x, y) if x == MAZE_WIDTH - 1 && y < MAZE_HEGIHT - 1 => {
                if let Some(cell_data) = cell_data {
                    self.east_cells[y].0 = cell_data;
                }
                if let Some(north_edge) = north_edge {
                    self.east_cells[y].1.north_edge = north_edge;
                }
            }

            // ##-
            // ---
            // ---
            (x, y) if x < MAZE_WIDTH - 1 && y == MAZE_HEGIHT - 1 => {
                if let Some(cell_data) = cell_data {
                    self.north_cells[x].0 = cell_data;
                }
                if let Some(east_edge) = east_edge {
                    self.north_cells[x].1.east_edge = east_edge;
                }
            }

            // --#
            // ---
            // ---
            (x, y) if x == MAZE_WIDTH - 1 && y == MAZE_HEGIHT - 1 => {
                if let Some(cell_data) = cell_data {
                    self.north_east_cell.0 = cell_data;
                }
            }

            (_, _) => {
                dbg!("Failed to set north or east edge!");
            }
        };

        let west_edge = match (x, y) {
            // ---
            // -##
            // -##
            (x, y) if x > 0 && x < MAZE_WIDTH && y < MAZE_HEGIHT - 1 => {
                Some(self.south_west_cells[x - 1][y].1.east_edge)
            }

            // -##
            // ---
            // ---
            (x, y) if x > 0 && x < MAZE_WIDTH && y == MAZE_HEGIHT - 1 => {
                Some(self.north_cells[x - 1].1.east_edge)
            }

            // #--
            // #--
            // #--
            (x, _y) if x == 0 => Some(Edge::Closed),

            (_, _) => {
                dbg!("Failed to get west edge!");
                None
            }
        };

        let south_edge = match (x, y) {
            // ##-
            // ##-
            // ---
            (x, y) if x < MAZE_WIDTH - 1 && y > 0 && y < MAZE_HEGIHT => {
                Some(self.south_west_cells[x][y - 1].1.north_edge)
            }

            // --#
            // --#
            // ---
            (x, y) if x == MAZE_WIDTH - 1 && y > 0 && y < MAZE_HEGIHT => {
                Some(self.east_cells[y - 1].1.north_edge)
            }

            // ---
            // ---
            // ###
            (_x, y) if y == 0 => Some(Edge::Closed),

            (_, _) => {
                dbg!("Failed to get south edge!");
                None
            }
        };
    }
}
