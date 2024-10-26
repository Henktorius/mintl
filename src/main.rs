mod app;
mod styles;
mod tui;
mod state;

use std::{env, fs};

fn main() -> std::io::Result<()> {

    let mut terminal = tui::init()?;

    let app_result = match env::current_dir() {
        Ok(env_path) => {
            match fs::read(env_path.join(".mintl")) {
                Ok(content) => {
                    crate::app::App {
                        tasks: state::content_to_task(content),
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
