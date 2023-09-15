use std::io;
use std::io::{Write, Stdout};
use std::{fs, error, thread};
use std::time::{Duration, SystemTime};
use crossterm::terminal::{disable_raw_mode, self};
use ratatui::prelude::CrosstermBackend;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use ratatui::{widgets::{Block, Borders}, Terminal};
use terminal::{EnterAlternateScreen};
use crossterm::event::{KeyCode, Event, EnableMouseCapture, KeyEvent, KeyEventKind};
use crossterm::{event, execute};

fn print_colored_text(str : &String,color : Color){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    match stdout.set_color(ColorSpec::new().set_fg(Some(color))){
        Ok(_) => (),
        Err(_) => panic!("Error changing color"),
    };
    match writeln!(&mut stdout, "{}", str){
        Ok(_) => (),
        Err(_) => panic!("Failed to print text"),
    };
    WriteColor::reset(&mut stdout).expect("Error resetting color");
    
}

fn main(){
    let now = SystemTime::now();
    
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("Error");

    let tbackend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(tbackend).expect("Error");

    let _day = 0;
    let mut station_dialouge: Vec<Vec<String>> = Vec::new();
    let station_amount = 1;
    for i in 0..station_amount{
        station_dialouge.push(Vec::new());
        let content = fs::read_to_string(&format!("station{}.txt", i)[..]).expect("error");
        let mut content : Vec<String> = content.split("§").map(|x| String::from(x)).collect();
        println!("{}",content.len());
        for _ in 0..7{
            station_dialouge[i].push(content.remove(0));
        
        }
    };

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    }).expect("Error");


    thread::spawn(|| loop {
        match event::read().expect("error reading"){
            Event::Key(event) => println!("{:?}", event.kind == KeyEventKind::Press && event.code == KeyCode::Char('e')),
            _ => (),

        };
    });
    thread::sleep(Duration::new(5, 1));
}
