use std::error::Error;
use std::{io, vec,fs,str};
use std::io::Stdout;
use std::time::{Duration, SystemTime};
use crossterm::terminal::{disable_raw_mode, self, LeaveAlternateScreen, enable_raw_mode};
use ratatui::prelude::{CrosstermBackend,Style, Layout, Direction, Constraint, Alignment, Color, Text};
use ratatui::style::{Stylize, Modifier};
use ratatui::widgets::{ListItem, List};
use ratatui::widgets::block::Title;
use std::cmp::min;
use ratatui::{widgets::{Paragraph, Block, Borders, Tabs, Wrap}, Terminal};
use terminal::EnterAlternateScreen;
use crossterm::event::{KeyCode, Event, KeyEventKind, KeyModifiers};
use crossterm::{event, execute};

//Deriving from PartialEq for equality support
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

//Code for finding the index and next menu for the sub-menus
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

//Enables all the terminal settings and creates a new terminal instance 
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>>{
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

//Restores terminal settings and terminates terminal (by taking ownership and dereferencing)
fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

//Main function for input taking and terminal rendering.
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {

    //Data
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
    let mut current_menu = MenuType::MainMenu;
    let mut sub_menu = SubMenu::Main;
    let mut input_state = InputState::Inputting;
    let mut text : String;
    let mut station_indexes = vec![0,1,2];
    let station_names = vec!["shelter.90", "frequency.15", "Billssurvivalguide.24", "PAT.00", "Vanta.07"];

    //Reading from the text files and sorting them into the vector
    for i in 0..station_amount{
        station_dialouge.push(Vec::new());
        let content = fs::read_to_string(&format!("station{}.txt", i)[..]).expect("error");
        let mut content : Vec<String> = content.split("\n§").map(|x| String::from(x)).collect();
        for _ in 0..3{
            station_dialouge[i].push(content.remove(0));
        
        }
    };

    //Render and input loop
    Ok(loop {
        //Unused
        if text_list.len() >= 10{
            for _i in [0..(text_list.len())-10]{
                text_list.remove(0);
            }
        }

        //Sets the current station to the station corresponding to the input
        let mut text_render: &str = "";
        if current_menu == MenuType::MainGame{
            let current_station = &station_dialouge[station_indexes[station]][day];
            if station != temp_station || day != tempday{
                if station != temp_station{
                    temp_station = station;
                }
                if day != tempday{
                    tempday = day;

                }
                //Sets the start index to where you were supposed to be in the text (scaled by character amount and day length)
                start_index = min(current_station.chars().collect::<Vec<_>>().len() as usize-1,(current_station.chars().collect::<Vec<_>>().len()) * (start.elapsed()?.as_millis() as usize)/(day_length) as usize);
            }
            
            //Formats the string depending on special characters.
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
                //Sets the total string to go from the start index to the current time (start index is reset upon changing station, indicated by the changing of "station" and not the changing of "temp_station")
                text = current_station.chars().collect::<Vec<_>>()[start_index..min(current_station.chars().collect::<Vec<_>>().len() as usize,(current_station.chars().collect::<Vec<_>>().len()) * (start.elapsed()?.as_millis() as usize)/(day_length) as usize)].iter().collect();
                text_render = text.as_ref();
            }
            //If the elapsed time is greater than the day length + a slight buffer, go to the next day.
            if start.elapsed()?.as_millis() > day_length + day_length/10{
                start = SystemTime::now();
                day += 1;
                if day >= 3{
                    current_menu = MenuType::Ending;
                }
                else{
                    current_menu = MenuType::DayTransition;
                }
            }
        }
        //Follow code handles rendering, see ratatui for more information
        //Fetches size of terminal window
        let area = terminal.size().expect("Error fetching terminal size");
        let menu_title_bar : &str;
        let menu_text : &str;
        //Since same layout is used for day transitions and the main menu, set these variables to change depending on which one it currently is
        let temp_menu_text = format!("Proceeding to day {}...", day+1);
        let temp_menu_bar = format!("Day {}", day+1);
        if current_menu == MenuType::MainMenu{
            menu_title_bar = "The War of the Worlds Radio BroadCast";
            menu_text = "Welcome to The War of the Worlds Radio BroadCast, please use the arrow keys to navigate through the different menus, when you are done, navigate to the main menu and press any key to start playing!"
        }
        else{
            menu_text = temp_menu_text.as_ref();
            menu_title_bar = temp_menu_bar.as_ref();
        }

        //Cuts the text to be max 3 lines long (gets abit screwy with text wrapping and newlines, but generally makes the text not go beyond the terminal bounds for medium sized terminals)
        let mut render_text = Text::from(text_render);
        if render_text.lines.len() > 3{
            render_text = Text::from(render_text.lines[render_text.lines.len()-3..].to_vec());
        }

        //Renders the terminal
        terminal.draw(|frame| {
            match &current_menu{
                //Main game menu
                MenuType::MainGame => {

                    //Divides the menu into two rects
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .margin(0)
                        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                        .split(area);
                    //Does some type conversion of the render text to make it wrap nicely. Aswell as changing the colors of the secret channels
                    let radio_text = Paragraph::new(render_text).block(Block::default().borders(Borders::ALL).title(Title::from("Radio broadcast"))).wrap(Wrap{trim : true});
                    let mut stations : Vec<ListItem>= station_indexes.iter().map(|f| {let mut item = ListItem::new(station_names[station_indexes[*f]]); if *f > 2 {item = item.blue()} return item}).collect();
                    
                    //Sets the color of the currently selected station to be red (or magenta if it is a secret channel)
                    if station_indexes[station] > 2{
                        stations[station] = ListItem::new(format!("{}", station_names[station_indexes[station]])).magenta();

                    }
                    else{
                        stations[station] = ListItem::new(format!("{}", station_names[station_indexes[station]])).light_red();

                    }
                    

                    //Creates the list widget for current stations by composing the List-item vector
                    let list = List::new(stations).block(Block::default().borders(Borders::ALL).title("Current station"));
                    //Renders the text widget and the list widget
                    frame.render_widget(list, chunks[1]);
                    frame.render_widget(radio_text, chunks[0]);
                },
                MenuType::Ending => {
                    let ending = Paragraph::new("Thanks for playing, press any key to exit!");
                    frame.render_widget(ending, area);
                }
                //Main menu and day transitions
                _ => {
                    //Tab vector and composing it into a widget
                    let titles = vec!["Main Menu", "Controls", "Secret..."];
                    let tabs = Tabs::new(titles).select(sub_menu.index()).style(Style::default().fg(Color::White))
                    .highlight_style(
                        Style::default()
                            .add_modifier(Modifier::BOLD).red()
                    );
                    
                    //renders different  menus depending on the current selected tab
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
                            let mut border_color = Color::Magenta;
                            match input_state{
                                InputState::Correct =>{ input_field =Text::styled("Secret channel has been added...", Style::new().green().bold()); border_color = Color::Green;},
                                InputState::Wrong => {input_field = Text::styled("Incorrect code", Style::new().red().bold()); border_color = Color::Red;},
                                InputState::Inputting => input_field = Text::from(&message[0..message.len()]),
                            }
                            let chunks = Layout::default().constraints([Constraint::Length(1), Constraint::Max(3), Constraint::Ratio(5, 8)]).margin(1).direction(Direction::Vertical).split(area);
                            let input_field = Paragraph::new(input_field).block(Block::new().borders(Borders::ALL).border_style(Style { fg: Some(border_color), bg: Some(Color::default()), underline_color: Some(Color::default()), add_modifier: Modifier::default(), sub_modifier: Modifier::default() }));
                            frame.render_widget(tabs, chunks[0]);
                            frame.render_widget(input_field, chunks[1]);

                        },
                    }
                },

            }


        })?;
        //Input system
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                //Quits if you press control c or q (except when you are inputting aka in the secret menu)
                if KeyCode::Char('q') == key.code && sub_menu != SubMenu::Secret || KeyCode::Char('c') == key.code && KeyModifiers::CONTROL == key.modifiers{
                    break;
                }
                else if key.kind == KeyEventKind::Press{
                    match current_menu{
                        MenuType::MainGame => {
                            match key.code{
                                //Sets the stations on input
                                KeyCode::Char('1') => {station = 0},
                                KeyCode::Char('2') => {station = 1},
                                KeyCode::Char('3') => {station = 2},
                                KeyCode::Char('4') => {if station_indexes.len() > 3 {station = 3;}}
                                KeyCode::Char('5') => {if station_indexes.len() > 4 {station = 4;}}
                                _ => (),
                            }
                        },
                        //Any input during the ending quits the game
                        MenuType::Ending => {
                            break;
                        },
                        //Day transitions and main menu
                        _ => {
                            //Goes to the next tab or previous tab
                            match key.code{
                                KeyCode::Right => sub_menu.next(),
                                KeyCode::Left => sub_menu.previous(),
                                _ => (),
                            }
                            //Input mode for the secret menu.
                            if sub_menu == SubMenu::Secret{
                                match key.code{

                                    //Pushes input to string
                                    KeyCode::Char(c) => {
                                        input_state = InputState::Inputting;
                                        message.push(c);
                                    }
                                    //Pops front of string when pressing backspace
                                    KeyCode::Backspace => {
                                        message.pop();
                                    }

                                    //Checks if you unlocked new things or input something wrong
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
                            //If you are in the main menu and input anything except the tab inputs, go to to the main game.
                            else if sub_menu == SubMenu::Main && key.code != KeyCode::Right && key.code != KeyCode::Left{
                                current_menu = MenuType::MainGame;
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

//Sets up, renders and restores the terminal.
fn main(){

    let mut terminal = setup_terminal().expect("Error creating terminal");


    run(&mut terminal).expect("Error running terminal");
    restore_terminal(&mut terminal).expect("Error closing terminal");

}

