use std::io::{self, stdout};
use crossterm::{
  event::{self, Event, KeyCode},
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

pub enum GuiState {
  Home,
  ProjectNew,
  ProjectList,
  ProjectConfiguration,
}

pub struct FpmGui {
  pub state: GuiState,
}

impl FpmGui {
  pub fn new() -> FpmGui {
    FpmGui {
      state: GuiState::Home,
    }
  }

  pub fn run(&self) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
      terminal.draw(FpmGui::ui)?;
      should_quit = self.handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
  }

  fn handle_events(&self) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
      if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
          return Ok(true);
        }
      }
    }
    Ok(false)
  }

  fn ui(frame: &mut Frame) {
    frame.render_widget(
      Paragraph::new("Ratio")
        .block(Block::default().title("Fast Project Manager").borders(Borders::ALL)),
      frame.size(),
    );
  }

}