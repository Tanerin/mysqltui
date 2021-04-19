use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use tui::widgets::*;
use tui::layout::*;
use tui::Frame;
use tui::backend::Backend;
use tui::style::*;

fn draw_login<B: Backend>(f: &mut Frame<B>, area: Rect)
where 
{
    let chunks = Layout::default()
        .constraints(
           [
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(60),
                Constraint::Percentage(10),
                Constraint::Percentage(10)
            ].as_ref(),
        )
            .split(area);
        let block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);
        f.render_widget(block, chunks[1]);
        let block = Block::default()
            .title(" Login")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Blue));
        f.render_widget(block, chunks[2]);
        draw_login_block(f, chunks[2]);
       /*  let block = Block::default()
            .borders(Borders::ALL);
        f.render_widget(block, chunks[3]); */

}
fn draw_login_block<B: Backend>(f: &mut Frame<B>, area: Rect)
where
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(40),
                Constraint::Percentage(10),
                Constraint::Percentage(40)
            ].as_ref(),
        )
        .margin(2)
        .split(area);
    let block = Block::default()
    .title(" User-name:")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
    let block = Block::default()
        .title(" Password:")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[3])

}

fn main() -> Result<(), io::Error> {
    //Clear Screen
    println!("{}",termion::clear::All);
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40)
            ].as_ref()
            )
            .split(f.size());
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Black));
        f.render_widget(block, chunks[0]);
        draw_login(f,chunks[1])

        
    })
}
