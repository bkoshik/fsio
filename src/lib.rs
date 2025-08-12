pub mod file;
pub mod flags;
pub mod errors;
pub mod prelude;

#[cfg(test)]
mod test {
    use std::time::Instant;
    use serde_json::Value;
    use crate::{
        file::{
            FileType,
            FileBuilder
        },
        flags::whence_flags::*,
        prelude::*,
    };

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let start_time = Instant::now();

        let mut file = FileBuilder::new()
            .read_write()
            .create()
            .truncate()
            .permissions(0o755)
            .open("test.txt")?;

        let duration = start_time.elapsed();
        println!("Working time creating: {:?}", duration);


        {
            let start_time = Instant::now();

            let _ = file.write("Helloworld!")?;
            let data = file.read_from_start()?;
            assert_eq!(data, "Helloworld!");

            let duration = start_time.elapsed();
            println!("Working time writing/reading from start: {:?}", duration);
        }


        {
            let start_time = Instant::now();

            let _ = file.insert(5, ", ")?;
            let data = file.read_from_start()?;
            assert_eq!(data, "Hello, world!");

            let duration = start_time.elapsed();
            println!("Working time inserting/reading from start: {:?}", duration);
        }


        {
            let start_time = Instant::now();

            let _ = file.truncate(5)?;
            let data = file.read_from_start()?;
            assert_eq!(data, "Hello");

            let duration = start_time.elapsed();
            println!("Working time truncating/reading from start: {:?}", duration);
        }


        {
            let start_time = Instant::now();

            let cursor_pos = file.tell()?;
            assert_eq!(cursor_pos, 5);

            let duration = start_time.elapsed();
            println!("Working time telling: {:?}", duration);
        }


        {
            let start_time = Instant::now();

            let _ = file.replace("l", "")?;
            let data = file.read_from_start()?;
            assert_eq!(data, "Heo");

            let duration = start_time.elapsed();
            println!("Working time replacing/reading from start: {:?}", duration);
        }


        {
            let start_time = Instant::now();

            let _ = file.seek(0, START_POS)?;
            let _ = file.write("N")?;
            let data = file.read_from_start()?;
            assert_eq!(data, "Neo");

            let duration = start_time.elapsed();
            println!("Working time seeking/writing/reading from start: {:?}", duration);
        }


        {
            let start_time = Instant::now();

            let metadata = file.metadata()?;

            assert_eq!(metadata.file_type().unwrap(), FileType::Regular);
            assert_eq!(metadata.size(), 3);

            assert!(metadata.uid() > 0, "UID should be positive");
            assert!(metadata.gid() > 0, "GID should be positive");

            let duration = start_time.elapsed();
            println!("Working time metadata: {:?}", duration);
        }


        {
            let start_time = Instant::now();

            let other_file = FileBuilder::new()
                .read()
                .permissions(0o755)
                .open("test.txt")?;

            assert_eq!(other_file, file);

            let duration = start_time.elapsed();
            println!("Working time eq: {:?}", duration);
        }


        {
            let start_time = Instant::now();

            let _ = file.truncate(0);

            let json_data =
                r#"
                {
                    "name": "JSON Statham",
                    "age": 122,
                    "yes?": true
                }
                "#;
            let _ = file.write(json_data)?;
            let _ = file.json::<Value>()?;

            let duration = start_time.elapsed();
            println!("Working time truncating/writing/deserialize: {:?}", duration);
        }

        return Ok(());
    }
}