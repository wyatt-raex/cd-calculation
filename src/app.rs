use std::fmt;

// Rust Analyzer is showing a false positive error here. Compiler shows no such error and app
// runs as intended.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Periods {
  M6  = 6,
  M12 = 12,
  M18 = 18,
  M24 = 24,
  M30 = 30,
  M36 = 36,
  M48 = 48,
  M60 = 60,
}

impl fmt::Display for Periods {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Periods::M6  => write!(f, "6 Months"),
      Periods::M12 => write!(f, "12 Months"),
      Periods::M18 => write!(f, "18 Months"),
      Periods::M24 => write!(f, "24 Months"),
      Periods::M30 => write!(f, "30 Months"),
      Periods::M36 => write!(f, "36 Months"),
      Periods::M48 => write!(f, "48 Months"),
      Periods::M60 => write!(f, "60 Months"),
    }
  }
}

struct Account {
  weeks_until_renewal: u32,
  interest_per_month: f32,
  balance: f32,
}

impl Account {
  fn new(period: u32) -> Account {
    Account {
      weeks_until_renewal: period,
      interest_per_month: 0.00,
      balance: 0.00,
    }
  }
}


pub struct Case {
  // User Parameters
  account_period: u32, // in months
  deposit_amount: f32,
  deposit_interest: bool,
  months_to_elapse: u32,
  paycheck_period: u32, // in weeks
  rate: f32,

  // Calculated Parameters
  interest_periods: usize,
  number_of_accounts: usize,

  // Data Parameters
  accounts: Vec<Account>,
  monthly_interest_accumulated: f32,
  period_interest: Vec<f32>,
  total_saved: f32,
}

impl Case {
  pub fn new(
    account_period: u32, // in months
    deposit_amount: f32, 
    deposit_interest: bool,
    months_to_elapse: u32, 
    paycheck_period: u32, // in weeks
    rate: f32) -> Case 
  {
    let interest_periods: usize = (4 / paycheck_period) as usize;
    let number_of_accounts: usize = ((account_period * 4) / paycheck_period) as usize;

    Case {
      // User Parameters
      account_period: account_period,
      deposit_amount: deposit_amount,
      deposit_interest: deposit_interest,
      months_to_elapse: months_to_elapse,
      paycheck_period: paycheck_period,
      rate: rate,

      // Calculated Parameters
      interest_periods: interest_periods,
      number_of_accounts: number_of_accounts,

      // Data Parameters
      accounts: Vec::with_capacity(number_of_accounts),
      monthly_interest_accumulated: 0.00f32,
      period_interest: vec![0.00f32; interest_periods], // Initalize a vec of size `interest_period` with default values 0.00
      total_saved: 0.00f32,
    } 
  }

  pub fn calculate(&mut self) {
    let mut current_paycheck_period: u32 = 0;
    let weeks_to_elapse: u32 = 4 * self.months_to_elapse;
    let periods_to_elapse = weeks_to_elapse / self.paycheck_period;

    while current_paycheck_period < periods_to_elapse {
      // Create a new CD Account each paycheck period, unless an account needs renewed. -------------
      
      // Figure out which account(s) and interest period we're dealing with
      let index_account:  usize = (current_paycheck_period as usize) % self.number_of_accounts;
      let index_interest: usize = (current_paycheck_period as usize) % self.interest_periods;

      let mut interest_earned: f32 = self.period_interest[index_interest];

      // Create a new account if we havn't reached the number of accounts we should have yet 
      if self.accounts.len() != self.number_of_accounts {
        self.accounts.push(Account::new(self.account_period));
      }

      // Update Account details
      self.accounts[index_account].balance += self.deposit_amount;
      if self.deposit_interest { self.accounts[index_account].balance += interest_earned; }

      self.accounts[index_account].interest_per_month = 
        self.accounts[index_account].balance * self.rate / 12f32;

      // This Account is being created or renewed, so reset it's weeks until renewal
      self.accounts[index_account].weeks_until_renewal = self.account_period * 4u32;

      // Update the interest earned this period -----------------------------------------------------
      interest_earned = 0.00f32;

      let mut current_index: usize = 0; 
      self.total_saved = 0.00f32;
      self.monthly_interest_accumulated = 0.00f32;

      for account in self.accounts.iter_mut() {
        // Only calculate interest for the accounts earning interest this period.
        if current_index % self.interest_periods == index_interest {
          interest_earned += account.interest_per_month;
        }

        account.weeks_until_renewal -= self.paycheck_period;

        // Update totals
        self.total_saved += account.balance;
        self.monthly_interest_accumulated += account.interest_per_month;

        current_index += 1;
      }

      // Update total interest for this period
      self.period_interest[index_interest] = interest_earned;
      
      current_paycheck_period += 1;
    }
  }

  pub fn print_accounts(&self) {
    println!("Account # | Weeks Until Renewal | Interest Per Month | Balance"); 
    let mut index: u32 = 1;
    for account in &self.accounts {
      let mut str_index = String::from("         ");
      let mut range = str_index.len() - String::from(index.to_string()).len()..;
      str_index.replace_range(range, &index.to_string());

      let mut str_weeks = String::from("             ");
      range = str_weeks.len() - String::from(account.weeks_until_renewal.to_string()).len()..;
      str_weeks.replace_range(range, &account.weeks_until_renewal.to_string());

      let mut str_interest = String::from("                  ");
      range = str_interest.len() - format!("${:.2}", account.interest_per_month).len()..;
      str_interest.replace_range(range, &format!("${:.2}", account.interest_per_month));

      println!(
        "{} | {} weeks | {} | ${:.2}", 
        str_index, 
        str_weeks, 
        str_interest,
        account.balance,
      );
      
      index += 1;
    }
  }
}

