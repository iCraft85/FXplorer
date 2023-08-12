use std::{io, thread, time::{Duration, UNIX_EPOCH, Instant}, fmt::format, os::unix::prelude::PermissionsExt};
use ratatui::{prelude::*, widgets::*};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{tUtils::*, permissions::{self, getModified, getSize}};

pub fn uiMain(dirList: DirList) -> Result<(), io::Error> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tickRate = Duration::from_millis(250);

    let res = uiRun(&mut terminal, tickRate, dirList);

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

fn uiRun<B: Backend> (terminal: &mut Terminal<B>, tickRate: Duration, mut dirList: DirList) -> io::Result<()> {
    let lastTick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut dirList))?;
        let timeout = tickRate 
            .checked_sub(lastTick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => dirList.items.unselect(),
                        KeyCode::Char('j') => dirList.items.next(),
                        KeyCode::Char('k') => dirList.items.previous(),
                        _ => {}
                    }
                }
            }
        }
    }
}

// TODO: Make a searchbox
// TODO: Make better names for vars
fn ui<B: Backend>(f: &mut Frame<B>, dirList: &mut DirList) {
    let screen = f.size();
    let [left, right] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(screen)
    else {
        return;
    };

    
    // Splits right side of screen
    let [rTop, rBottom] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(right)
    else {
        return;
    };

    let dirView = Block::default()
        .borders(Borders::ALL)
        .title(block::Title::from("Directory").alignment(Alignment::Center)) // TODO: Change title
                                                                             // to show dir path 
        .border_type(BorderType::Rounded)
        .padding(Padding{left: 2, right: 0, top: 1, bottom: 0});

    let [lLeft, llMid, lrMid, lRight] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(dirView.inner(left))
    else {
        return;
    };

    // Defining Windows (blocks that will appear)

    let contentView = Block::default()
        .borders(Borders::ALL)
        .title(block::Title::from("Content View").alignment(Alignment::Center))
        .border_type(BorderType::Rounded);

    let help = Block::default()
        .borders(Borders::ALL)
        .title(block::Title::from("Help").alignment(Alignment::Center))
        .border_type(BorderType::Rounded);

    //
    // Directory List Names
    // 

    let mut nameList: Vec<ListItem> = vec![];
    for item in &dirList.items.items {
        let listItem = ListItem::new(format!("{}", item.name))
            .style(Style::default().fg(Color::White));
        nameList.push(listItem);
    }

    let mut permList : Vec<ListItem> = vec![];
    for item in &dirList.items.items {
        // let mut permSpan = 
        let mut spans: Vec<Span> = vec![];
        for i in item.perm.chars() {
           if i == 'r' {
               spans.push(Span::styled("r", Style::default()
                            .fg(Color::Green),))
           } 
           if i == 'w' {
               spans.push(Span::styled("w", Style::default()
                            .fg(Color::Yellow),))
           } 
           if i == 'x' {
               spans.push(Span::styled("x", Style::default()
                            .fg(Color::Red),))
           } 
           if i == '-' {
               spans.push(Span::styled("-", Style::default()
                            .fg(Color::DarkGray),))
           } 
        }

        // let listItem = ListItem::new(format!("{}", item.perm))
        //     .style(Style::default().fg(Color::White));

        let listItem = ListItem::new(Line::from(spans));
        permList.push(listItem);
    }

    let mut sizeList : Vec<ListItem> = vec![];
    for item in &dirList.items.items {
        let listItem = ListItem::new(format!("{}", getSize(&item.path).unwrap().to_string()))
            .style(Style::default().fg(Color::White));
        sizeList.push(listItem);
    }

    let mut modifiedList : Vec<ListItem> = vec![];
    for item in &dirList.items.items {

        let time = getModified(&item.path);
        let mut listItem = ListItem::new(format!("--"));

        match(getModified(&item.path)){
            Ok(time) => {
                listItem = ListItem::new(time)
                    .style(Style::default().fg(Color::White));
            }
            Err(err) => {
                listItem = ListItem::new(format!("Error"))
                    .style(Style::default().fg(Color::White));
            }
        }
        
        
        modifiedList.push(listItem);
    }

    let dirNameList = List::new(nameList)
        .block(Block::default()
               .title(block::Title::from("[Path Name]")) // TODO: Display Path Name
               .padding(Padding{left: 0, right: 0, top: 1, bottom: 0})) 
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let dirPermList = List::new(permList)
        .block(Block::default()
               .title(block::Title::from("Perm")) 
               .padding(Padding{left: 0, right: 0, top: 1, bottom: 0})) 
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
    );

    let dirSizeList = List::new(sizeList)
        .block(Block::default()
               .title(block::Title::from("Size")) 
               .padding(Padding{left: 0, right: 0, top: 1, bottom: 0})) 
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
    );

    let dirModifiedList = List::new(modifiedList)
        .block(Block::default()
               .title(block::Title::from("Modified")) 
               .padding(Padding{left: 0, right: 0, top: 1, bottom: 0})) 
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
    );

    // Renders a block by mapping it to a "chunk"
    f.render_widget(dirView, left);

    f.render_widget(dirNameList, lLeft);
    f.render_widget(dirPermList, llMid);
    f.render_widget(dirSizeList, lrMid);
    f.render_widget(dirModifiedList, lRight);

    f.render_widget(contentView, rTop);
    f.render_widget(help, rBottom);

}

