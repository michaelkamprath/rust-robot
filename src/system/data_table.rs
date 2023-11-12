use ufmt;
use ufmt::{uDebug, uDisplay, uWrite, uwrite, uwriteln, Formatter};
use crate::{print, println};

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

    pub fn get(&self, index: usize) -> Result<&T, DataTableError> {
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

    pub fn plot(&self, value: fn(&T) -> i32) {
        // first we need to scan through the data to find the range of
        // values that we need to plot
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for row in self.data.iter() {
            let value = value(row);
            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }
        }
        // now we can calculate the scale factor
        let scale = 1.0 / (max - min) as f32;
        const HEIGHT: i32 = 20;
        // now we can plot the data with rows on horizontal axis and values on vertical axis
        println!("min: {}, max: {}\n", min, max);
        for h in (0..HEIGHT+1).rev() {
            if h == (HEIGHT as f32 * (0 - min) as f32 / (max - min) as f32) as i32 {
                print!("0 |");
            } else {
                print!("  |");
            }
            for row in self.data.iter() {
                let value = value(row);
                let scaled_value = ((value - min) as f32 * scale * HEIGHT as f32) as i32;
                if scaled_value == h {
                    print!("*");
                } else if scaled_value > h {
                    print!(".");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
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
