//! Library code for Part 2 of Day04 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`

use tracing::{self as tea, Level, instrument};

#[expect(unused)]
use crate::{Result, parse::parse_input_2};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part2(input: &str) -> Result<u64> {
        tea::trace!(%input);
        let example_count = parse_input_2(input, 10, None)?;
        println!("Example count: {}", example_count);
        let final_count = parse_input_2(input, 140, None)?;
        println!("Final Count: {}", final_count);
        todo!();
}

// #[cfg(test)]
// mod tests {
//         use indoc::indoc;
//         use quickcheck::TestResult;
//         use quickcheck_macros::quickcheck;
//         use rand::Rng;
//         use test_log::test;
//         use tracing::{self as tea, instrument};

//         use super::*;
//         use crate::{EXAMPLE_INPUT, FINAL_INPUT};

//         #[test]
//         #[instrument]
//         fn test_process_example() -> Result<()> {
//                 let input = EXAMPLE_INPUT;
//                 let expected = todo!();
//                 assert_eq!(process_part2(input)?, expected);
//                 Ok(())
//         }

//         // /// Test's expected value to be populated after solution verification.
//         // /// NOTE: `#[ignore]` is set for this test by default.
//         // #[ignore]
//         // #[test]
//         // fn test_process_problem_input() -> Result<()> {
//         //         tracing_subscriber::fmt::init();
//         //         let input = FINAL_INPUT;
//         //         let expected = todo!();
//         //         assert_eq!(process_part2(input)?, expected);
//         //         Ok(())
//         // }
// }
