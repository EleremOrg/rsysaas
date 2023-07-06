use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_env_file() {
    let file = File::open(".env").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed_line = line.trim();
            let is_readable = !trimmed_line.is_empty() && !trimmed_line.starts_with('#');
            if is_readable {
                let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    env::set_var(key, value);
                }
            }
        }
    }
}

pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| String::from(""))
}
