use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use bevy::prelude::*;
use polars::prelude::*;
use crate::files_manager::files_porperties::FileProperties;

#[derive(Component, Clone)]
pub struct CsvFile{
    pub path: String,
    pub header: bool,
    pub sep: u8
}

impl FileProperties for CsvFile {
    fn path(&self) -> String {
        self.path.clone()
    }
}

impl CsvFile {

    pub fn get_file(&self) -> Result<File, Box<dyn Error>>{
        let file = File::open(self.path.clone())?;
        Ok(file)
    }

    pub fn read_csv_file(&self) -> Result<impl Iterator<Item = String>, Box<dyn Error>> {
        let file = File::open(self.path.clone())?;
        let reader = BufReader::new(file);
        let lines = reader.lines().map(|line| line.unwrap());
        Ok(lines)
    }


    pub fn dataframe(&self) -> PolarsResult<DataFrame> {
        let file = File::open(self.path.clone())?;

        let mut df = CsvReader::new(file)
            .has_header(self.header)
            .finish()?;

        if self.header {
            // Convierte los encabezados a minúsculas
            let lowercase_columns: Vec<String> = df
                .get_column_names()
                .iter()
                .map(|col_name| col_name.to_lowercase())
                .collect();

            df.set_column_names(&lowercase_columns)?;
        }

        Ok(df)
    }

}