use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_env_file() {
    if let Ok(file) = File::open(".env") {
        println!("Environment variables file found");
        let mut total = 0;
        for line in BufReader::new(file).lines() {
            if let Ok(line) = line {
                let trimmed_line = line.trim();
                let is_readable = !trimmed_line.is_empty() && !trimmed_line.starts_with('#');
                if is_readable {
                    let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim();
                        let value = parts[1].trim();
                        env::set_var(key, value);
                        total += 1;
                    }
                }
            }
        }
        println!("{:?} Environment variables set", total);
    }
}

pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or(String::from(""))
}

pub fn get_bool_env(key: &str) -> bool {
    env::var(key)
        .unwrap_or(String::from("false"))
        .parse::<bool>()
        .unwrap_or(false)
}
