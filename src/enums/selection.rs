use core::fmt;
use std::{fmt::Display, str::FromStr};

#[derive(Copy, Clone, Debug)]
pub enum Selection {
    ToReview,
    UnderReview,
    Draft,
}
impl Display for Selection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Selection::ToReview => write!(f, "ToReview"),
            Selection::UnderReview => write!(f, "UnderReview"),
            Selection::Draft => write!(f, "Draft"),
        }
    }
}
impl FromStr for Selection {
    type Err = ();
    fn from_str(input: &str) -> Result<Selection, Self::Err> {
        match input {
            "ToReview" => Ok(Selection::ToReview),
            "UnderReview" => Ok(Selection::UnderReview),
            "Draft" => Ok(Selection::Draft),
            _ => Err(()),
        }
    }
}