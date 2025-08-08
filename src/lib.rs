pub mod fileio;
pub mod errors;
pub mod prelude;

#[cfg(test)]
mod test {
    use crate::{
        fileio::{
            FileIO,
            flags::{
                open_flags::*,
                permission_flags::*,
                whence_flags::*,
                file_type_flags::*
            }
        },
        prelude::*,
    };

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = FileIO::open("test.txt", READ_WRITE | CREATE | TRUNCATE, ALL)?;
        let _ = file.write("Helloworld!")?;
        let _ = file.seek(0, START_POS)?;

        let data = file.read()?;
        assert_eq!(data, "Helloworld!");

        let _ = file.insert(5, ", ")?;
        let _ = file.seek(0, START_POS)?;

        let data = file.read()?;
        assert_eq!(data, "Hello, world!");

        let _ = file.truncate(5)?;

        let _ = file.seek(0, START_POS)?;
        let data = file.read()?;
        assert_eq!(data, "Hello");

        let cursor_pos = file.tell()?;
        assert_eq!(cursor_pos, 5);

        let _ = file.replace("l", "")?;
        let _ = file.seek(0, START_POS)?;
        let data = file.read()?;
        assert_eq!(data, "Heo");

        let metadata = file.metadata()?;

        assert_eq!(metadata.file_type, REGULAR);
        assert_eq!(metadata.size, 3);

        assert!(metadata.uid > 0, "UID should be positive");
        assert!(metadata.gid > 0, "GID should be positive");

        return Ok(());
    }
}