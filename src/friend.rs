use std::fmt::Display;

#[allow(dead_code)]
pub enum Friends {
    Pancho,
    Hector,
    Sebastian,
    Juan,
}

impl Display for Friends {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slice: &str = match self {
            Friends::Juan => "Juan",
            Friends::Pancho => "Pancho",
            Friends::Hector => "Hector",
            Friends::Sebastian => "Sebastian",
        };

        write!(f, "{}", slice)
    }
}
