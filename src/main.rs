use ratatui::{DefaultTerminal, Frame};
use crate::game::GameState;

mod display;
mod game;

fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}


fn render(frame: &mut Frame) {
    let state: GameState = GameState::new();
    frame.render_widget(state, frame.area());
}
