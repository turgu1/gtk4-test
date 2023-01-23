// Cargo Dependencies:
//
// gtk = { version = "0.5.5", package = "gtk4" }
// derivative = "2.2.0"

// ----- Observer pattern -----

use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

/// A subscriber (listener) has type of a callable function.
pub type Subscriber<E> = fn(event: E);

/// Publisher sends events to subscribers (listeners).
#[derive(Default)]
pub struct Publisher<E: Eq + Hash + Clone + Copy> {
    events: HashMap<E, Vec<Subscriber<E>>>,
}

impl<E: Eq + Hash + Clone + Copy> Publisher<E> {
    pub fn new() -> Self {
        Self {
            events: HashMap::new()
        }
    }

    pub fn subscribe(&mut self, event_type: E, listener: Subscriber<E>) {
        self.events
            .entry(event_type.clone())
            .or_default();
        self.events
            .get_mut(&event_type)
            .unwrap()
            .push(listener);
    }

    pub fn notify(&self, event: E) {
        let listeners = self.events.get(&event).unwrap();
        for listener in listeners {
            listener(event);
        }
    }
}

// ----- GTK4 Simple Window -----

use gtk::prelude::*;
use gtk::{ Application, ApplicationWindow, Button };

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Event {
  Test,
  Nothing,
}

pub struct Gtk4LowUi {
  app: Application,
  publisher: Rc<RefCell<Publisher<Event>>>,
}

impl Gtk4LowUi  {
  fn new() -> Gtk4LowUi {
    const APP_ID: &str = "com.turgu.test";

    // let ui = Self {
    let ui = Self {
      app: Application::builder()
        .application_id(APP_ID)
        .build(),
      publisher: Rc::new(RefCell::new(Publisher::<Event>::new())),
    };

    const GAP: i32 = 24;

    ui.app.connect_activate(|app| {
      let button = Button::builder()
        .label("This is a GTK4 Test")
        .margin_top(GAP)
        .margin_bottom(GAP)
        .margin_start(GAP)
        .margin_end(GAP)
        .build();

      let window = ApplicationWindow::builder()
        .title("GTK Trials")
        .application(app)
        .child(&button)
        .build();

      
      let publisher = Rc::clone(&ui.publisher);
      button.connect_clicked({
        move |_| { 
          println!("Input handler called..."); 
          publisher.borrow().notify(Event::Test); }});
      window.show();
    });

    ui
  }

  fn run(&self) {
    self.app.run();
  }

  fn subscribe(&mut self, listener: Subscriber<Event>, event_type: Event) {
    self.publisher.borrow_mut().subscribe(event_type, listener);
  }

}

// ----- Main Application -----

fn input_handler(event: Event) {
  println!("It's working! {:?}", event);
}

fn main() {
  let mut ui = Gtk4LowUi::new();
  ui.subscribe(input_handler, Event::Test);
  ui.run();
}