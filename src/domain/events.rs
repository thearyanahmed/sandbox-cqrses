use serde::{Serialize, Deserialize};
use cqrs_es::DomainEvent;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankAccountEvent {
    AccountOpened {
        account_id: String,
    },
    CustomerDepositedMoney {
        amount: f64,
        balance: f64,
    },
    CustomerWithdrewCash {
        amount: f64,
        balance: f64,
    },
    CustomerWroteCheck {
        check_number: String,
        amount: f64,
        balance: f64,
    },
}

impl DomainEvent for BankAccountEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            BankAccountEvent::AccountOpened { .. } => "AccountOpened",
            BankAccountEvent::CustomerDepositedMoney { .. } => "CustomerDepositedMoney",
            BankAccountEvent::CustomerWroteCheck { .. } => "CustomerWroteCheck",
            BankAccountEvent::CustomerWithdrewCash { .. } => "CustomerWithdrewCash",
        };
        
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1".to_string()
    }
}

#[derive(Debug)]
pub struct BankAccountError(String);

impl From<&str> for BankAccountError {
    fn from(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Display for BankAccountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for BankAccountError {}
pub struct AtmError;
pub struct CheckingError;

pub struct BankAccountService;

impl BankAccountService {
    async fn atm_withdrawal(&self, atm_id: &str, amount: f64) -> Result<(), AtmError> {
        Ok(())        
    }

    async fn validate_check(&self, account: &str, check: &str) -> Result<(), CheckingError> {
        Ok(())
    }
}