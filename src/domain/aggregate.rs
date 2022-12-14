use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use crate::domain::commands::BankAccountCommand;
use crate::domain::events::{BankAccountError, BankAccountEvent};
use crate::services::BankAccountServices;

#[derive(Serialize, Deserialize)]
pub struct BankAccount {
    account_id: String,
    balance: f64,
}

#[async_trait]
impl Aggregate for BankAccount {
    type Command = BankAccountCommand;
    type Event = BankAccountEvent;
    type Error = BankAccountError;
    type Services = BankAccountServices;

    fn aggregate_type() -> String {
        "Bank Account".to_string()
    }

    async fn handle(&self, command: Self::Command, service: &Self::Services) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            BankAccountCommand::DepositMoney { amount } => {
                let balance = self.balance + amount;
                Ok(vec![
                    BankAccountEvent::CustomerDepositedMoney { amount, balance }
                ])
            },
            BankAccountCommand::WithdrawMoney { amount, atm_id : _ } => {
                let balance = self.balance - amount;

                if balance < 0_f64 {
                    return Err(BankAccountError::from("insufficient fund"))
                }

                Ok(vec![
                    BankAccountEvent::CustomerWithdrewCash { amount, balance }
                ])
            }
            _ => { Ok(vec![])}
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            BankAccountEvent::AccountOpened { account_id } => {
                self.account_id = account_id;
            },
            BankAccountEvent::CustomerDepositedMoney { amount: _, balance } => {
                self.balance = balance;
            },
            BankAccountEvent::CustomerWithdrewCash { amount: _, balance } => {
                self.balance = balance;
            },

            BankAccountEvent::CustomerWroteCheck {
                check_number: _,
                amount: _,
                balance,
            } => {
                self.balance = balance;
            }
        }
    }
}

impl Default for BankAccount {
    fn default() -> Self {
        BankAccount {
            balance: 0_f64,
            account_id: "".to_string(),
        }
    }
}

#[cfg(test)]
mod aggregate_tests {
    use cqrs_es::test::TestFramework;

    type AccountTestFramework = TestFramework<BankAccount>;

    use crate::domain::aggregate::BankAccount;
    use crate::domain::commands::BankAccountCommand;
    use crate::domain::events::{BankAccountError, BankAccountEvent};
    use crate::services::{BankAccountServices};

    #[test]
    fn test_deposit_money() {
        let command = BankAccountCommand::DepositMoney { amount: 200.0 };
        let expected = BankAccountEvent::CustomerDepositedMoney { amount: 200.0, balance: 200.0 };

        AccountTestFramework::with(BankAccountServices)
            .given_no_previous_events()
            .when(command)
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_deposit_money_with_balance() {
        let previous = BankAccountEvent::CustomerDepositedMoney { amount: 200.0, balance: 200.0 };
        let expected = BankAccountEvent::CustomerDepositedMoney { amount: 200.0, balance: 400.0 };
        let command = BankAccountCommand::DepositMoney { amount: 200.0 };

        AccountTestFramework::with(BankAccountServices)
            .given(vec![previous])
            .when(command)
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_withdraw_money() {
        let previous = BankAccountEvent::CustomerDepositedMoney {
            amount: 200.0,
            balance: 200.0,
        };
        let expected = BankAccountEvent::CustomerWithdrewCash {
            amount: 100.0,
            balance: 100.0,
        };

        let command = BankAccountCommand::WithdrawMoney {
            amount: 100.0,
            atm_id: "ATM34f1ba3c".to_string(),
        };

        AccountTestFramework::with(BankAccountServices)
            .given(vec![previous])
            .when(command)
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_withdraw_money_when_fund_is_not_available() {
        AccountTestFramework::with(BankAccountServices)
            .given_no_previous_events()
            .when(BankAccountCommand::WithdrawMoney { amount: 200.0, atm_id: "".to_string() })
            .then_expect_error_message("insufficient fund");
    }
}