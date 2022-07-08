use ariadne::{Report, ReportKind, Label, Source};

pub enum Error {
    UnrecognizedToken,
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Error::UnrecognizedToken => String::from("Unrecognized Token"),
        }
    }

    pub fn report_error(&self, line: usize, character: usize) {
        Report::build(ReportKind::Error, (), 34)
            .with_message(self.to_string())
            .finish();
    }
}