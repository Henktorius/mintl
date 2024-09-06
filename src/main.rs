mod app;
mod styles;
mod tui;

use std::{env, fs};

use app::Task;

fn main() -> std::io::Result<()> {

    let mut terminal = tui::init()?;

    let app_result = match env::current_dir() {
        Ok(env_path) => {
            match fs::read(env_path.join(".mintl")) {
                Ok(content) => {
                    crate::app::App {
                        tasks: content_to_task(content),
                        ..Default::default()
                    }.run(&mut terminal)
                }
                Err(_) => {
                    crate::app::App::default().run(&mut terminal)
                }
            }
        }
        Err(err) => {
            Err(err)
        }
    };

    tui::restore()?;
    app_result
}

fn content_to_task(content: Vec<u8>) -> Vec<Vec<Task>> {
    let mut tasks: Vec<Vec<Task>> = vec![Vec::new(), Vec::new(), Vec::new()];

    content.split(|&data| data == b'\n')
        .map(|s| s.to_vec())
        .enumerate()
        .take(3)
        .for_each(|(i, d)| tasks[i] = d.split(|&data| data == b'\t').map(|s| Task { content: s.to_vec().iter().map(|x| *x as char).collect::<Vec<char>>() }).collect());

    tasks
}
