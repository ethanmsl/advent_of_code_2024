//! Library code for Part 2 of Day02 of Advent of Code 2024.
//! `bin > part2_bin.rs` will run this code along with content of `input2.txt`

use tracing as tea;
use tracing::{Level, instrument};

use crate::{Result,
            parse::{ReportStatus, parse_input}};

#[instrument(skip_all, ret(level = Level::DEBUG))]
pub fn process_part2(input: &str) -> Result<u64> {
        tea::trace!(%input);
        let mut statuses = Vec::new();
        let lines = parse_input(input)?;
        for line in lines {
                let wins = line.windows(2);
                let diffs: Vec<i64> = wins.map(|x| x[0] - x[1]).collect();
                tracing::info!(?diffs);
                statuses.push(is_safe_2(diffs, None));
        }
        tracing::info!(?statuses);
        let sum_safes = statuses
                .iter()
                .filter(|x| **x == ReportStatus::Safe)
                .count()
                .try_into()?;
        Ok(sum_safes)
}

fn is_safe_2(diffs: Vec<i64>, has_skipped: Option<bool>) -> ReportStatus {
        // WARN: assuming no empty diffs
        let mut has_skipped = has_skipped.unwrap_or(false);

        let first_elem = diffs[0];
        'level: for diff in diffs.clone() {
                tracing::debug!(?first_elem, ?diff);
                let is_out_of_magnitude = !(1..=3).contains(&diff.abs());
                let is_sign_change = (first_elem.is_positive() && diff.is_negative())
                        || (first_elem.is_negative() && diff.is_positive());
                tracing::debug!(is_out_of_magnitude, is_sign_change);
                if is_out_of_magnitude || is_sign_change {
                        if !has_skipped {
                                tracing::warn!(has_skipped, "forking to create skip");
                                has_skipped = true;
                                let diffs_variant1 = diffs.clone();
                                let diffs_variant2 = diffs.clone();
                                continue 'level;
                        }
                        return ReportStatus::Unsafe;
                }
        }
        tea::trace!("safe");
        ReportStatus::Safe
}

// #[cfg(test)]
// mod tests {
//         use indoc::indoc;

//         use super::*;

//         #[test]
//         fn test_process_example() -> Result<()> {
//                 let input = EXAMPLE_INPUT_2
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
//         //         let file_input = include_str!("../input1.txt");
//         //         let expected = todo!();
//         //         assert_eq!(process(file_input)?, expected);
//         //         Ok(())
//         // }
// }
