use super::Task;
use std::{env, fs, path::{Path, PathBuf}, time::Duration};
use regex::Regex;

const _CURRENT_VERSION: &str = "1.0";

pub fn read_data() -> Result<Vec<Task>, String> {
    let mut data_file_path = get_data_dir()?;
    data_file_path.push("data.txt");
    let data_file_contents = match fs::read_to_string(&data_file_path) {
        Ok(contents) => contents,
        Err(_) => return Ok(Vec::new()),
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

fn get_data_dir() -> Result<PathBuf, String> {
    let mut data_dir = match env::var("XDG_DATA_HOME") {
        Ok(path) => Path::new(&path).to_path_buf(),
        Err(_) => {
            let mut home_dir = match env::var("HOME") {
                Ok(path) => Path::new(&path).to_path_buf(),
                Err(_) => return Err("$HOME not found".to_owned()),
            };
            home_dir.push(".local");
            home_dir
        }
    };
    data_dir.push("share");
    data_dir.push("jtodo");
    Ok(data_dir)
}
