use std::io;

use io::BufWriter;
use io::Write;

use io::BufRead;

use der::Encode;

#[derive(der::Sequence)]
pub struct NameSerPair {
    pub name: String,
    pub snum: u32,
}

impl NameSerPair {
    pub fn encoded_length(&self) -> Result<u32, io::Error> {
        self.encoded_len()
            .map(|l| l.into())
            .map_err(io::Error::other)
    }

    pub fn to_der_bytes(&self) -> Result<Vec<u8>, io::Error> {
        self.to_der().map_err(io::Error::other)
    }

    pub fn to_writer<W>(&self, wtr: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        self.encode(wtr).map_err(io::Error::other)
    }
}

impl NameSerPair {
    pub fn pairs2writer<W>(pairs: &Vec<Self>, wtr: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        pairs.encode(wtr).map_err(io::Error::other)
    }
}

pub fn names2pairs<I>(names: I) -> impl Iterator<Item = NameSerPair>
where
    I: Iterator<Item = String>,
{
    names.enumerate().map(|pair| {
        let (ix, name) = pair;
        let serial: u32 = ix as u32;
        NameSerPair { name, snum: serial }
    })
}

pub fn names2pairs2der2writer<I, W>(names: I, wtr: &mut W) -> Result<(), io::Error>
where
    I: Iterator<Item = String>,
    W: Write,
{
    let pairs = names2pairs(names);
    let v: Vec<_> = pairs.collect();
    NameSerPair::pairs2writer(&v, wtr)
}

pub fn stdin2names2pairs2der2stdout() -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let lines = il.lines();
    let noerr = lines.map_while(Result::ok);

    let o = io::stdout();
    let mut ol = o.lock();
    {
        let mut bw = BufWriter::new(&mut ol);
        names2pairs2der2writer(noerr, &mut bw)?;
        bw.flush()?;
    }
    ol.flush()
}
