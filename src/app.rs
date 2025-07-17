#[path = "rdt 3.0/mod.rs"]
mod rtd3_0;
use rtd3_0::client::client;
use rtd3_0::server::server;

use std::io::{Result, Write, stdin, stdout};

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn run(&self) -> Result<()> {
        let ascii_art = r#"
 __  __     _____     ______      ______     ______     ______     __  __     ______     ______                         
/\ \/\ \   /\  __-.  /\  == \    /\  ___\   /\  __ \   /\  ___\   /\ \/ /    /\  ___\   /\__  _\                        
\ \ \_\ \  \ \ \/\ \ \ \  _-/    \ \___  \  \ \ \/\ \  \ \ \____  \ \  _"-.  \ \  __\   \/_/\ \/                        
 \ \_____\  \ \____-  \ \_\       \/\_____\  \ \_____\  \ \_____\  \ \_\ \_\  \ \_____\    \ \_\                        
  \/_____/   \/____/   \/_/        \/_____/   \/_____/   \/_____/   \/_/\/_/   \/_____/     \/_/                        
                                                                                                                        
 ______   ______     ______     ______     ______     ______     __    __     __    __     __     __   __     ______    
/\  == \ /\  == \   /\  __ \   /\  ___\   /\  == \   /\  __ \   /\ "-./  \   /\ "-./  \   /\ \   /\ "-.\ \   /\  ___\   
\ \  _-/ \ \  __<   \ \ \/\ \  \ \ \__ \  \ \  __<   \ \  __ \  \ \ \-./\ \  \ \ \-./\ \  \ \ \  \ \ \-.  \  \ \ \__ \  
 \ \_\    \ \_\ \_\  \ \_____\  \ \_____\  \ \_\ \_\  \ \_\ \_\  \ \_\ \ \_\  \ \_\ \ \_\  \ \_\  \ \_\\"\_\  \ \_____\ 
  \/_/     \/_/ /_/   \/_____/   \/_____/   \/_/ /_/   \/_/\/_/   \/_/  \/_/   \/_/  \/_/   \/_/   \/_/ \/_/   \/_____/                                                                                                                      
"#;

        println!("{} \n", ascii_art.trim());

        println!("Welcome to UDP Socket Programming\n");
        println!("1 for client mode");
        println!("2 for server mode\n");
        print!("please select your mode : ");

        let mut mode = String::new();

        stdout().flush().unwrap();

        stdin().read_line(&mut mode).expect("cannot read input");

        let mode = mode.trim().parse::<u8>();

        match mode {
            Ok(value) => match value {
                1 => client()?,
                2 => server()?,
                _ => {}
            },
            Err(err) => println!("Error: {:?}", err),
        }

        Ok(())
    }
}
