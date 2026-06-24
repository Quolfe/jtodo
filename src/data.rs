use super::Task;
use std::{env, fs, time::Duration};
use regex::Regex;

const _CURRENT_VERSION: &str = "1.0";

pub fn read_data() -> Result<Vec<Task>, String> {
    let data_dir = get_data_dir()?;
    let dir_iter = match fs::read_dir(&data_dir) {
        Ok(dir_iter) => dir_iter,
        Err(_) => {
            if let Err(_) = fs::create_dir_all(&data_dir) {
                return Err(format!("Unable to create directory {data_dir}"));
            }
            match fs::read_dir(&data_dir) {
                Ok(dir_iter) => dir_iter,
                Err(_) => return Err(format!("Created directory {data_dir}, but was unable to open {data_dir}")),
            }
        },
    };
    let mut data_file_path = None;
    for dirent in dir_iter {
        if let Err(_) = dirent {
            continue;
        }
        let dirent = dirent.unwrap();
        let entry_path = dirent.path();
        if entry_path.is_file() && entry_path.file_name().expect("Should be able to read file name").to_str().unwrap() == "data.txt" {
            data_file_path = Some(entry_path);
            break;
        }
    }
    if let None = data_file_path {
        return Ok(Vec::new());
    }
    let data_file_path = data_file_path.unwrap();
    let data_file_contents = match fs::read_to_string(&data_file_path) {
        Ok(contents) => contents,
        Err(_) => return Err(format!("Unable to read {}", data_file_path.to_str().unwrap())),
    };
    let mut data_file_lines = data_file_contents.lines();
    let _version = data_file_lines.next().unwrap_or("");
    let line_re = Regex::new("(\\d+) (\\d+) (\\d+) (\\d+|-) \\[([\\d ]+)\\] \"(.*)\" \"(.*)\"").unwrap();
    let parse_value = |s: &str, line_num: usize| {
        match s.parse() { Ok(val) => Ok(val), Err(_) => Err(format!("Bad value on line {line_num}: {}", &s)) }
    };
    let mut tasks = Vec::new();
    for (i, line) in data_file_lines.enumerate() {
        let i = i;
        let caps = match line_re.captures(line) {
            Some(caps) => caps,
            None => return Err(format!("Line {i} does not match data format.")),
        };
        let priority: usize = parse_value(&caps[1], i + 1)?;
        let time_due: usize = parse_value(&caps[2], i + 1)?;
        let duration: Duration = Duration::from_mins((parse_value(&caps[3], i+ 1)?) as u64);
        let parent_id: Option<usize> = match &caps[4] {
            "-" => None,
            n => Some(parse_value(n, i + 1)?),
        };
        let subtasks: Vec<usize> = caps[5]
            .split(" ")
            .map(|n| parse_value(n, i + 1))
            .filter(|e| e.is_ok())
            .map(|e| e.unwrap())
            .collect();
        let name = caps[6].to_owned();
        let description = caps[7].to_owned();
        let task = Task::new(i, name, description, priority, time_due, duration, parent_id, subtasks);
        tasks.push(task);
    }
    Ok(tasks)
}

pub fn write_data() -> Result<(), &'static str> {
    Ok(())
}

fn get_data_dir() -> Result<String, String> {
    let mut data_dir = match env::var("XDG_DATA_HOME") {
        Ok(path) => path,
        Err(_) => {
            let mut home_dir = match env::var("HOME") {
                Ok(path) => path,
                Err(_) => return Err("$HOME not found".to_owned()),
            };
            home_dir.push_str("/.local");
            home_dir
        }
    };
    data_dir.push_str("/share");
    data_dir.push_str("/jtodo");
    Ok(data_dir)
}
