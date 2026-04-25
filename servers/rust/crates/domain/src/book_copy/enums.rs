#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookCopyStatus {
    Active,
    Maintenance,
    Lost,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[error("Unknown book copy status '{input}'")]
pub struct ParseBookCopyStatusError {
    input: String,
}

impl std::fmt::Display for BookCopyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Active => "active",
            Self::Maintenance => "maintenance",
            Self::Lost => "lost",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for BookCopyStatus {
    type Err = ParseBookCopyStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "maintenance" => Ok(Self::Maintenance),
            "lost" => Ok(Self::Lost),
            _ => Err(ParseBookCopyStatusError {
                input: s.to_owned(),
            }),
        }
    }
}
