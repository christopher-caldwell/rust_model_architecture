#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberStatus {
    Active,
    Suspended,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[error("Unknown member status '{input}'")]
pub struct ParseMemberStatusError {
    input: String,
}

impl std::fmt::Display for MemberStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Active => "active",
            Self::Suspended => "suspended",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for MemberStatus {
    type Err = ParseMemberStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "suspended" => Ok(Self::Suspended),
            _ => Err(ParseMemberStatusError {
                input: s.to_owned(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_active_status() {
        assert_eq!(MemberStatus::Active.to_string(), "active");
    }

    #[test]
    fn parses_suspended_status() {
        let status = "suspended".parse::<MemberStatus>().unwrap();
        assert_eq!(status, MemberStatus::Suspended);
    }
}
