use crate::data::generation::record_generator::RecordGenerator;
use std::fs::File;
use crate::data::record::Record;

/// Writes records to a given file, generated by the given RecordGenerator
pub trait RecordWriter<T: Record> {
    fn write(&self, path: &String, generator: &mut dyn RecordGenerator<T>, limit: usize) -> Result<(), &str>;
}