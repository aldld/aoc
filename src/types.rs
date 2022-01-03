use std::{io, error::Error};

pub type Input = Box<dyn Iterator<Item=Result<String, io::Error>>>;
pub type UnitResult = Result<(), Box<dyn Error>>;
