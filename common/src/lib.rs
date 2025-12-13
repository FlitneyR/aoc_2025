use std::collections::HashMap;
use std::env::args;
use std::io::{Read, Stdin, stdin};
use std::str::FromStr;

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

pub struct RegexStringIterator<'a, 'b, T: FromRegexCaptures> {
    str: &'a str,
    regex: &'b regex::Regex,
    _phantom_t: std::marker::PhantomData<T>,
}

pub trait IterByRegex<'a> {
    fn iter_by_regex<T: FromRegexCaptures>(self, regex: &'a regex::Regex) -> impl Iterator<Item = T>;
}

impl<'a, 'b> IterByRegex<'a> for &'b str {
    fn iter_by_regex<T: FromRegexCaptures>(self, regex: &'a regex::Regex) -> impl Iterator<Item = T> {
        RegexStringIterator { str: self, regex, _phantom_t: Default::default() }
    }
}

impl<'a, 'b> IterByRegex<'a> for &'b String {
    fn iter_by_regex<T: FromRegexCaptures>(self, regex: &'a regex::Regex) -> impl Iterator<Item = T> {
        RegexStringIterator { str: self.as_str(), regex, _phantom_t: Default::default() }
    }
}

impl<'a> IterByRegex<'a> for Stdin {
    fn iter_by_regex<T: FromRegexCaptures>(self, regex: &'a regex::Regex) -> impl Iterator<Item = T> {
        self.lines()
            .flatten()
            .map(|line| line.iter_by_regex(regex).collect::<Vec<T>>())
            .flatten()
    }
}

impl<'a, 'b> IterByRegex<'a> for &'b mut Input {
    fn iter_by_regex<T: FromRegexCaptures>(self, regex: &'a regex::Regex) -> impl Iterator<Item = T> {
        self.lines()
            .map(|line| line.iter_by_regex(regex).collect::<Vec<T>>())
            .flatten()
    }
}

impl<'a, 'b, T> Iterator for RegexStringIterator<'a, 'b, T>
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
    String {
        str: String,
        lines_read: usize,
    }
}

pub struct Arguments {
    pub named: HashMap<String, String>,
    pub unnamed: Vec<String>
}

static mut ARGUMENTS: Option<Arguments> = None;

impl Arguments {
    #[allow(static_mut_refs)]
    fn get() -> &'static Self {
        if unsafe { ARGUMENTS.is_none() } {
            let mut result = Self { named: HashMap::new(), unnamed: Vec::new() };
            let mut args = args().skip(1);
            let named_arg_regex = regex::Regex::new("(?<key>.*)=(?<value>.*)").unwrap();
            
            while let Some(arg) = args.next() {
                if let Some(named_arg) = named_arg_regex.captures(&arg) {
                    let key = named_arg.name("key").unwrap().as_str().to_string();
                    let value = named_arg.name("value").unwrap().as_str().to_string();
                    result.named.insert(key, value);
                } else {
                    result.unnamed.push(arg);
                }
            }
            
            unsafe { ARGUMENTS = Some(result); }
        }
        
        unsafe { ARGUMENTS.as_ref().unwrap() }
    }

    pub fn get_named<T: FromStr>(name: &str) -> Option<T> {
        Self::get().named.get(name)?.parse().ok()
    }
}

pub fn get_input() -> Result<Input, GetInputError> {
    // if let Some(path) = args().skip(1).next() {
    //     let mut buffer = String::new();
    //     let mut file = File::open(&path).map_err(|_| GetInputError::FileDoesNotExist { path })?;
    //     file.read_to_string(&mut buffer).map_err(|_| GetInputError::FailedToReadFile)?;
    //     Ok(Input::String{ str: buffer, lines_read: 0 })
    // } else {
    //     Ok(Input::Stdin)
    // }

    Ok(Input::Stdin)
}

impl Input {
    pub fn new(str: String) -> Self { Self::String { str, lines_read: 0 } }

    pub fn lines<'a>(&'a mut self) -> InputLines<'a> {
        match self {
            Input::Stdin => InputLines::Stdin(stdin().lines()),
            Input::String{ str, lines_read } => InputLines::File {
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
            Input::String { str, lines_read } => {
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

pub struct WrapAround<T: Clone, I: Iterator<Item = T>> {
    buffer: Vec<Option<T>>,
    index: isize,
    iter: I,
}

pub trait WrapAroundAble<T: Clone> where Self: Iterator<Item = T> + Sized {
    /// Creates an iterator that repeats the first `count` many items returned by this iterator
    fn wrap_around(self, count: usize) -> WrapAround<T, Self> {
        WrapAround { buffer: vec![None; count], index: 0, iter: self }
    }
}

impl<T: Clone, I: Iterator<Item = T>> WrapAroundAble<T> for I {}

impl<T: Clone, I: Iterator<Item = T>> Iterator for WrapAround<T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => {
                if self.index < self.buffer.len() as isize {
                    self.buffer[self.index as usize] = Some(item.clone());
                    self.index += 1;
                }
                Some(item)
            },
            None => {
                if self.index > 0 && self.index <= self.buffer.len() as isize {
                    let mut result = None;
                    let count = self.buffer.len();
                    std::mem::swap(&mut self.buffer[count - self.index as usize], &mut result);
                    self.index -= 1;
                    result
                } else {
                    None
                }
            },
        }
    }
}

pub struct WindowIter<T: Clone, I: Iterator<Item = T>> {
    buffer: Vec<T>,
    window_size: usize,
    iter: I,
}

impl<T: Clone, I: Iterator<Item = T>> Iterator for WindowIter<T, I> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.buffer.len() < self.window_size {
            self.buffer.push(self.iter.next()?);
        }

        let result = Some(self.buffer.clone());
        self.buffer.rotate_left(1);
        self.buffer.pop();
        result
    }
}

pub trait WindowAble<T> where T: Clone, Self: Iterator<Item = T> + Sized {
    /// Creates an iterator that outputs vectors containing `window_size` consecutive items returned by this iterator
    fn windows(self, window_size: usize) -> WindowIter<T, Self> {
        WindowIter { buffer: Vec::new(), window_size, iter: self }
    }
}

impl<T: Clone, I: Iterator<Item = T> + Sized> WindowAble<T> for I {}

pub struct ConcatIterator<T, First: Iterator<Item = T>, Second: Iterator<Item = T>> {
    first: First,
    second: Second,
}

impl<T, First: Iterator<Item = T>, Second: Iterator<Item = T>> Iterator for ConcatIterator<T, First, Second> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.first.next().or_else(|| self.second.next())
    }
}

pub trait PrependIterator<T, I: Iterator<Item = T>> where Self: Iterator<Item = T> + Sized {
    fn prepend(self, other: I) -> ConcatIterator<T, Self, I> {
        ConcatIterator { first: self, second: other }
    }
}

pub trait AppendIterator<T, I: Iterator<Item = T>> where Self: Iterator<Item = T> + Sized {
    fn append(self, other: I) -> ConcatIterator<T, I, Self> {
        ConcatIterator { first: other, second: self }
    }
}

impl<T, First: Iterator<Item = T>, Second: Iterator<Item = T>> PrependIterator<T, Second> for First {}
impl<T, First: Iterator<Item = T>, Second: Iterator<Item = T>> AppendIterator<T, First> for Second {}
