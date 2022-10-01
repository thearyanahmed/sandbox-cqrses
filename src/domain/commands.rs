use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BankAccountCommand {
    OpenAcount {
        account_id: String, 
    },
    DepositeMoney {
        amount: f64,
    },
    WithdrawMoney {
        amount: f64, 
        atm_id: String,
    },
    WriteCheck { 
        check_number: String, 
        amount: f64, 
    },
}