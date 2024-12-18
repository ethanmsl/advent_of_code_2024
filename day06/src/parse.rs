//! Raw-input parsing code for Day06 of Advent of Code 2024.

use std::io;

use tracing::{Level, debug, instrument};

use crate::Result;

/// Parse txt input ...
#[instrument(skip_all, ret(level = Level::TRACE))]
pub fn parse_input(raw_input: &str) -> Result<()> {
        println!("\n{}\n", raw_input);
        dirty_pause()?;
        todo!()
}

/// Quick and dirty pause button so I can watch as program runs.
#[instrument]
fn dirty_pause() -> Result<()> {
        println!("Press Enter to continue...");
        let mut _input = String::new();
        let read_in = io::stdin().read_line(&mut _input)?;
        debug!(?read_in);
        Ok(())
}

mod objects {
        use tracing::instrument;

        use crate::support::{ErrWrapperDay06, error::ErrKindDay06};

        /// State of Position
        /// ('guard is not part state')
        #[derive(Clone, Copy, PartialEq, Eq, Debug, derive_more::Display)]
        enum PositionState {
                #[display("#")]
                Obstacle,
                #[display(".")]
                Empty,
        }
        impl TryFrom<char> for PositionState {
                type Error = ErrWrapperDay06;

                #[instrument(skip_all)]
                fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
                        match c {
                                '#' => Ok(PositionState::Obstacle),
                                '.' => Ok(PositionState::Empty),
                                '\n' => Err(ErrKindDay06::ParseNewline {
                                        source_string: c.to_string(),
                                })?,
                                _ => Err(ErrKindDay06::ParseOther {
                                        source_string: c.to_string(),
                                })?,
                        }
                }
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
        impl TryFrom<char> for Direction {
                type Error = ErrWrapperDay06;

                #[instrument(skip_all)]
                fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
                        match c {
                                '^' => Ok(Direction::Up),
                                'v' => Ok(Direction::Down),
                                '<' => Ok(Direction::Left),
                                '>' => Ok(Direction::Right),
                                '\n' => Err(ErrKindDay06::ParseNewline {
                                        source_string: c.to_string(),
                                })?,
                                _ => Err(ErrKindDay06::ParseOther {
                                        source_string: c.to_string(),
                                })?,
                        }
                }
        }
}

#[cfg(test)]
mod tests {
        use indoc::indoc;
        #[expect(unused)]
        use quickcheck::TestResult;
        #[expect(unused)]
        use quickcheck_macros::quickcheck;
        use rand::Rng;
        use test_log::test;
        use tracing::{instrument, trace};

        use super::*;

        #[test]
        #[instrument]
        fn example_test() -> Result<()> {
                let input = indoc!("
                        ....#.....
                        .........#
                        ..........
                        ..#.......
                        .......#..
                        ..........
                        .#..^.....
                        ........#.
                        #.........
                        ......#...");
                let expected_ys = 10;
                let expected_xs = 10;
                let vv: Vec<&str> = input.lines().collect();
                let found_ys = vv.len();
                let found_xs = vv[0].len();
                assert_eq!(found_ys, expected_ys);
                assert_eq!(found_xs, expected_xs);
                Ok(())
        }

        /// Generates a square maze as a string of '.' and '#' characters.
        ///
        /// Returns a string representing a maze with:
        /// - Random size between 1x1 and 300x300
        /// - Random obstacles ('#') with 1-30% probability
        /// - Empty spaces ('.') for remaining cells
        /// - Newline character after each row
        #[instrument]
        fn input_maze_generator() -> String {
                let mut rng = rand::thread_rng();
                let side_len = rng.gen_range(1..=300);
                trace!(side_len);
                let chance_of_obstacle: f64 = rng.gen_range(0.01..=0.3);
                let is_obstacle_gen = std::iter::from_fn(move || Some(rng.gen_bool(chance_of_obstacle)));
                let row_iter = is_obstacle_gen
                        .take(side_len)
                        .map(|is_obstacle| if is_obstacle { '#' } else { '.' })
                        .chain(std::iter::once('\n'));
                let maze_string: String = row_iter.cycle().take(side_len * (side_len + 1)).collect();
                maze_string
        }

        // #[quickcheck]
        // #[instrument]
        // fn qc_example_quickcheck(sum: u16, step_range_inclusive: (u8, u8)) -> TestResult {
        //         let maze_input =
        //         let Some(vector) = example_iw_generator(sum, step_range_inclusive) else {
        //                 return TestResult::discard();
        //         };
        //         let vector_sum: i64 = vector.iter().sum();
        //         TestResult::from_bool(sum as i64 == vector_sum)
        // }
}
