use log::{self, LevelFilter};
use simple_logger::SimpleLogger;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path,
};

pub fn init_logs() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .env()
        .init()
        .unwrap();
}

pub mod prelude {
    // pub use itertools::Itertools;
    // pub use num::Integer;
    // pub use rayon::prelude::*;
    // pub use regex::Regex;
}
