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

    pub fn plot<W>(
        &self,
        value: fn(&T) -> i32,
        f: &mut W,
    ) where
        W: uWrite + ?Sized,
    {
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
        let min_digits = Self::count_digits(min);
        let max_digits = Self::count_digits(max);
        let digits = if min_digits > max_digits {
            min_digits
        } else {
            max_digits
        };

        // now we can calculate the scale factor
        let scale = 1.0 / (max - min) as f32;
        const HEIGHT: i32 = 23;
        // now we can plot the data with rows on horizontal axis and values on vertical axis
        for h in (0..HEIGHT+1).rev() {
            if h == (HEIGHT as f32 * (0 - min) as f32 / (max - min) as f32) as i32 {
                Self::write_n_spaces(digits-1, f);
                uwrite!(f, "0 |").ok();
            } else if h == HEIGHT {
                Self::write_n_spaces(digits-max_digits, f);
                uwrite!(f, "{} |", max).ok();
            } else if h == 0 {
                Self::write_n_spaces(digits-min_digits, f);
                uwrite!(f, "{} |", min).ok();
            } else {
                Self::write_n_spaces(digits, f);
                uwrite!(f, " |").ok();
            }
            for r in 0..self.length() {
                if let Ok(row) = self.get(r) {
                    let value = value(row);
                    let scaled_value = ((value - min) as f32 * scale * HEIGHT as f32) as i32;
                    if scaled_value == h {
                        uwrite!(f, "*").ok();
                    } else if scaled_value > h {
                        uwrite!(f, ".").ok();
                    } else {
                        uwrite!(f, " ").ok();
                    }
                }
            }
            uwriteln!(f,"").ok();
        }
    }

    fn count_digits(value: i32) -> u32 {
        let mut n = value;
        let mut count = 0;
        if n < 0 {
            n = -n;
            count += 1; // for the '-' sign
        }
        loop {
            count += 1;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        count
    }

    fn write_n_spaces<W>(n: u32,  f: &mut W)
    where
        W: uWrite + ?Sized,
    {
        for _ in 0..n {
            uwrite!(f, " ").ok();
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
