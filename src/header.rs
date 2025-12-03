use super::{Error, ShapeType};

use crate::record::BBoxZ;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::Read;

pub(crate) const HEADER_SIZE: usize = 100;
const FILE_CODE: i32 = 9994;
/// Size of reserved bytes in the header, that have do defined use
const SIZE_OF_SKIP: usize = size_of::<i32>() * 5;

/// struct representing the Header of a shapefile
/// can be retrieved via the reader used to read
#[derive(Copy, Clone, PartialEq)]
pub struct Header {
    /// Total file length (Header + Shapes) in 16bit word
    pub file_length: usize,
    /// The bbox contained all the shapes in this shapefile
    ///
    /// For shapefiles where the shapes do not have `m` or `z` values
    /// the associated min and max will be `0`s.
    pub bbox: BBoxZ,
    /// Type of all the shapes in the file
    /// (as mixing shapes is not allowed)
    pub shape_type: ShapeType,
    /// Version of the shapefile specification
    pub version: i32,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            bbox: BBoxZ::default(),
            shape_type: ShapeType::NullShape,
            file_length: HEADER_SIZE / 2,
            version: 1000,
        }
    }
}

impl Header {
    pub fn read_from<T: Read>(mut source: &mut T, file_size: usize) -> Result<Header, Error> {
        let file_code = source.read_i32::<BigEndian>()?;

        if file_code != FILE_CODE {
            return Err(Error::InvalidFileCode(file_code));
        }

        let mut skip: [u8; SIZE_OF_SKIP] = [0; SIZE_OF_SKIP];
        source.read_exact(&mut skip)?;

        let _ = source.read_i32::<BigEndian>()?;
        let file_length = file_size;
        let version = source.read_i32::<LittleEndian>()?;
        let shape_type = ShapeType::read_from(&mut source)?;

        let mut hdr = Header {
            shape_type,
            version,
            file_length,
            ..Default::default()
        };

        hdr.bbox.min.x = source.read_f64::<LittleEndian>()?;
        hdr.bbox.min.y = source.read_f64::<LittleEndian>()?;
        hdr.bbox.max.x = source.read_f64::<LittleEndian>()?;
        hdr.bbox.max.y = source.read_f64::<LittleEndian>()?;
        hdr.bbox.min.z = source.read_f64::<LittleEndian>()?;
        hdr.bbox.max.z = source.read_f64::<LittleEndian>()?;
        hdr.bbox.min.m = source.read_f64::<LittleEndian>()?;
        hdr.bbox.max.m = source.read_f64::<LittleEndian>()?;

        Ok(hdr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::{Seek, SeekFrom};

    #[test]
    fn wrong_file_code() {
        use std::io::Cursor;

        let mut src = Cursor::new(vec![]);
        crate::byteorder::WriteBytesExt::write_i32::<BigEndian>(&mut src, 42).unwrap();

        src.seek(SeekFrom::Start(0)).unwrap();
        assert!(Header::read_from(&mut src, 1234).is_err());
    }
}
