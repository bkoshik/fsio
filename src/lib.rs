pub mod fileio;
pub mod flags;
pub mod errors;
pub mod prelude;

#[cfg(test)]
mod test {
    use crate::{
        fileio::builder::FileIOBuilder,
        flags::{
            whence_flags::*,
            file_type_flags::*
        },
        prelude::*,
    };

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = FileIOBuilder::new()
            .read_write()
            .create()
            .truncate()
            .permissions(0o755)
            .open("test.txt")?;
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
        
        let _ = file.seek(0, START_POS)?;
        let _ = file.write("N")?;
        let _ = file.seek(0, START_POS)?;
        let data = file.read()?;
        assert_eq!(data, "Neo");

        let metadata = file.metadata()?;

        assert_eq!(metadata.file_type, REGULAR);
        assert_eq!(metadata.size, 3);

        assert!(metadata.uid > 0, "UID should be positive");
        assert!(metadata.gid > 0, "GID should be positive");

        return Ok(());
    }
}