#[derive(Debug,Eq,PartialEq)]
pub enum Command {
    Publish(String),
    Retrieve,
}

#[derive(Debug,Eq,PartialEq)]
pub enum Error {
    IllegalNewline,
    MissingNewline,
    TooFewArguments,
    TooManyArguments,
    UnknownCommand(String),
    UnknownError,
}

pub fn parse(input: &str) -> Result<Command, Error> {
    match input.find('\n') {
        Some(index) => {
            if index != input.len() - 1 {
                return Err(Error::IllegalNewline);
            }
        },
        None => {
            return Err(Error::MissingNewline);
        }
    }

    let cmd: Vec<&str> = input.trim().splitn(2, ' ').collect();

    match cmd.get(0) {
        Some(&"RETRIEVE") => {
            match cmd.get(1) {
                None => Ok(Command::Retrieve),
                Some(_) => Err(Error::TooManyArguments),
            }
        },
        Some(&"PUBLISH") => {
            match cmd.get(1) {
                Some(val) => Ok(Command::Publish(val.trim().to_string())),
                None => Err(Error::TooFewArguments),
            }
        },
        Some(unknown) => Err(Error::UnknownCommand(unknown.trim().to_string())),
        None => Err(Error::UnknownError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_retrieve() {
        assert_eq!(parse("RETRIEVE\n"), Ok(Command::Retrieve));

        assert_eq!(parse("RETRIEVE foo\n"), Err(Error::TooManyArguments));
        assert_eq!(parse("RETRIEVE\nfoo\n"), Err(Error::IllegalNewline));
        assert_eq!(parse("RETRIEVE\nfoo"), Err(Error::IllegalNewline));
        assert_eq!(parse("RETRIEVE"), Err(Error::MissingNewline));
    }

    #[test]
    fn it_parses_publish() {
        assert_eq!(parse("PUBLISH foo\n"), Ok(Command::Publish(String::from("foo"))));
        assert_eq!(parse("PUBLISH foo bar\n"), Ok(Command::Publish(String::from("foo bar"))));

        assert_eq!(parse("PUBLISH\n"), Err(Error::TooFewArguments));
        assert_eq!(parse("PUBLISH foo\nbar\n"), Err(Error::IllegalNewline));
        assert_eq!(parse("PUBLISH foo\nbar"), Err(Error::IllegalNewline));
        assert_eq!(parse("PUBLISH foo"), Err(Error::MissingNewline));
    }

    #[test]
    fn it_rejects_garbage() {
        assert_eq!(parse("GARBAGE\n"), Err(Error::UnknownCommand(String::from("GARBAGE"))));
        assert_eq!(parse("GARBAGE arg\n"), Err(Error::UnknownCommand(String::from("GARBAGE"))));
        assert_eq!(parse("\n"), Err(Error::UnknownCommand(String::from(""))));
    }
}
