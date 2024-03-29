use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader, Error as ioe, ErrorKind},
    path,
};

pub struct CommandLine {
    pub query_text: String,
    pub path: String,
    pub ignore_case: bool,
    pub suffix: String,
}

impl CommandLine {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<CommandLine, &'static str> {
        args.next();

        let path = args.next().ok_or("Path is not found")?;
        let query_text = args.next().ok_or("Path is not found")?;

        let mut ignore_case = false;
        let mut suffix = String::from("*");
        for option in args {
            if "--ignore-case" == option {
                ignore_case = true;
            }

            if option.contains("--s=") {
                suffix = option.replace("--s=", "");
            }
        }

        Ok(CommandLine {
            path: path,
            query_text: query_text,
            ignore_case: ignore_case,
            suffix: suffix,
        })
    }

    pub fn copy(s: &CommandLine, path: String) -> CommandLine {
        CommandLine {
            path: path,
            query_text: s.query_text.clone(),
            ignore_case: s.ignore_case,
            suffix: s.suffix.clone(),
        }
    }
}

pub fn run(command_line: &CommandLine) -> Result<(), Box<dyn Error>> {
    let path = path::Path::new(&command_line.path);
    if !path.exists() {
        return Err(Box::new(ioe::new(
            ErrorKind::NotFound,
            format!("Path/Entity Not Found! {}", &command_line.path),
        )));
    }
    if path.is_dir() {
        for entity in fs::read_dir(path)? {
            let entity = entity?;
            let sub = String::from(format!(
                "{}/{}",
                &command_line.path,
                &entity.file_name().to_string_lossy()
            ));
            run(&CommandLine::copy(command_line, sub)).unwrap()
        }
        return Ok(());
    }
    if command_line.suffix != "*" {
        if let Some(name) = path.file_name() {
            if !name.to_string_lossy().ends_with(&command_line.suffix) {
                return Ok(());
            }
        } else {
            return Ok(());
        }
    }

    let reader = BufReader::new(fs::File::open(&command_line.path)?);
    if command_line.ignore_case {
        search_case_insensitive(command_line, reader)
    } else {
        search(command_line, reader)
    }

    Ok(())
}

pub fn search(command_line: &CommandLine, content: BufReader<File>) {
    content
        .lines()
        .enumerate()
        .filter(|(i, r)| match r {
            Ok(line) => line.contains(&command_line.query_text),
            Err(e) => panic!("Read Line Error {}:{}\n{}", command_line.path, i, e),
        })
        .for_each(|(i, r)| match r {
            Ok(line) => println!("{}:{}\n{}", command_line.path, i + 1, line),
            Err(e) => panic!("Read Line Error {}:{}\n{}", command_line.path, i, e),
        });
}

pub fn search_case_insensitive(command_line: &CommandLine, content: BufReader<File>) {
    let revert = command_line.query_text.to_lowercase();
    content
        .lines()
        .enumerate()
        .filter(|(i, r)| match r {
            Ok(line) => line.to_lowercase().contains(&revert),
            Err(e) => panic!("Read Line Error {}:{}\n{}", command_line.path, i, e),
        })
        .for_each(|(i, r)| match r {
            Ok(line) => println!("{}:{}\n{}", command_line.path, i + 1, line),
            Err(e) => panic!("Read Line Error {}:{}\n{}", command_line.path, i, e),
        });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test() {
        let args = vec!["", "./assert/a.txt", "Body", "--ignore_case"];
        let a = args.iter().map(|i| i.to_string());
        let command_line = CommandLine::new(a).unwrap();
        run(&command_line).unwrap();
    }
}
