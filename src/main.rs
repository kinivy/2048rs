use crossterm::event::{Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};
use crate::game::GameState;
use::crossterm::event;

mod display;
mod game;

fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut state: GameState = GameState::new();
    loop {
        terminal.draw(|f| render(&state, f))?;
        if let Event::Key(key) = event::read()? {
            let is_valid = match key.code {
                KeyCode::Right => state.shift_right(),
                KeyCode::Left => state.shift_left(),
                _ => break Ok(()),
            };
            if is_valid { state.add_tile(); }
        }
    }
}


fn render(state: &GameState, frame: &mut Frame) {
    frame.render_widget(state.clone(), frame.area());
}
