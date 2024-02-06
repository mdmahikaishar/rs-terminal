pub struct Command {
    pub name: String,
    // flags: HashMap<String, String>,
    // flags_short: HashMap<String, String>,
    pub values: Vec<String>,
}

impl Command {
    pub fn new() -> Command {
        Command {
            name: String::new(),
            values: vec![String::new()],
        }
    }

    pub fn prase(&mut self, input: String) {
        self.values.clear();
        
        input
            .replace("\r\n", "") // remove \newline \row
            .split(' ')
            .enumerate()
            .for_each(|value| {
                if value.0 == 0 {
                    self.name = value.1.to_owned();
                } else if value.1 == "" {
                }
                // else if value.1.starts_with("--") {} // flags
                // else if value.1.starts_with("-") {} // flags_short
                else {
                    self.values.push(value.1.to_owned())
                }
            });
    }
}
