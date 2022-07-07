pub enum Error {
    UnrecognizedToken,
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Error::UnrecognizedToken => String::from("Unrecognized Token"),
        }
    }

    pub fn report_error(line: usize, ) {

    }
}