const PAYCHECK_PERIOD: u32 = 2; // In weeks
const DEPOSIT_AMT: f32 = 250.00;
const MONTHS_TO_ELAPSE: u32 = 6; 

const WEEKS_IN_6M: u32 = 6 * 4;
const WEEKS_IN_1Y: u32 = 12 * 4;

const RATE_6M: f32 = 0.045;
const RATE_1Y: f32 = 0.051;

const INTEREST_PERIODS: usize = (4 / PAYCHECK_PERIOD) as usize;

const NUM_ACCOUNTS_6M: usize = ((6 * 4)  / PAYCHECK_PERIOD) as usize;
const NUM_ACCOUNTS_1Y: usize = ((12 * 4) / PAYCHECK_PERIOD) as usize;


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


fn main() {
  // Initialize data structures and variables -----------------------------------------------------
  let mut accounts_6m: Vec<Account> = Vec::with_capacity(NUM_ACCOUNTS_6M);
  let mut accounts_1y: Vec<Account> = Vec::with_capacity(NUM_ACCOUNTS_1Y);

  // Interest is earned montly. But I might open more than one cd account per month. This will 
  // ensure the correct interest is being calculated for each period.
  let mut period_interest_6m: [f32; INTEREST_PERIODS] = [0.00; INTEREST_PERIODS];
  let mut period_interest_1y: [f32; INTEREST_PERIODS] = [0.00; INTEREST_PERIODS];
  
  let mut total_saved_6m: f32 = 0.00;
  let mut total_saved_1y: f32 = 0.00;
  let mut monthly_interest_accumulated_6m: f32 = 0.00;
  let mut monthly_interest_accumulated_1y: f32 = 0.00;
  
  let mut current_paycheck_period: usize = 0;  

  // Do calculations over elapsed time ------------------------------------------------------------
  while (current_paycheck_period as u32) < (MONTHS_TO_ELAPSE * 4) / PAYCHECK_PERIOD {
    // Each paycheck period create a new CD Account = DEPOSIT_AMT + PERIOD_INTEREST.
    // If an account has 0 weeks until renewal, add `DEPOSIT_AMT` + PERIOD_INTEREST to that account.

    // Figure out which account(s) and interest period we're dealing with -------------------------
    let index_6m: usize = current_paycheck_period % NUM_ACCOUNTS_6M;
    let index_1y: usize = current_paycheck_period % NUM_ACCOUNTS_1Y;
    let index_interest: usize = current_paycheck_period % INTEREST_PERIODS;
    
    let mut interest_earned_6m: f32 = period_interest_6m[index_interest];
    let mut interest_earned_1y: f32 = period_interest_1y[index_interest]; 
  
    // Update new/renewed account balance and interest --------------------------------------------
    if accounts_6m.len() != NUM_ACCOUNTS_6M {
      accounts_6m.push(Account::new(WEEKS_IN_6M));
    }
    if accounts_1y.len() != NUM_ACCOUNTS_1Y {
      accounts_1y.push(Account::new(WEEKS_IN_1Y));
    }

    accounts_6m[index_6m].balance += DEPOSIT_AMT + interest_earned_6m;
    // accounts_6m[index_6m].balance += DEPOSIT_AMT;
    accounts_1y[index_1y].balance += DEPOSIT_AMT + interest_earned_1y;
    accounts_6m[index_6m].interest_per_month = accounts_6m[index_6m].balance * RATE_6M / 12f32;
    accounts_1y[index_1y].interest_per_month = accounts_1y[index_1y].balance * RATE_1Y / 12f32;
    accounts_6m[index_6m].weeks_until_renewal = WEEKS_IN_6M;
    accounts_1y[index_1y].weeks_until_renewal = WEEKS_IN_1Y;  
    
    // Update the interest earned this period -----------------------------------------------------
    interest_earned_6m = 0.00;
    interest_earned_1y = 0.00;

    update_interest(
      &mut accounts_6m, 
      &mut total_saved_6m,
      &mut monthly_interest_accumulated_6m,
      &mut interest_earned_6m,
      &index_interest
    );
    update_interest(
      &mut accounts_1y, 
      &mut total_saved_1y,
      &mut monthly_interest_accumulated_1y,
      &mut interest_earned_1y,
      &index_interest
    );

    period_interest_6m[index_interest] = interest_earned_6m;
    period_interest_1y[index_interest] = interest_earned_1y;

    current_paycheck_period += 1;

    println!("\n");
    println!("=============================================================");
    let week: u32 = (current_paycheck_period as u32) * PAYCHECK_PERIOD;
    let month: u32 = week / 4;
    let year: u32 = month / 12;
    println!("Year {} Month {} Week {}", year, (month % 12) + 1, (week % 4) + 1);
    println!("6 Month CD Accounts");
    println!(
      "Total Saved: ${:.2} | Monthly Interest: ${:.2}\n",
      total_saved_6m,
      monthly_interest_accumulated_6m,
    );
    print_info(&accounts_6m);
    println!("-------------------------------------------------------------");
    println!("1 Year CD Accounts:");
    println!(
      "Total Saved: ${:.2} | Monthly Interest: ${:.2}\n",
      total_saved_1y,
      monthly_interest_accumulated_1y,
    );
    print_info(&accounts_1y);
  }

  // Display final results ------------------------------------------------------------------------
  println!("\n\n");
  println!(
    "6m Total Saved: ${:.2} | 6m Monthly Interest: ${:.2}",
    total_saved_6m,
    monthly_interest_accumulated_6m,
  );
  println!(
    "1y Total Saved: ${:.2} | 1y Monthly Interest: ${:.2}",
    total_saved_1y,
    monthly_interest_accumulated_1y,
  );
}


fn update_interest(
  accounts: &mut Vec<Account>, 
  total_saved: &mut f32, 
  monthly_interest_accumulated: &mut f32,
  interest_earned: &mut f32,
  index_interest: &usize,
) {
  let mut current_index: usize = 0;
  *total_saved = 0.00;
  *monthly_interest_accumulated = 0.00;

  for account in accounts.iter_mut() {
    // Only calculate interest for the accounts earning interest this period.
    if current_index % INTEREST_PERIODS == *index_interest {
      *interest_earned += account.interest_per_month;
    }

    account.weeks_until_renewal -= PAYCHECK_PERIOD;

    // Update totals
    *total_saved += account.balance;
    *monthly_interest_accumulated += account.interest_per_month;

    current_index += 1;
  }
}


fn print_info(accounts: &Vec<Account>) {
  println!("Account # | Weeks Until Renewal | Interest Per Month | Balance"); 
  let mut index: u32 = 1;
  for account in accounts {
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
