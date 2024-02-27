use std::fs;
use std::io::{self, stdout};
use crossterm::{
  event::{self, Event, KeyCode},
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

use crate::constants::PROJECTS_DIR;

#[derive(PartialEq)]
pub enum GuiState {
  Home,
  // ProjectNew,
  // ProjectList,
  // ProjectConfiguration,
}

pub struct FpmGui {
  pub state: GuiState,
  pub projects: Vec<String>,
}

impl FpmGui {
  pub fn new() -> FpmGui {
    return FpmGui {
      state: GuiState::Home,
      projects: FpmGui::fill_project_list(),
    }
  }

  pub fn fill_project_list() -> Vec<String> {
    return fs::read_dir(PROJECTS_DIR)
      .map(|entries| {
        entries
          .filter_map(Result::ok)
          .filter_map(|entry| {
            let path = entry.path();
            if path.extension()? == "yml" {
              path.file_stem()?.to_str().map(String::from)
            } else {
              None
            }
          })
          .collect()
      })
      .unwrap_or_else(|_| Vec::new());
  }

  pub fn run(&self) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
      terminal.draw(|f| Self::ui(self, f))?;
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

  fn ui(&self, frame: &mut Frame) {
    match self.state {
      GuiState::Home => {
        let projects: Vec<ListItem> = self.projects.iter().map(|p| ListItem::new(p.clone())).collect();
        let projects_list = List::new(projects)
          .block(Block::default().title("Projects").borders(Borders::ALL))
          .highlight_style(Style::default().add_modifier(Modifier::BOLD))
          .highlight_symbol(">>");

        let size = frame.size();

        let chunks = Layout::default()
          .direction(Direction::Vertical)
          .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
          ])
          .split(size);

        let welcome_message_chunk = Layout::default()
          .direction(Direction::Horizontal)
          .constraints([Constraint::Percentage(100)])
          .split(chunks[0])[0];

        frame.render_widget(Paragraph::new("Welcome to Fast Project Manager")
          .alignment(Alignment::Center)
          .block(Block::default().borders(Borders::ALL)), welcome_message_chunk);

        let left_side_chunks = Layout::default()
          .direction(Direction::Horizontal)
          .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
          ])
          .split(chunks[1]);
        frame.render_widget(projects_list, left_side_chunks[0]);
      },
      _ => {},
    }
  }
}
