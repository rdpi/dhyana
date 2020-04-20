use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct TimerDisplay {
    pub label: Rc<RefCell<gtk::Label>>,
    pub source_id: Rc<RefCell<Option<glib::source::SourceId>>>,
    pub time: Rc<RefCell<i32>>,
    pub button: Rc<RefCell<gtk::Button>>,
}

impl TimerDisplay {
    pub fn new(label: gtk::Label, button: gtk::Button) -> Self {
        let source_id = Rc::new(RefCell::new(None));
        let label = Rc::new(RefCell::new(label));
        let button = Rc::new(RefCell::new(button));
        let time = Rc::new(RefCell::new(30));
        TimerDisplay{
            label, button, source_id, time
        }
    }
    
    /*
    pub fn start_timer(mut self) -> glib::source::SourceId{
        //let time = self.time;
        let label = self.label.clone();
        let mut time = self.time;
        self.source_id = Some(gtk::timeout_add(1000, move || {
            time = time - 1;
            if time < 0 {
                return Continue(false);
            }
            label.set_text(&time.to_string());
            Continue(true)
        }));
        self.source_id.unwrap()
    }
    
    fn end_timer(mut self){
        let source_id = self.source_id.take().unwrap();
        glib::source_remove(source_id);
        self.label.set_text("00:00");
    }
    */
}
