use ufmt::{uWrite, uwrite};

pub fn log_csv_headers<W>(f: &mut W, headers: &[&str]) -> Result<(), W::Error>
where
    W: uWrite + ?Sized,
{
    for i in 0..headers.len() {
        uwrite!(f, "\"{}\"", headers[i])?;
        if i < headers.len() - 1 {
            uwrite!(f, ", ")?;
        }
    }
    uwrite!(f, "\n")?;
    Ok(())
}
