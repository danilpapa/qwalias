use std::io;
use std::process::Command;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Frame, Terminal};

#[derive(Clone, Copy)]
enum Field {
    Name,
    Call,
}

impl Field {
    fn next(self) -> Self {
        match self {
            Self::Name => Self::Call,
            Self::Call => Self::Name,
        }
    }
}

struct App {
    name: String,
    call: String,
    active: Field,
    status: String,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            name: String::new(),
            call: String::new(),
            active: Field::Name,
            status: "Tab: switch field | Enter: install | Esc: quit".to_string(),
            should_quit: false,
        }
    }

    fn active_mut(&mut self) -> &mut String {
        match self.active {
            Field::Name => &mut self.name,
            Field::Call => &mut self.call,
        }
    }

    fn install_aliases(&self) -> Result<String, String> {
        if self.name.trim().is_empty() {
            return Err("Field 'name' is empty".to_string());
        }
        if self.call.trim().is_empty() {
            return Err("Field 'call' is empty".to_string());
        }

        let output = Command::new("./install_aliases.sh")
            .arg(self.name.trim())
            .arg(self.call.trim())
            .output()
            .map_err(|e| format!("Failed to run install_aliases.sh: {e}"))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if stdout.is_empty() {
                Ok("Alias installed".to_string())
            } else {
                Ok(stdout)
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            if stderr.is_empty() {
                Err("install_aliases.sh failed".to_string())
            } else {
                Err(stderr)
            }
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let run_result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    run_result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            let ev = event::read()?;
            if let Event::Key(key) = ev {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => app.should_quit = true,
                    KeyCode::Char('q') if key.modifiers.is_empty() => app.should_quit = true,
                    KeyCode::Tab => app.active = app.active.next(),
                    KeyCode::Backspace => {
                        app.active_mut().pop();
                    }
                    KeyCode::Enter => {
                        app.status = match app.install_aliases() {
                            Ok(msg) => {
                                app.should_quit = true;
                                msg
                            },
                            Err(err) => format!("Error: {err}"),
                        };
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.should_quit = true;
                    }
                    KeyCode::Char(ch) => {
                        app.active_mut().push(ch);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(frame.area());

    let name_block = Block::default()
        .borders(Borders::ALL)
        .title("name")
        .border_style(if matches!(app.active, Field::Name) {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        });

    let call_block = Block::default()
        .borders(Borders::ALL)
        .title("call")
        .border_style(if matches!(app.active, Field::Call) {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        });

    let status_block = Block::default().borders(Borders::ALL).title("status");

    let help = Paragraph::new("Fill both fields. Press Enter to run install_aliases.sh.")
        .block(Block::default().borders(Borders::ALL).title("help"));
    let name = Paragraph::new(app.name.as_str()).block(name_block);
    let call = Paragraph::new(app.call.as_str()).block(call_block);
    let status = Paragraph::new(app.status.as_str()).block(status_block);

    frame.render_widget(name, chunks[0]);
    frame.render_widget(call, chunks[1]);
    frame.render_widget(status, chunks[2]);
    frame.render_widget(help, chunks[3]);
}
