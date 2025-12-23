//! Unit tests for the M49 CSV data

use crate::m49::Record;
use csv::ReaderBuilder;

/// The M49 dataset as of the second update from 2021, retrieved Sep. 5, 2025.
const M49_2021_2: &str = include_str!("2025-09-05.csv");

#[test]
fn count_2021_2() {
    const EXPECTED: usize = 248;

    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(M49_2021_2.as_bytes());
    let mut count = 0usize;

    for result in reader.deserialize::<Record>() {
        let record = result.expect("Could not parse record");
        eprintln!("{record:?}");
        count += 1;
    }

    assert_eq!(EXPECTED, count);
}
