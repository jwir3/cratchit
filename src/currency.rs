#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Currency {
    USDollar,
    Unknown,
}

impl<'a> From<&'a str> for Currency {
    fn from(abbrev: &'a str) -> Currency {
        match abbrev {
            "USD" => Currency::USDollar,
            _ => Currency::Unknown,
        }
    }
}
