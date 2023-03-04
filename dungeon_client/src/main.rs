use dungeon_sdk::client::Connector;
use std::io::{stdin, stdout, Error};
use std::{thread, time::Duration};
use tui::text::Text;
use tui::widgets::Paragraph;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Wrap},
    Terminal,
};

fn main() -> Result<(), Error> {
    let c = get_connection();
    menu(c)
}

fn menu(c: Connector) -> Result<(), Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    /*
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(format!("DungeonGame v{}", env!("CARGO_PKG_VERSION")))
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;
    */
    terminal.draw(|f| {
        let size = f.size();
        let p = c.get_stats().unwrap().to_string();
        let data = Paragraph::new(Text::from(p))
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(data, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn get_connection() -> Connector {
    println!("Ввкaжіть айпі серверу до якого бажаєте підключитись");
    let (mut server_ip, mut username, mut password) = (String::new(), String::new(), String::new());
    let _ = stdin().read_line(&mut server_ip);
    let mut con = Connector::new(server_ip).unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(-1);
    });

    println!(
        "Вам потрібно авторизуватись! Введіть ваш логін(або придумайте, якщо не реєтрувались):"
    );
    let _ = stdin().read_line(&mut username);
    println!("Пароль: ");
    let _ = stdin().read_line(&mut password);

    con.auth(username.trim().to_string(), password.trim().to_string());
    println!("{}", con.connect().message);

    con
}
