use ufmt;
use ufmt::{uDebug, uDisplay, uWrite, uwrite, uwriteln, Formatter};

/// A two dimensional array with a fixed number columns and a maximum capacity of rows.
/// N rows of M columns.
/// The debug and display implementations are meant to be used with the ufmt crate. The debug
/// implementation will print the headers and the length of the table. The display implementation
/// will print the table in a csv format.
pub struct DataTable<'a, T: Copy + Default + uDebug + uDisplay, const N: usize, const M: usize> {
    headers: [&'a str; M],
    data: [T; N],
    length: usize,
}

#[allow(dead_code)]
impl<'a, T: Copy + Default + uDebug + uDisplay, const N: usize, const M: usize>
    DataTable<'a, T, N, M>
{
    pub fn new(headers: [&'a str; M]) -> Self {
        Self {
            headers,
            data: [T::default(); N],
            length: 0,
        }
    }

    pub fn get(&self, index: usize) -> Result<&T,DataTableError> {
        if index < self.length {
            Ok(&self.data[index])
        } else {
            Err(DataTableError::RowIndexOutOfBounds)
        }
    }

    pub fn append(&mut self, row: T) -> Result<&T, DataTableError> {
        if self.length < N {
            self.data[self.length] = row;
            self.length += 1;
            Ok(&self.data[self.length - 1])
        } else {
            Err(DataTableError::CannotGrowTable)
        }
    }

    pub fn erase(&mut self) {
        self.length = 0;
        for i in 0..N {
            self.data[i] = T::default();
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn headers(&self) -> &[&'a str; M] {
        &self.headers
    }
}

impl<'a, T: Copy + Default + uDebug + uDisplay, const N: usize, const M: usize> uDebug
    for DataTable<'a, T, N, M>
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "DataTable<[\"")?;
        for i in 0..M {
            uwrite!(f, "{}", self.headers[i])?;
            if i < M - 1 {
                uwrite!(f, "\", \"")?;
            }
        }
        uwrite!(f, "\"], length: {}>", self.length)?;
        Ok(())
    }
}

impl<'a, T: Copy + Default + uDebug + uDisplay, const N: usize, const M: usize> uDisplay
    for DataTable<'a, T, N, M>
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        for i in 0..M {
            uwrite!(f, "\"{}\"", self.headers[i])?;
            if i < M - 1 {
                uwrite!(f, ",")?;
            }
        }
        uwrite!(f, "\n")?;
        for i in 0..self.length {
            uwriteln!(f, "{}", self.data[i])?;
        }
        Ok(())
    }
}

pub enum DataTableError {
    RowIndexOutOfBounds,
    CannotGrowTable,
}

impl uDebug for DataTableError {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            DataTableError::RowIndexOutOfBounds => uwrite!(f, "RowIndexOutOfBounds"),
            DataTableError::CannotGrowTable => uwrite!(f, "CannotGrowTable"),
        }
    }
}

impl uDisplay for DataTableError {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uDebug::fmt(self, f)
    }
}