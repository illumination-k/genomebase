pub mod line;

use std::io::BufRead;

use crate::reader::line::Line;

pub struct GffReader<R> {
    reader: R,
}

impl<R> GffReader<R>
where
    R: BufRead,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    fn read_line(&mut self) -> Result<Line, String> {
        let mut line = String::new();
        let bytes_read = self.reader.read_line(&mut line).map_err(|e| e.to_string())?;

        if bytes_read == 0 {
            return Err("EOF".to_string());
        }

        line.parse::<Line>().map_err(|e| e.to_string())
    }

    
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::directive::Directive;
    use crate::directive::GffVersion;

    #[test]
    fn test_read_line() {
        let data = b"##gff-version 3.1.26\n";
        let mut reader = GffReader::new(&data[..]);
        let line = reader.read_line().unwrap();
        assert_eq!(line, Line::Directive(Directive::GffVersion(GffVersion::new(3, Some(1), Some(26)))));
    }
}
