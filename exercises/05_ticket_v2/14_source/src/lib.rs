use crate::status::{ParseStatusError, Status};
use std::{error::Error, fmt::{self, Display, Formatter}};

// We've seen how to declare modules in one of the earliest exercises, but
// we haven't seen how to extract them into separate files.
// Let's fix that now!
//
// In the simplest case, when the extracted module is a single file, it is enough to
// create a new file with the same name as the module and move the module content there.
// The module file should be placed in the same directory as the file that declares the module.
// In this case, `src/lib.rs`, thus `status.rs` should be placed in the `src` directory.
mod status;

// TODO: Add a new error variant to `TicketNewError` for when the status string is invalid.
//   When calling `source` on an error of that variant, it should return a `ParseStatusError` rather than `None`.

#[derive(Debug)]
pub enum TicketNewError {
    TitleCannotBeEmpty,
    TitleTooLong,
    DescriptionCannotBeEmpty,
    DescriptionTooLong,
    InvalidStatus(ParseStatusError),
}

impl Display for TicketNewError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TicketNewError::TitleCannotBeEmpty => write!(f, "{}", "Title cannot be empty"),
            TicketNewError::TitleTooLong => write!(f, "{}", "Title cannot be longer than 50 bytes"),
            TicketNewError::DescriptionCannotBeEmpty => write!(f, "{}", "Description cannot be empty"),
            TicketNewError::DescriptionTooLong => write!(f, "{}", "Description cannot be longer than 500 bytes"),
            TicketNewError::InvalidStatus(parse_status_error) => {
                write!(f, "{}", parse_status_error)
            }
        }
    }
}

impl Error for TicketNewError  {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TicketNewError::InvalidStatus(e) => Some(e),
            _ => None
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        let status = match status.try_into() {
            Ok(s) => s,
            Err(error) => return Err(TicketNewError::InvalidStatus(error)),
        };

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
