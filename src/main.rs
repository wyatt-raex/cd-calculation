/* My Files */
mod app;
mod ui;

fn main() {
  let mut my_case: app::Case = app::Case::new(
    12u32, 250.00f32, true, 24u32, 4u32, 0.051f32
  );

  my_case.calculate();
  // my_case.print_accounts();

  let _ = ui::run(my_case);

}


  //   if LOG_TO_CONSOLE {
  //     println!("\n");
  //     println!("=============================================================");
  //     let week: u32 = (current_paycheck_period as u32) * PAYCHECK_PERIOD;
  //     let month: u32 = week / 4;
  //     let year: u32 = month / 12;
  //     println!("Year {} Month {} Week {}", year, (month % 12) + 1, (week % 4) + 1);
  //     println!("6 Month CD Accounts");
  //     println!(
  //       "Total Saved: ${:.2} | Monthly Interest: ${:.2}\n",
  //       total_saved_6m,
  //       monthly_interest_accumulated_6m,
  //     );
  //     print_info(&accounts_6m);
  //     println!("-------------------------------------------------------------");
  //     println!("1 Year CD Accounts:");
  //     println!(
  //       "Total Saved: ${:.2} | Monthly Interest: ${:.2}\n",
  //       total_saved_1y,
  //       monthly_interest_accumulated_1y,
  //     );
  //     print_info(&accounts_1y);
  //     // Display final results --------------------------------------------------------------------
  //     println!("\n\n");
  //     println!(
  //       "6m Total Saved: ${:.2} | 6m Monthly Interest: ${:.2}",
  //       total_saved_6m,
  //       monthly_interest_accumulated_6m,
  //     );
  //     println!(
  //       "1y Total Saved: ${:.2} | 1y Monthly Interest: ${:.2}",
  //       total_saved_1y,
  //       monthly_interest_accumulated_1y,
  //     );
  //   }
  // }
// }
