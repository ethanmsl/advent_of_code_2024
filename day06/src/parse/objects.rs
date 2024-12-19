//! Objects for Parsing into
//!
//! Need:
//! - Maze
//!     - PositionState
//! - Guard
//!     - Direction
//!     - Point2D
//!
//!

use derive_more::derive::{Constructor, From, Index, Into};
use itertools::Itertools as _;
use tracing::{Level, event, instrument};

use crate::{Result,
            support::{ErrWrapperDay06, error::ErrKindDay06}};

#[derive(Index, Debug, Clone, From, Into)]
pub struct Maze {
        #[index]
        pub positions: Vec<PositionState>,
        pub max_dims:  Point2D,
}
impl std::fmt::Display for Maze {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for (r, c) in (0..self.max_dims.y).cartesian_product(0..self.max_dims.x) {
                        if c == 0 {
                                writeln!(f)?
                        }
                        write!(f, "{}", self.positions[r * self.max_dims.x + c])?;
                }
                Ok(())
        }
}
impl Maze {
        #[instrument(skip_all, err, ret(level = Level::TRACE))]
        fn new(positions: Vec<PositionState>, max_dims: Point2D) -> Result<Self> {
                if positions.len() != max_dims.x * max_dims.y {
                        Err("Maze dimensions do not match the positions vector length.".to_string())?
                }
                Ok(Self { positions, max_dims })
        }

        #[instrument(skip_all)]
        pub fn from_input_string(input: &str) -> Result<(Self, Option<Guard>)> {
                let mut guard: Option<Guard> = None;
                let mut positions: Vec<PositionState> = Vec::new();
                for (y, line) in input.lines().enumerate() {
                        for (x, c) in line.chars().enumerate() {
                                let ps: PositionState = match c.try_into() {
                                        Ok(ps) => ps,
                                        Err(ErrWrapperDay06 {
                                                source: ErrKindDay06::ParseUnexpectedDirection { source_char },
                                                ..
                                        }) => {
                                                assert!(guard.is_none()); // designed for a single guard per maze
                                                guard = Some(Guard {
                                                        pos: Point2D::from((x, y)),
                                                        dir: source_char.try_into().expect("expected direction char"),
                                                });
                                                PositionState::Empty
                                        }
                                        Err(e) => Err(e)?,
                                };
                                positions.push(ps);
                        }
                }
                let max_dims = Point2D {
                        x: input.lines().next().expect("expected at least one line in input").len(),
                        y: input.lines().count(),
                };
                debug_assert_eq!(max_dims.x, max_dims.y);
                let maze = Self::new(positions, max_dims)?;
                Ok((maze, guard))
        }

        #[instrument(skip(self),ret(level = Level::DEBUG))]
        pub fn get(&self, point: Point2D) -> Option<PositionState> {
                self.pt_to_ln_index(point).map(|index| self.positions[index])
        }

        #[instrument]
        fn pt_to_ln_index(&self, point: Point2D) -> Option<usize> {
                if point.x >= self.max_dims.x || point.y >= self.max_dims.y {
                        event![Level::DEBUG, %point, ?self.max_dims, "point out of bounds"];
                        None
                } else {
                        Some(point.y * self.max_dims.x + point.x)
                }
        }
}

/// State of Position
/// ('guard is not part state')
#[derive(Clone, Copy, PartialEq, Eq, Debug, derive_more::Display)]
pub enum PositionState {
        #[display("#")]
        Obstacle,
        #[display(".")]
        Empty,
}

#[derive(Clone, Copy, Debug, derive_more::Display, PartialEq, Eq, Constructor)]
#[display("G_'{}'@{}", dir, pos)]
pub struct Guard {
        pos: Point2D,
        dir: Direction,
}

#[derive(Clone, Copy, From, Into, PartialEq, Eq, PartialOrd, Debug, derive_more::Display, Constructor)]
#[display("({},{})", x, y)]
#[from(forward)]
pub struct Point2D {
        pub x: usize,
        pub y: usize,
}

/// Direction of Facing.
#[derive(Copy, Clone, PartialEq, Eq, Debug, derive_more::Display)]
pub enum Direction {
        #[display("^")]
        Up,
        #[display("v")]
        Down,
        #[display("<")]
        Left,
        #[display(">")]
        Right,
}

// impls
impl TryFrom<char> for PositionState {
        type Error = ErrWrapperDay06;

        #[instrument(skip_all)]
        fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
                match c {
                        '#' => Ok(PositionState::Obstacle),
                        '.' => Ok(PositionState::Empty),
                        '\n' => Err(ErrKindDay06::ParseNewline { source_char: c })?,
                        '^' | 'v' | '<' | '>' => Err(ErrKindDay06::ParseUnexpectedDirection { source_char: c })?,
                        _ => Err(ErrKindDay06::ParseOther { source_char: c })?,
                }
        }
}
impl TryFrom<char> for Direction {
        type Error = ErrWrapperDay06;

        #[instrument(skip_all)]
        fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
                match c {
                        '^' => Ok(Direction::Up),
                        'v' => Ok(Direction::Down),
                        '<' => Ok(Direction::Left),
                        '>' => Ok(Direction::Right),
                        '\n' => Err(ErrKindDay06::ParseNewline { source_char: c })?,
                        '#' | '.' => Err(ErrKindDay06::ParseUnexpectedPositionState { source_char: c })?,
                        _ => Err(ErrKindDay06::ParseOther { source_char: c })?,
                }
        }
}

#[cfg(test)]
mod test {

        use super::*;
        #[test]
        fn test_maze_from_input_string() {
                let input = "##\n.#\n";
                let maze = Maze::from_input_string(input).unwrap();
                assert_eq!(maze.max_dims, Point2D { x: 2, y: 2 });
                assert_eq!(maze.get(Point2D { x: 0, y: 0 }), Some(PositionState::Obstacle));
                assert_eq!(maze.get(Point2D { x: 1, y: 0 }), Some(PositionState::Obstacle));
                assert_eq!(maze.get(Point2D { x: 0, y: 1 }), Some(PositionState::Empty));
                assert_eq!(maze.get(Point2D { x: 1, y: 1 }), Some(PositionState::Obstacle));
        }

        #[test]
        fn test_maze_from_input_string_err() {
                let input = "##\n.#\n\n";
                let maze = Maze::from_input_string(input);
                assert!(maze.is_err());
        }

        #[test]
        fn test_position_state_try_from() {
                assert_eq!(PositionState::try_from('#').unwrap(), PositionState::Obstacle);
                assert_eq!(PositionState::try_from('.').unwrap(), PositionState::Empty);
                assert!(PositionState::try_from('x').is_err());
                assert!(PositionState::try_from('\n').is_err());
                assert!(PositionState::try_from('^').is_err());
        }

        #[test]
        fn test_direction_try_from() {
                assert_eq!(Direction::try_from('^').unwrap(), Direction::Up);
                assert_eq!(Direction::try_from('v').unwrap(), Direction::Down);
                assert_eq!(Direction::try_from('<').unwrap(), Direction::Left);
                assert_eq!(Direction::try_from('>').unwrap(), Direction::Right);
                assert!(Direction::try_from('x').is_err());
                assert!(Direction::try_from('\n').is_err());
                assert!(Direction::try_from('.').is_err());
        }

        #[test]
        fn test_point2d_try_from() {
                assert_eq!(Point2D::from((1_usize, 2_usize)), Point2D { x: 1, y: 2 });
        }

        // #[test]
        // fn test_maze_string_display() {
        //         let input = "##\n.#\n";
        //         let maze = Maze::from_input_string(input).unwrap();
        //         assert_eq!(format!("{}", maze), "##\n.#\n");
        // }
}
