use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpordParserError {
    #[error("Could not determine LLAB audience from name '{0}'.")]
    InvalidLLABAudience(String),

    #[error("Could not determine PT day from the name `{0}`")]
    InvalidPTDay(String),

    #[error("Error parsing the location `{0}`. Expected the the location to start with 'Main Location'. ")]
    InvalidLocationFormat(String),

    #[error("Mission name started with `Week` but did not contain a `/`")]
    WeekMisionNameParseFail(String),

    #[error("Encountered EOF mid parse. Is the file corrupt?")]
    IncompleteParse,

    #[error("Could not parse the week.")]
    InvalidWeekFormat,

    #[error("Could not parse the week number")]
    InvalidWeekNumber(#[from] std::num::ParseIntError),

    #[error("Error opening the OPORD.")]
    FileError(#[from] std::io::Error),
}
