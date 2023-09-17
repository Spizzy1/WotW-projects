use std::alloc::System;
use std::error::Error;
use std::{io, vec};
use std::io::{Write, Stdout};
use std::ops::Add;
use std::{fs};
use std::sync::mpsc;
use std::time::{Duration, SystemTime};
use crossterm::terminal::{disable_raw_mode, self, LeaveAlternateScreen, enable_raw_mode};
use ratatui::prelude::{CrosstermBackend,Style, Layout, Direction, Constraint};
use ratatui::style::Stylize;
use ratatui::widgets::{ListItem, List};
use ratatui::widgets::block::Title;
use std::cmp::min;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use ratatui::{widgets::{Paragraph, Block, Borders}, Terminal};
use terminal::{EnterAlternateScreen};
use crossterm::event::{KeyCode, Event, KeyEventKind, KeyEvent, MouseEvent};
use crossterm::{event, execute};

fn print_colored_text(str : &String,color : Color){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))){
        Ok(_) => (),
        Err(_) => panic!("Error changing color"),
    };
    match writeln!(&mut stdout, "{}", str){
        Ok(_) => (),
        Err(_) => panic!("Failed to print text"),
    };
    WriteColor::reset(&mut stdout).expect("Error resetting color");
    
}


fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>>{
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut text_list: Vec<String> = Vec::new();
    let mut start = SystemTime::now();
    let mut day = 0;
    let mut station = 0;
    let mut station_dialouge: Vec<Vec<String>> = Vec::new();
    let station_amount = 2;
    let mut start_index = 0;
    let mut temp_station = 0;
    let day_length = 1000;

    for i in 0..station_amount{
        station_dialouge.push(Vec::new());
        let content = fs::read_to_string(&format!("station{}.txt", i)[..]).expect("error");
        let mut content : Vec<String> = content.split("\n§").map(|x| String::from(x)).collect();
        println!("{}",content.len());
        for _ in 0..3{
            station_dialouge[i].push(content.remove(0));
        
        }
    };

    Ok(loop {
        if text_list.len() >= 10{
            for _i in [0..(text_list.len())-10]{
                text_list.remove(0);
            }
        }
        let current_station = &station_dialouge[station][day];
        if station != temp_station{
            temp_station = station;
            start_index = min(current_station.len()-1,(current_station.len()) * (start.elapsed()?.as_millis() as usize)/(day_length) as usize);
        }
        let mut text_render = "";
        if current_station.trim() == &String::from("&"){
            text_render = "*Nothing but static...*";
        }
        else if current_station.trim() == &String::from("<"){
            text_render = "*Music is playing...*";
        }
        else{
            text_render = &current_station[start_index..min(current_station.len(),((current_station.len()) * (start.elapsed()?.as_millis() as usize)/(day_length) as usize))];
        }
        if start.elapsed()?.as_millis() > day_length + 1000{
            start = SystemTime::now();
            day += 1;
            if day >= 3{
                break;
            }
        }
        let area = terminal.size().expect("Error fetching terminal size");
        let title = format!("day {}", day);
        terminal.draw(|frame| {
            let header = Block::default().borders(Borders::ALL)
            .title(Title::from(title.as_ref()))
            .title_style(Style::default());
            frame.render_widget(header, area);

            let radio_text = Paragraph::new(text_render).block(Block::default().borders(Borders::ALL).title(Title::from("Radio broadcast")));
            let mut stations = vec![ListItem::new("0"), ListItem::new("1"), ListItem::new("2")];

            stations[station] = ListItem::new(format!("{}", station)).light_red();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                .split(area);
            
            let list = List::new(stations).block(Block::default().borders(Borders::ALL).title("Current station"));
            frame.render_widget(list, chunks[1]);
            frame.render_widget(radio_text, chunks[0]);


        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
                else if key.kind == KeyEventKind::Press{
                    match key.code{
                        KeyCode::Char('0') => {station = 0},
                        KeyCode::Char('1') => {station = 1},
                        _ => (),
                    }
                }
                else{
                    text_list.push(String::from("false"));
                }
            }
        }
    })
}


fn main(){

    let mut terminal = setup_terminal().expect("Error creating terminal");


    run(&mut terminal).expect("Error running terminal");
    restore_terminal(&mut terminal).expect("Error closing terminal");

}

