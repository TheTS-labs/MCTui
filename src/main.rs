use std::io;

use app::App;
use tui::Tui;

mod app;
mod tui;
mod screens;

#[tokio::main]
async fn main() -> io::Result<()> {
    color_eyre::install().expect("failed to install color_eyre");
    Tui::init_panic_hook();
    let mut terminal = Tui::init()?;

    App::new().run(&mut terminal).await?;

    Tui::restore()?;
    Ok(())
}
