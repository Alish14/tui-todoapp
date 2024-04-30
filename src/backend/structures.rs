use tui::widgets::ListState; 

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}
pub struct StatefulListDone<T> {
    pub state: ListState,
    pub items_done_arr: Vec<T>,
}

pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub items: StatefulList<String>,
    pub items_done: StatefulListDone<String>,
    pub input_mode: InputMode,
    pub input: String,
    pub messages: Vec<String>,
}

impl <T>StatefulListDone<T>{
    pub fn with_items(items: Vec<T>) -> StatefulListDone<T> {
        StatefulListDone {
            state: ListState::default(),
            items_done_arr:items,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items_done_arr.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items_done_arr.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn add_task(&mut self,task: T){
        self.items_done_arr.push(task);
    }
    pub fn unselect(&mut self) {
        self.state.select(Some(0));
    }
    pub fn delete_task(&mut self){
        if self.items_done_arr.is_empty() {
            return;
        }
        let selected_index = self.state.selected().unwrap();
 
        self.items_done_arr.remove(selected_index);
        self.state.select(None); // Unselect the item
    }
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn task_done(&mut self,other: &mut StatefulListDone<T>)->Option<T>{
        let selected_index = self.state.selected()?;
        if selected_index >= self.items.len() {
            return None;
        }
        let removed_item = self.items.remove(selected_index);
        self.state.select(None); // Unselect the item
        // other.items_done_arr.push(self.items.remove(self.state.selected().unwrap()));
        let i: usize = match self.state.selected(){
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        Some(removed_item)

    }

    pub fn previous(&mut self) {    
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
    pub fn delete_task(&mut self){
        if self.items.is_empty() {
            return;
        }
        let selected_index = self.state.selected().unwrap();
        self.items.remove(selected_index);
        self.state.select(None); // Unselect the item
    }
}


impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            titles: vec!["day1", "day2", "day3", "day4","day5","day6","day7"],
            index: 0,
            messages: Vec::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            items: StatefulList::with_items(vec![
            ]),
            items_done: StatefulListDone::with_items(vec![]),
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }


    pub fn previous(&mut self){
    
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}