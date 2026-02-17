use crate::{
    archive::{Exchange, request::Variables, response::ParseWithVariables},
    request::{filter::RequestFilter, name::RequestName},
};
use bounded_static::IntoBoundedStatic;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Enumerate;
use std::marker::PhantomData;
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Parse error")]
    Parse(#[from] crate::archive::parse::Error),
}

pub fn parse_exchanges<V, D, P: AsRef<Path>, F>(
    input: P,
    filter: F,
) -> Result<ExchangeParser<V, D, BufReader<File>, F>, std::io::Error> {
    let reader = BufReader::new(File::open(input)?);

    Ok(ExchangeParser::new(reader, filter))
}

pub fn parse_exchanges_zst<V, D, P: AsRef<Path>, F>(
    input: P,
    filter: F,
) -> Result<
    ExchangeParser<V, D, BufReader<zstd::stream::read::Decoder<'static, BufReader<File>>>, F>,
    std::io::Error,
> {
    let reader = BufReader::new(zstd::stream::read::Decoder::new(File::open(input)?)?);

    Ok(ExchangeParser::new(reader, filter))
}

pub struct ExchangeParser<V, D, R, F> {
    lines: Enumerate<Lines<R>>,
    filter: F,
    _variables: PhantomData<V>,
    _data: PhantomData<D>,
}

impl<V, D, R: BufRead, F> ExchangeParser<V, D, R, F> {
    pub fn new(reader: R, filter: F) -> Self {
        Self {
            lines: reader.lines().enumerate(),
            filter,
            _variables: PhantomData,
            _data: PhantomData,
        }
    }
}

impl<V, D, R: BufRead, F: RequestFilter> ExchangeParser<V, D, R, F>
where
    for<'v> V: Variables<'v> + IntoBoundedStatic,
    for<'d> D: ParseWithVariables<'d, V> + IntoBoundedStatic,
{
    pub fn ignore_excluded(
        self,
    ) -> impl Iterator<Item = Result<Exchange<'static, V::Static, D::Static>, Error>> {
        self.filter_map(|result| {
            result.map_or_else(|error| Some(Err(error)), |result| result.ok().map(Ok))
        })
    }
}

impl<V, D, R: BufRead> ExchangeParser<V, D, R, ()> {
    pub fn new_without_filter(reader: R) -> Self {
        Self::new(reader, ())
    }
}

impl<V, D, R: BufRead, F: RequestFilter> Iterator for ExchangeParser<V, D, R, F>
where
    for<'v> V: Variables<'v> + IntoBoundedStatic,
    for<'d> D: ParseWithVariables<'d, V> + IntoBoundedStatic,
{
    type Item = Result<Result<Exchange<'static, V::Static, D::Static>, RequestName>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|(i, result)| match result {
            Ok(line) => {
                crate::archive::parse::parse_exchange::<V, D, F>(&line, i + 1, &self.filter)
                    .map(|result| result.into_static())
                    .map_err(Error::from)
            }
            Err(error) => Err(error.into()),
        })
    }
}
