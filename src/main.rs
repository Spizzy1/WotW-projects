use std::error::Error;
use std::ops::Add;
use std::{io, vec};
use std::io::{Write, Stdout};
use std::{fs};
use std::time::{Duration, SystemTime};
use crossterm::terminal::{disable_raw_mode, self, LeaveAlternateScreen, enable_raw_mode};
use ratatui::prelude::{CrosstermBackend,Style, Layout, Direction, Constraint, Alignment, Color, Text, Rect};
use ratatui::style::{Stylize, Modifier};
use ratatui::widgets::{ListItem, List};
use ratatui::widgets::block::Title;
use std::cmp::min;
use ratatui::{widgets::{Paragraph, Block, Borders, Tabs, Wrap, Clear}, Terminal};
use terminal::EnterAlternateScreen;
use crossterm::event::{KeyCode, Event, KeyEventKind, KeyModifiers};
use crossterm::{event, execute};
use std::str;

#[derive(PartialEq)]
enum MenuType{
    MainMenu,
    DayTransition,
    MainGame,
    Ending,
}
#[derive(PartialEq)]
enum SubMenu{
    Main,
    Controls,
    Secret,
}
enum InputState{
    Inputting,
    Wrong,
    Correct,
}

impl SubMenu{
    fn next(&mut self){
        match self{
            SubMenu::Main => *self = SubMenu::Controls,
            SubMenu::Controls => *self = SubMenu::Secret,
            SubMenu::Secret => *self = SubMenu::Main,
        }
    }
    fn previous(&mut self){
        match self{
            SubMenu::Main => *self = SubMenu::Secret,
            SubMenu::Controls => *self = SubMenu::Main,
            SubMenu::Secret => *self = SubMenu::Controls,
        }
    }
    fn index(&self) -> usize{
        match self{
            SubMenu::Main => 0,
            SubMenu::Controls => 1,
            SubMenu::Secret => 2,
        }
    }
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
    let mut tempday = 0;
    let mut station = 0;
    let mut station_dialouge: Vec<Vec<String>> = Vec::new();
    let station_amount = 5;
    let mut start_index = 0;
    let mut temp_station = 0;
    let day_length = 50000;
    let mut message = String::new();
    let mut CurrentMenu = MenuType::MainMenu;
    let mut sub_menu = SubMenu::Main;
    let mut input_state = InputState::Inputting;
    let mut text : String = String::new();
    let mut station_indexes = vec![0,1,2];
    let station_names = vec!["shelter.90", "frequency.15", "Billssurvivalguide.24", "PAT.00", "Vanta.07"];
    for i in 0..station_amount{
        station_dialouge.push(Vec::new());
        let content = fs::read_to_string(&format!("station{}.txt", i)[..]).expect("error");
        let mut content : Vec<String> = content.split("\n§").map(|x| String::from(x)).collect();
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
        let mut text_render: &str = "";
        if CurrentMenu == MenuType::MainGame{
            let current_station = &station_dialouge[station_indexes[station]][day];
            if station != temp_station || day != tempday{
                if station != temp_station{
                    temp_station = station;
                }
                if day != tempday{
                    tempday = day;

                }
                start_index = min(current_station.chars().collect::<Vec<_>>().len() as usize-1,(current_station.chars().collect::<Vec<_>>().len()) * (start.elapsed()?.as_millis() as usize)/(day_length) as usize);
            }
    
            if current_station.trim() == &String::from("&"){
                text_render = "*Nothing but static...*";
            }
            else if current_station.trim() == &String::from("<"){
                text_render = "*Music is playing...*";
            }
            else if current_station.trim() == &String::from(">"){
                text_render = "We recommend bringing food or other beneficial item’s with you and seeking shelter immediately.";
            }
            else{
                text = current_station.chars().collect::<Vec<_>>()[start_index..min(current_station.chars().collect::<Vec<_>>().len() as usize-1,(current_station.chars().collect::<Vec<_>>().len()) * (start.elapsed()?.as_millis() as usize)/(day_length) as usize)].iter().collect();
                text_render = text.as_ref();
            }
            if start.elapsed()?.as_millis() > day_length + day_length/10{
                start = SystemTime::now();
                day += 1;
                if day >= 3{
                    CurrentMenu = MenuType::Ending;
                }
                else{
                    CurrentMenu = MenuType::DayTransition;
                }
            }
        }
        let area = terminal.size().expect("Error fetching terminal size");
        let title = format!("day {}", day);
        let menu_title_bar : &str;
        let menu_text : &str;
        let temp_menu_text = format!("Proceeding to day {}...", day);
        let temp_menu_bar = format!("Day {}", day);
        if CurrentMenu == MenuType::MainMenu{
            menu_title_bar = "The War of the Worlds Radio BroadCast";
            menu_text = "Welcome to The War of the Worlds Radio BroadCast, please use the arrow keys to navigate through the different menus, when you are done, navigate to the main menu and press any key to start playing!"
        }
        else{
            menu_text = temp_menu_text.as_ref();
            menu_title_bar = temp_menu_bar.as_ref();
        }
        let mut render_text = Text::from(text_render);
        if render_text.lines.len() > 3{
            render_text = Text::from(render_text.lines[render_text.lines.len()-3..].to_vec());
        }

        terminal.draw(|frame| {
            match &CurrentMenu{
                MenuType::MainGame => {        
                    let radio_text = Paragraph::new(render_text).block(Block::default().borders(Borders::ALL).title(Title::from("Radio broadcast"))).wrap(Wrap{trim : true});
                    let mut stations : Vec<ListItem>= station_indexes.iter().map(|f| {let mut item = ListItem::new(station_names[*f]); if *f > 2 {item = item.blue()} return item}).collect();
                    if station_indexes[station] > 2{
                        stations[station] = ListItem::new(format!("{}", station_names[station_indexes[station]])).magenta();

                    }
                    else{
                        stations[station] = ListItem::new(format!("{}", station_names[station_indexes[station]])).light_red();

                    }
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .margin(0)
                        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                        .split(area);
                    
                    let list = List::new(stations).block(Block::default().borders(Borders::ALL).title("Current station"));
                    frame.render_widget(list, chunks[1]);
                    frame.render_widget(radio_text, chunks[0]);
                },
                MenuType::Ending => {
                    let ending = Paragraph::new("Thanks for playing, press any key to exit!");
                    frame.render_widget(ending, area);
                }
                _ => {
                    let titles = vec!["Main Menu", "Controls", "Secret..."];
                    let tabs = Tabs::new(titles).select(sub_menu.index()).style(Style::default().fg(Color::White))
                    .highlight_style(
                        Style::default().underline_color(Color::LightRed)
                            .add_modifier(Modifier::BOLD)
                    );
                    
                    match sub_menu {
                        SubMenu::Main => {
                            let chunks = Layout::default().constraints([Constraint::Length(1), Constraint::Percentage(95)]).margin(1).direction(Direction::Vertical).split(area);
                            let header = Paragraph::new(Text::from(menu_text)).wrap(Wrap { trim:true })
                            .block(Block::default()
                            .borders(Borders::ALL).title(Title::from(menu_title_bar).alignment(Alignment::Center)));
                            frame.render_widget(tabs, chunks[0]);
                            frame.render_widget(header, chunks[1]);

                        },
                        SubMenu::Controls => {
                            let chunks = Layout::default().constraints([Constraint::Length(1), Constraint::Length(95)]).margin(1).direction(Direction::Vertical).split(area);
                            let controls = vec![
                                ListItem::new("-Numbers to switch between broadcasts (1 for first, 2 for second etc.)"), 
                                ListItem::new("-Go to the secret tab and input for secret channels (enter to confirm)"), 
                                ListItem::new("-Arrow keys to navigate"),
                            ];
                            let list = List::new(controls).block(Block::new().title("Controls").borders(Borders::ALL));
                            frame.render_widget(tabs, chunks[0]);
                            frame.render_widget(list, chunks[1]);

                        },
                        SubMenu::Secret => {
                            let input_field : Text;
                            let mut borderColor = Color::Magenta;
                            match input_state{
                                InputState::Correct =>{ input_field =Text::styled("Secret channel has been added...", Style::new().green().bold()); borderColor = Color::Green;},
                                InputState::Wrong => {input_field = Text::styled("Incorrect code", Style::new().red().bold()); borderColor = Color::Red;},
                                InputState::Inputting => input_field = Text::from(&message[0..message.len()]),
                            }
                            let chunks = Layout::default().constraints([Constraint::Length(1), Constraint::Max(3), Constraint::Ratio(5, 8)]).margin(1).direction(Direction::Vertical).split(area);
                            let inputField = Paragraph::new(input_field).block(Block::new().borders(Borders::ALL).border_style(Style { fg: Some(borderColor), bg: Some(Color::default()), underline_color: Some(Color::default()), add_modifier: Modifier::default(), sub_modifier: Modifier::default() }));
                            frame.render_widget(tabs, chunks[0]);
                            frame.render_widget(inputField, chunks[1]);

                        },
                    }
                },

            }


        })?;
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code && sub_menu != SubMenu::Secret || KeyCode::Char('c') == key.code && KeyModifiers::CONTROL == key.modifiers{
                    break;
                }
                else if key.kind == KeyEventKind::Press{
                    match CurrentMenu{
                        MenuType::MainGame => {
                            match key.code{
                                KeyCode::Char('1') => {station = 0},
                                KeyCode::Char('2') => {station = 1},
                                KeyCode::Char('3') => {station = 2},
                                KeyCode::Char('4') => {if station_indexes.len() > 3 {station = 3;}}
                                KeyCode::Char('5') => {if station_indexes.len() > 4 {station = 4;}}
                                _ => (),
                            }
                        },
                        MenuType::Ending => {
                            break;
                        },
                        _ => {
                            match key.code{
                                KeyCode::Right => sub_menu.next(),
                                KeyCode::Left => sub_menu.previous(),
                                _ => (),
                            }
                            if sub_menu == SubMenu::Secret{
                                match key.code{
                                    KeyCode::Char(c) => {
                                        input_state = InputState::Inputting;
                                        message.push(c);
                                    }
                                    KeyCode::Backspace => {
                                        message.pop();
                                    }
                                    KeyCode::Enter => {
                                        if message == String::from("Omega") || message == String::from("Vanta"){
                                            input_state = InputState::Correct;
                                            if message == String::from("Omega"){
                                                station_indexes.push(3);
                                            }
                                            else if message == String::from("Vanta"){
                                                station_indexes.push(4);
                                            }
                                            
                                        }
                                        else{
                                            input_state = InputState::Wrong;
                                        }
                                        message.clear();

                                    }
                                    _ => (),
                                }
                            }
                            else if sub_menu == SubMenu::Main && key.code != KeyCode::Right && key.code != KeyCode::Left{
                                CurrentMenu = MenuType::MainGame;
                                start = SystemTime::now();
                            }

                        },
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

