use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
    Frame
};
use std::io;

fn main() -> Result<(), io::Error> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let res = run_app(&mut terminal);

    // Clean up terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    loop {
        // Draw the UI
        terminal.draw(render)?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn render(f: &mut Frame) {

    let size = f.size();

    // First split: horizontal split for left and right sections
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),  // Left side (50% width)
            Constraint::Percentage(50),  // Right side (50% width)
        ].as_ref())
        .split(size);

    // Second split: vertical split for the three boxes on the left
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),  // Top box (33% of left height)
            Constraint::Percentage(33),  // Middle box (33% of left height)
            Constraint::Percentage(34),  // Bottom box (34% of left height)
        ].as_ref())
        .split(horizontal_chunks[0]);

    // Left box 1 (top)
    let block1 = Block::default()
        .title("Box 1")
        .borders(Borders::ALL);
    let text1 = Paragraph::new("Top left box").block(block1);
    f.render_widget(text1, vertical_chunks[0]);

    // Left box 2 (middle)
    let block2 = Block::default()
        .title("Box 2")
        .borders(Borders::ALL);
    let text2 = Paragraph::new("Middle left box").block(block2);
    f.render_widget(text2, vertical_chunks[1]);

    // Left box 3 (bottom)
    let block3 = Block::default()
        .title("Box 3")
        .borders(Borders::ALL);
    let text3 = Paragraph::new("Bottom left box").block(block3);
    f.render_widget(text3, vertical_chunks[2]);

    // Right box
    let block4 = Block::default()
        .title("Right Box")
        .borders(Borders::ALL);
    let text4 = Paragraph::new("Press 'q' to quit.\nThis is the right box.").block(block4);
    f.render_widget(text4, horizontal_chunks[1]);
}
