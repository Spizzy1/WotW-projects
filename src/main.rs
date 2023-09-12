use std::io::{self, Write};
use std::fs;
use std::vec;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

fn main() {
    let mut day = 0;
    print_colored_text(&String::from("Sus"), Color::Green);
    let mut stationDialouge: Vec<(i32, i32, String)> = Vec::new();
    let mut content = match fs::read_to_string("sus.txt"){
        Ok(i) => i,
        Err(_) => panic!("Error"),
    };
    let content = content.split('-');
    println!("{}", content.collect::<String>());
}
