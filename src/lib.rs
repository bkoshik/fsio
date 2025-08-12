pub mod file;
pub mod flags;
pub mod errors;
pub mod prelude;

#[cfg(test)]
mod test {
    static TEST_COUNT: usize = 1_000;

    use std::{
        cell::RefCell,
        time::{
            Duration,
            Instant
        }
    };
    use serde_json::Value;
    use crate::{
        file::{
            File,
            FileType,
            FileBuilder
        },
        flags::whence_flags::*,
        prelude::*,
    };

    fn get_file() -> Result<File, Box<dyn std::error::Error>> {
        return Ok(
            FileBuilder::new()
                .read_write()
                .create()
                .truncate()
                .permissions(0o755)
                .open("test.txt")?
        )
    }

    fn get_average<F>(get_duration: F) -> Result<(Duration, Duration, Duration), Box<dyn std::error::Error>>
    where
        F: Fn() -> Result<Duration, Box<dyn std::error::Error>>,
    {
        let results: Vec<Result<Duration, _>> = (0..TEST_COUNT)
            .into_iter()
            .map(|_| get_duration())
            .collect();

        let times: Vec<Duration> = results.into_iter().filter_map(Result::ok).collect();
        let times_sum = times.iter().sum::<Duration>();

        return Ok((
            times.iter().max().unwrap().to_owned(),
            times_sum / times.len() as u32,
            times.iter().min().unwrap().to_owned()
        ))
    }

    #[test]
    fn creating_file() -> Result<(), Box<dyn std::error::Error>> {
        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            // Get duration
            let start_time = Instant::now();
            let _ = get_file()?;
            return Ok(start_time.elapsed())
        };

        let average_duration = get_average(get_duration)?;
        println!("Creating average: {:?}", average_duration);

        Ok(())
    }

    #[test]
    fn reading_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = RefCell::new(get_file()?);
        let _ = file.borrow_mut().write("Hello, world!")?;

        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            let mut file = file.borrow_mut();
            file.seek(0, START_POS)?;

            // Get duration
            let start_time = Instant::now();
            let _ = file.read()?;
            Ok(start_time.elapsed())
        };

        {
            let mut file = file.borrow_mut();

            file.seek(0, START_POS)?;
            let _ = file.write("Hello, world!")?;
            file.seek(0, START_POS)?;
            let data = file.read()?;

            assert_eq!(data, "Hello, world!");
        }

        let average_duration = get_average(get_duration)?;
        println!("Reading average: {:?}", average_duration);

        Ok(())
    }

    #[test]
    fn reading_from_start_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = RefCell::new(get_file()?);
        let _ = file.borrow_mut().write("Hello, world!")?;

        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            let mut file = file.borrow_mut();
            let _ = file.seek(0, START_POS)?;
            let _ = file.write("Hello, world!")?;

            let start_time = Instant::now();
            let _ = file.replace("l", "")?;
            Ok(start_time.elapsed())
        };

        {
            let mut file = file.borrow_mut();

            file.seek(0, START_POS)?;
            let _ = file.write("Hello, world!")?;
            let data = file.read_from_start()?;

            assert_eq!(data, "Hello, world!");
        }

        let average_duration = get_average(get_duration)?;
        println!("Reading from start (read + seek(0)) average: {:?}", average_duration);

        Ok(())
    }

    #[test]
    fn writing_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = RefCell::new(get_file()?);

        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            let mut file = file.borrow_mut();

            let _ = file.truncate(0);

            let start_time = Instant::now();
            let _ = file.write("Hello, world!")?;
            Ok(start_time.elapsed())
        };

        {
            let mut file = file.borrow_mut();
            let _ = file.truncate(0);

            let _ = file.write("Hello, world!")?;

            let data = file.read_from_start()?;
            assert_eq!(data, "Hello, world!");
        }

        let average_duration = get_average(get_duration)?;
        println!("Writing average: {:?}", average_duration);

        Ok(())
    }

    #[test]
    fn inserting_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = RefCell::new(get_file()?);

        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            let mut file = file.borrow_mut();

            let _ = file.truncate(0)?;
            let _ = file.write("Helloworld!")?;

            let start_time = Instant::now();
            let _ = file.insert(5, ", ")?;
            Ok(start_time.elapsed())
        };

        {
            let mut file = file.borrow_mut();

            let _ = file.truncate(0);
            let _ = file.write("Helloworld!")?;
            let _ = file.insert(5, ", ")?;
            let data = file.read_from_start()?;

            assert_eq!(data, "Hello, world!");
        }

        let average_duration = get_average(get_duration)?;
        println!("Inserting average: {:?}", average_duration);

        Ok(())
    }

    #[test]
    fn replacing_by_write_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = RefCell::new(get_file()?);

        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            let mut file = file.borrow_mut();

            let _ = file.seek(0, START_POS)?;
            let _ = file.write("Hello, world!")?;
            let _ = file.seek(0, START_POS)?;

            let start_time = Instant::now();
            let _ = file.write("Ohayo")?;
            Ok(start_time.elapsed())
        };

        {
            let mut file = file.borrow_mut();

            let _ = file.seek(0, START_POS)?;
            let _ = file.write("Hello, world!")?;
            let _ = file.seek(0, START_POS)?;

            let _ = file.write("Ohayo")?;
            let data = file.read_from_start()?;
            assert_eq!(data, "Ohayo, world!");
        }

        let average_duration = get_average(get_duration)?;
        println!("Replacing by write (Hello, world! -> Ohayo, World!) average: {:?}", average_duration);

        Ok(())
    }

    #[test]
    fn seek_test() -> Result<(), Box<dyn std::error::Error>> {
        let file = RefCell::new(get_file()?);

        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            let mut file = file.borrow_mut();

            let _ =file.write("Hello, world!")?;

            let start_time = Instant::now();
            file.seek(0, START_POS)?;
            Ok(start_time.elapsed())
        };

        {
            let mut file = file.borrow_mut();

            file.seek(0, START_POS)?;
            assert_eq!(file.tell()?, 0);
        }

        let average_duration = get_average(get_duration)?;
        println!("Seeking average: {:?}", average_duration);

        Ok(())
    }

    #[test]
    fn telling_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = RefCell::new(get_file()?);

        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            let mut file = file.borrow_mut();
            let _ = file.truncate(0);
            let _ = file.write("Hello, world!")?;

            let start_time = Instant::now();
            let _ = file.tell()?;
            Ok(start_time.elapsed())
        };

        {
            let mut file = file.borrow_mut();

            assert_eq!(file.tell()?, 0);
        }

        let average_duration = get_average(get_duration)?;
        println!("Telling average: {:?}", average_duration);

        Ok(())
    }

    #[test]
    fn truncating_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = RefCell::new(get_file()?);

        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            let mut file = file.borrow_mut();

            let _ = file.seek(0, START_POS)?;
            let _ =file.write("Hello, world!")?;

            let start_time = Instant::now();
            file.truncate(5)?;
            Ok(start_time.elapsed())
        };

        {
            let mut file = file.borrow_mut();

            let _ = file.seek(0, START_POS)?;
            let _ =file.write("Hello, world!")?;
            let _ = file.truncate(5);

            let data = file.read_from_start()?;
            assert_eq!(data, "Hello");
        }

        let average_duration = get_average(get_duration)?;
        println!("Truncating average: {:?}", average_duration);

        Ok(())
    }

    #[test]
    fn metadata_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = RefCell::new(get_file()?);

        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            let mut file = file.borrow_mut();
            let _ = file.seek(0, START_POS)?;
            file.write("Hello, world!")?;

            let start_time = Instant::now();
            let metadata = file.metadata()?;
            let _ = metadata.file_type().unwrap();
            let _ = metadata.size();
            let _ = metadata.uid();
            let _ = metadata.gid();
            Ok(start_time.elapsed())
        };

        {
            let mut file = file.borrow_mut();
            let _ = file.seek(0, START_POS)?;
            file.write("Hello, world!")?;

            let metadata = file.metadata()?;
            assert_eq!(metadata.file_type().unwrap(), FileType::Regular);
            assert_eq!(metadata.size(), 13);
            assert!(metadata.uid() > 0, "UID Should be positive");
            assert!(metadata.gid() > 0, "GID Should be positive");
        }

        let average_duration = get_average(get_duration)?;
        println!("Metadata average: {:?}", average_duration);

        Ok(())
    }

    #[test]
    fn eq_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = RefCell::new(get_file()?);

        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            let mut file = file.borrow_mut();
            file.write("Hello, world!")?;
            let other_file = FileBuilder::new()
                .read()
                .permissions(0o755)
                .open("test.txt")?;

            let start_time = Instant::now();
            assert_eq!(other_file, *file);
            Ok(start_time.elapsed())
        };

        let average_duration = get_average(get_duration)?;
        println!("Eq average: {:?}", average_duration);

        Ok(())
    }

    #[test]
    fn json_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = RefCell::new(get_file()?);

        let get_duration = || -> Result<Duration, Box<dyn std::error::Error>> {
            let mut file = file.borrow_mut();
            let _ = file.truncate(0)?;
            let json_data =
                r#"
                {
                    "name": "JSON Statham",
                    "age": 122,
                    "yes?": true
                }
                "#;
            let _ =file.write(json_data)?;

            let start_time = Instant::now();
            file.json::<Value>()?;
            Ok(start_time.elapsed())
        };

        let average_duration = get_average(get_duration)?;
        println!("Json average: {:?}", average_duration);

        Ok(())
    }
}