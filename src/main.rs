mod app;
mod styles;
mod tui;

fn main() -> std::io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = crate::app::App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
