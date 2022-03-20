use alloc::string::{String};
use alloc::vec::Vec;
use alloc::vec;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::{print, println, sys};
use crate::vga_buffer::WRITER;

lazy_static! {
    pub static ref TERMINAL: Mutex<Terminal> = Mutex::new(Terminal {
            current_text: String::from(""),
            commands: vec![String::from("help"), String::from("neofetch"), String::from("cpuinfo"), String::from("clear"), String::from("time"), String::from("pciinfo"), String::from("atainfo")]
        });
}


pub struct Terminal {
    current_text: String,
    commands: Vec<String>
}

impl Terminal {
    pub fn backspace(&mut self) {
        self.current_text.pop();
    }

    pub fn write(&mut self, char: char) {
        self.current_text.push(char);
    }

    pub fn enter(&mut self) {
        let mut found_match = false;


        for command in &self.commands {
            let result = self.current_text.find(command);
            match result {
                Some(_) => {
                    found_match = true;

                    if command == "neofetch" {
                        println!("No neofetch for you because this is my OS!")
                    }

                    if command == "help" {
                        for cmd in &self.commands {
                            println!("{}", cmd);
                        }
                    }

                    if command == "clear" {
                        WRITER.lock().clear();
                    }

                    if command == "cpuinfo" {
                        sys::cpu::init();
                    }

                    if command == "time" {
                        sys::clock::init();
                    }

                    if command == "pciinfo" {
                        sys::pci::init();
                    }

                    if command == "atainfo" {
                        sys::ata::init();
                    }
                },
                None => {}
            }
        }

        if !found_match  {
            println!("not very poggers");
        }

        self.current_text.clear();
        print!("lukas&rusty_os:-$ ");
    }
}