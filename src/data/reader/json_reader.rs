use std::fs::File;
use std::io::{BufRead, BufReader};

use serde::de::DeserializeOwned;

use crate::data::reader::record_reader::RecordReader;
use crate::data::record::Record;
use crate::data::record_io_error::RecordIOError;

//Implementation of RecordReader. The JSONReader reads files, where every line is a valid JSON-Object
pub struct JSONReader;

impl JSONReader {
    fn read_full<T: Record + DeserializeOwned>(&self, reader: BufReader<&File>) -> Result<Vec<T>, RecordIOError> {
        let mut reviews = Vec::new();

        for line in reader.lines() {
            let json = line?;
            let review = serde_json::from_str(json.as_str())?;

            reviews.push(review);
        }

        Ok(reviews)
    }

    fn read_limited<T: Record + DeserializeOwned>(&self, reader: BufReader<&File>, mut limit: usize) -> Result<Vec<T>, RecordIOError> {
        let mut reviews = Vec::new();

        for line in reader.lines().take(limit) {
            let json = line?;
            let review = serde_json::from_str(json.as_str())?;

            reviews.push(review);

            limit -= 1;
        }

        Ok(reviews)
    }
}

impl<T: Record + DeserializeOwned> RecordReader<T> for JSONReader {
    fn read(&self, path: &str, limit: Option<usize>) -> Result<Vec<T>, RecordIOError> {
        let file = &File::open(path)?;
        let reader = BufReader::new(file);

        match limit {
            Some(n) => self.read_limited(reader, n),
            None => self.read_full(reader)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data::reader::json_reader::JSONReader;
    use crate::data::reader::record_reader::RecordReader;
    use crate::data::review::Review;

    const TEST_FILE: &'static str = "tests/data_sets/read_test.json";
    const RECORDS: usize = 10;
    const LIMIT: usize = 5;

    #[test]
    fn success_limited() {
        let reader = JSONReader;
        let limit = Some(LIMIT);

        let result: Vec<Review> = reader.read(TEST_FILE, limit).unwrap();

        assert_eq!(result.len(), LIMIT)
    }

    #[test]
    fn success_unlimited() {
        let reader = JSONReader;

        let result: Vec<Review> = reader.read(TEST_FILE, None).unwrap();

        assert_eq!(result.len(), RECORDS)
    }
}