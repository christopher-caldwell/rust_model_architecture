use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MemberId(pub i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MemberIdent(pub String);

impl From<MemberIdent> for String {
    fn from(value: MemberIdent) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberStatus {
    Active,
    Suspended,
}

pub struct Member {
    pub id: MemberId,
    pub ident: MemberIdent,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status: MemberStatus,
    pub full_name: String,
    pub max_active_loans: i16,
}

pub struct MemberCreationPayload {
    pub full_name: String,
    pub max_active_loans: i16,
}

pub struct MemberPrepared {
    pub ident: MemberIdent,
    pub full_name: String,
    pub max_active_loans: i16,
    pub status: MemberStatus,
}

impl Member {
    /// Transition: any non-Suspended → Suspended.
    pub fn suspend(&self) -> Result<MemberStatus, MemberError> {
        if self.status == MemberStatus::Suspended {
            return Err(MemberError::CannotBeSuspended);
        }
        Ok(MemberStatus::Suspended)
    }

    /// Transition: Suspended → Active.
    pub fn reactivate(&self) -> Result<MemberStatus, MemberError> {
        if self.status != MemberStatus::Suspended {
            return Err(MemberError::CannotBeReactivated);
        }
        Ok(MemberStatus::Active)
    }

    #[must_use]
    fn can_borrow(&self) -> bool {
        self.status == MemberStatus::Active
    }

    /// Guard: ensures member is eligible to borrow.
    pub fn ensure_can_borrow(&self) -> Result<(), MemberError> {
        if !self.can_borrow() {
            return Err(MemberError::CannotBorrowWhileSuspended);
        }
        Ok(())
    }

    #[must_use]
    fn can_check_out_more_books(&self, active_loan_count: i16) -> bool {
        active_loan_count < self.max_active_loans
    }

    /// Guard: ensures member has not reached their loan limit.
    pub fn ensure_within_loan_limit(&self, active_loan_count: i16) -> Result<(), MemberError> {
        if !self.can_check_out_more_books(active_loan_count) {
            return Err(MemberError::LoanLimitReached);
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MemberError {
    #[error("Member not found")]
    NotFound,
    #[error("Member is already suspended")]
    CannotBeSuspended,
    #[error("Member is not currently suspended")]
    CannotBeReactivated,
    #[error("Member is suspended and cannot borrow new books")]
    CannotBorrowWhileSuspended,
    #[error("Member has reached the maximum number of active loans")]
    LoanLimitReached,
}

impl MemberCreationPayload {
    #[must_use]
    pub fn prepare(self, ident: MemberIdent) -> MemberPrepared {
        MemberPrepared {
            ident,
            full_name: self.full_name,
            max_active_loans: self.max_active_loans,
            status: MemberStatus::Active,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn active_member() -> Member {
        Member {
            id: MemberId(1),
            ident: MemberIdent("TEST-0001".to_string()),
            dt_created: Utc::now(),
            dt_modified: Utc::now(),
            status: MemberStatus::Active,
            full_name: "Alice Smith".to_string(),
            max_active_loans: 3,
        }
    }

    fn suspended_member() -> Member {
        Member {
            status: MemberStatus::Suspended,
            ..active_member()
        }
    }

    #[test]
    fn active_member_can_be_suspended() {
        let member = active_member();
        let result = member.suspend();
        assert_eq!(result.unwrap(), MemberStatus::Suspended);
    }

    #[test]
    fn suspended_member_cannot_be_suspended_again() {
        let member = suspended_member();
        let result = member.suspend();
        assert!(matches!(result, Err(MemberError::CannotBeSuspended)));
    }

    #[test]
    fn suspended_member_can_be_reactivated() {
        let member = suspended_member();
        let result = member.reactivate();
        assert_eq!(result.unwrap(), MemberStatus::Active);
    }

    #[test]
    fn active_member_cannot_be_reactivated() {
        let member = active_member();
        let result = member.reactivate();
        assert!(matches!(result, Err(MemberError::CannotBeReactivated)));
    }

    #[test]
    fn active_member_can_borrow() {
        let member = active_member();
        assert!(member.ensure_can_borrow().is_ok());
    }

    #[test]
    fn suspended_member_cannot_borrow() {
        let member = suspended_member();
        assert!(matches!(
            member.ensure_can_borrow(),
            Err(MemberError::CannotBorrowWhileSuspended)
        ));
    }

    #[test]
    fn member_below_loan_limit_can_check_out() {
        let member = active_member();
        assert!(member.ensure_within_loan_limit(2).is_ok());
    }

    #[test]
    fn member_at_loan_limit_cannot_check_out() {
        let member = active_member();
        assert!(matches!(
            member.ensure_within_loan_limit(3),
            Err(MemberError::LoanLimitReached)
        ));
    }

    #[test]
    fn prepare_sets_active_status() {
        let payload = MemberCreationPayload {
            full_name: "Bob Jones".to_string(),
            max_active_loans: 5,
        };
        let prepared = payload.prepare(MemberIdent("NEW-IDENT".to_string()));
        assert_eq!(prepared.status, MemberStatus::Active);
        assert_eq!(prepared.max_active_loans, 5);
    }
}
