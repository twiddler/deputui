use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,              // the currently being edited json key.
    pub value_input: String,            // the currently being edited json value.
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        match &self.currently_editing {
            Some(edit_mode) => match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            },
            _ => self.currently_editing = Some(CurrentlyEditing::Key),
        };
    }

    pub fn abort_editing(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{output}");
        Ok(())
    }

    pub fn type_char(&mut self, value: char) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::Key => self.key_input.push(value),
                CurrentlyEditing::Value => self.value_input.push(value),
            };
        }
    }

    pub fn backspace_content(&mut self) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::Key => self.key_input.pop(),
                CurrentlyEditing::Value => self.value_input.pop(),
            };
        };
    }

    pub fn show_editing_screen(&mut self) {
        self.current_screen = CurrentScreen::Editing;
        self.currently_editing = Some(CurrentlyEditing::Key);
    }

    pub fn show_exit_screen(&mut self) {
        self.current_screen = CurrentScreen::Exiting;
    }

    pub fn start_editing_value(&mut self) {
        self.currently_editing = Some(CurrentlyEditing::Value)
    }

    pub fn confirm_pair(&mut self) {
        self.save_key_value();
        self.current_screen = CurrentScreen::Main;
    }
}
