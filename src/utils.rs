#[derive(Debug)]
pub enum Action {
    Add,
    Remove,
    Transfer,
    Show,
}

impl Action {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "add" => Some(Action::Add),
            "remove" => Some(Action::Remove),
            "transfer" => Some(Action::Transfer),
            "show" => Some(Action::Show),
            _ => None,
        }
    }
}

pub struct ParsedOutput {
    pub action: Action,
    pub object: Option<String>,
    pub source: Option<String>,
    pub destination: String,
}

pub fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string().to_ascii_lowercase()
}

pub fn parser(input: &str) -> Option<ParsedOutput> {
    let words: Vec<&str> = input.split_whitespace().collect();
    //action
    let action = Action::from_str(words.first()?)?;

    //OBJECT
    let object_start = input.rfind(words.first()?)? + words.first()?.len();
    let object_end = match action {
        Action::Add => input.rfind("to"),
        Action::Remove | Action::Transfer => input.rfind("from"),
        Action::Show => None,
    };
    let object: Option<String> = if let Some(obj_end) = object_end {
        Some(input[object_start..obj_end].trim().to_string())
    } else {
        None
    };
    //Optional source if actions is transfer
    let source: Option<String> = match action {
        Action::Add | Action::Remove => None,
        Action::Transfer => {
            let from = input.rfind("from")? + "from".len();
            let to = input.rfind("to")?;
            Some(input[from..to].trim().to_string())
        }
        Action::Show => None,
    };

    //DESTINATION
    let dest_start: Option<usize> = match action {
        Action::Add | Action::Transfer => Some(input.rfind("to")? + "to".len()),
        Action::Remove => Some(object_end? + "from".len()),
        Action::Show => None,
    };
    let destination: String = if let Some(dest_start) = dest_start {
        input[dest_start..].trim().to_string()
    } else {
        let first = *words.first()?;
        input[first.len()..].trim().to_string()
    };

    Some(ParsedOutput {
        action,
        object: object,
        source: source,
        destination: destination.to_string(),
    })
}

pub fn clear_screen() {
    println!("\x1b[2J\x1b[H");
}

pub fn stay_screen() {
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
