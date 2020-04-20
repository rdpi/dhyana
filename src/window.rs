use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use std::io::BufReader;
use rodio;
//use crate::timer_display::TimerDisplay;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let window = Self::default();
        window
    }
}

impl Default for Window{
    fn default() -> Self{
        let builder = gtk::Builder::new_from_resource("/dev/rdpi/Builder/window.ui");
        let widget: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("Failed to find the window object");
        let time_entry: gtk::Entry = builder.get_object("time").unwrap();
        time_entry.set_text("30");
        let start_pause_button: gtk::Button = builder.get_object("start_pause_timer").unwrap();
        let reset_button: gtk::Button = builder.get_object("reset_timer").unwrap();

        let state = Rc::new(RefCell::new("reset"));
        
        let time_entry_clone = time_entry.clone();
        let state_clone = Rc::clone(&state);
        let start_pause_button_clone = start_pause_button.clone();
        gtk::timeout_add(1000, move || {
            let secs = time_entry_clone.get_text().unwrap().parse::<i32>();
            let mut secs = match secs {
                Ok(secs) => secs,
                Err(_) => 0,
            };
            let current_state = *state_clone.borrow_mut();
            match current_state {
                "paused" => {
                },
                "running" => {
                    secs = secs-1;
                    if secs < 1{
                        *state_clone.borrow_mut() = "reset";
                    }
                    time_entry_clone.set_text(&format!("{:02}",secs));
                },
                "reset" => {
                    start_pause_button_clone.set_label("Start");
                    //time_entry_clone.set_text(&format!("30"));
                },
                _ => {
                }
            }
            Continue(true)
        });
        
        let state_clone = Rc::clone(&state);
        let time_entry_clone = time_entry.clone();
        start_pause_button.connect_clicked(move |btn|{
            let current_state = *state_clone.borrow_mut();
            match current_state{
                "paused" | "reset" => {
                    *state_clone.borrow_mut() = "running";
                    btn.set_label("Pause");
                    time_entry_clone.set_editable(false);
                    let device = rodio::default_output_device().unwrap();
                    let file = std::fs::File::open("/home/rdpi/Projects/Dhyana/src/gong.mp3").unwrap();
                    //let r = rodio::play_once(&device, BufReader::new(file));
                    //match r{
                    //    Ok(r) => r.detach(),
                    //    Err(_) => println!("oops"),
                    //};
                },
                "running" => {
                    *state_clone.borrow_mut() = "paused";
                    btn.set_label("Start");
                    time_entry_clone.set_editable(true);
                },
                _ => {
                }
            }
        });
        
        let state_clone = Rc::clone(&state);
        let time_entry_clone = time_entry.clone();
        let start_pause_button_clone = start_pause_button.clone();
        reset_button.connect_clicked(move |_|{
            *state_clone.borrow_mut() = "reset";
            time_entry_clone.set_text("30");
            start_pause_button_clone.set_label("Start");
        });
        
        Self { widget }
    }
}

