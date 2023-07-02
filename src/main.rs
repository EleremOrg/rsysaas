mod business;
mod data;
mod web;

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};
use web::routes::routes;

fn read_env_file() -> std::io::Result<()> {
    let file = File::open(".env")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed_line = line.trim();
            if !trimmed_line.is_empty() && !trimmed_line.starts_with('#') {
                let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    env::set_var(key, value);
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    read_env_file();
    axum::Server::bind(&([0, 0, 0, 0], 8000).into())
        .serve(routes().into_make_service())
        .await
        .unwrap();
}
