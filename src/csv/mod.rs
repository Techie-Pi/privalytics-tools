use std::error::Error;
use crate::{Entry, Transformation};
use crate::structures::entry::DataValues;

pub struct CsvTransformation;

impl Transformation<Box<dyn Error>> for CsvTransformation {
    fn transform(input: Vec<Entry>) -> Result<(Vec<u8>, String), Box<dyn Error>> {
        let mut keys = vec!["duid".to_string(), "date".to_string(), "page".to_string(), "uid".to_string()];
        for entry in &input {
            for key in entry.data.keys() {
                if !keys.contains(key) {
                    keys.push(key.clone());
                }
            }
        }
        let keys = keys;

        let mut writer = csv::Writer::from_writer(vec![]);
        writer.write_record(keys.as_slice())?;

        for entry in &input {
            let mut record = vec!["".to_string(); keys.len()];
            *record.get_mut(0).unwrap() = entry.metadata.duid.to_string();
            *record.get_mut(1).unwrap() = entry.metadata.date.to_string();
            *record.get_mut(2).unwrap() = entry.metadata.page.as_ref().unwrap_or(&"".to_string()).to_string();
            *record.get_mut(3).unwrap() = entry.metadata.uid.as_ref().unwrap_or(&"".to_string()).to_string();

            for (key, value) in &entry.data {
                for (i, entry_key) in keys.iter().enumerate() {
                    if key == entry_key {
                        *record.get_mut(i).unwrap() = match value.clone() {
                            DataValues::String(v) => v,
                            DataValues::Number(n) => n.to_string(),
                            DataValues::Bool(b) => b.to_string(),
                        }
                    }
                }
            }
            writer.write_record(record.as_slice())?;
        }

        Ok((writer.into_inner()?, "csv".to_string()))
    }
}
