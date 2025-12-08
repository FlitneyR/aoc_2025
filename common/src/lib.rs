use std::env::args;
use std::fs::File;
use std::io::{Read, stdin};

#[derive(Clone, Copy, Debug)]
pub enum FromRegexCapturesError {
    MissingField(&'static str),
    FailedToParse(&'static str),
    RegexDidntMatch,
}

#[allow(unused)]
pub trait FromRegexCaptures
    where Self: Sized
{
    fn from_regex_captures(captures: &regex::Captures) -> Result<Self, FromRegexCapturesError>;
}

pub struct ParseByRegexIter<'a, T: FromRegexCaptures> {
    str: &'a str,
    regex: &'a regex::Regex,
    _phantom_t: std::marker::PhantomData<T>
}

pub trait ParseByRegex {
    fn iter_by_regex<'a, T: FromRegexCaptures>(&'a self, regex: &'a regex::Regex) -> ParseByRegexIter<'a, T>;
}

impl ParseByRegex for &str {
    fn iter_by_regex<'a, T: FromRegexCaptures>(&'a self, regex: &'a regex::Regex) -> ParseByRegexIter<'a, T> {
        ParseByRegexIter { str: self, regex, _phantom_t: Default::default() }
    }
}

impl<'a, T> Iterator for ParseByRegexIter<'a, T>
    where T: FromRegexCaptures
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let captures = self.regex.captures(self.str)?;
        let t = T::from_regex_captures(&captures).ok()?;
        self.str = &self.str[captures.get_match().end()..];
        Some(t)
    }
}

#[derive(Debug)]
pub enum GetInputError {
    FileDoesNotExist { path: String },
    FailedToReadFile,
}

pub enum Input {
    Stdin,
    File {
        str: String,
        lines_read: usize,
    }
}

pub fn get_input() -> Result<Input, GetInputError> {
    if let Some(path) = args().skip(1).next() {
        let mut buffer = String::new();
        let mut file = File::open(&path).map_err(|_| GetInputError::FileDoesNotExist { path })?;
        file.read_to_string(&mut buffer).map_err(|_| GetInputError::FailedToReadFile)?;
        Ok(Input::File{ str: buffer, lines_read: 0 })
    } else {
        Ok(Input::Stdin)
    }
}

impl Input {
    pub fn lines<'a>(&'a mut self) -> InputLines<'a> {
        match self {
            Input::Stdin => InputLines::Stdin(stdin().lines()),
            Input::File{ str, lines_read } => InputLines::File {
                iter: str.lines().skip(*lines_read),
                lines_read,
            },
        }
    }
    
    pub fn collect_to_string(self) -> String {
        match self {
            Input::Stdin => {
                let mut buffer = String::new();
                stdin().read_to_string(&mut buffer).expect("File io error while reading input");
                buffer
            },
            Input::File { str, lines_read } => {
                str.lines().skip(lines_read)
                    .map(|line| format!("{line}\n"))
                    .collect()
            },
        }
    }
}

pub enum InputLines<'a> {
    Stdin(std::io::Lines<std::io::StdinLock<'static>>),
    File {
        iter: std::iter::Skip<std::str::Lines<'a>>,
        lines_read: &'a mut usize,
    },
}

impl<'a> Iterator for InputLines<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            InputLines::Stdin(lines) => lines.next().into_iter().flatten().next(),
            InputLines::File { iter, lines_read } => {
                **lines_read += 1;
                iter.next().map(|s| s.into())
            }
        }
    }
}
