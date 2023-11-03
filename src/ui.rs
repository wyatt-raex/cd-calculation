use eframe::egui;

use crate::app::{ Case, Periods };

pub fn run() -> Result<(), eframe::Error> {

  // TEMP APP STATE
  let mut deposit_amount: f32 = 250.00f32;
  let mut deposit_interest: bool = true;
  let mut months_to_elapse: u32 = 6u32;
  let mut paycheck_period: u32 = 4u32;
  let mut rate: f32 = 0.01f32;

  let mut account_period = Periods::M6;

  let options = eframe::NativeOptions {
    initial_window_size: Some(egui::vec2(400.0, 240.0)),
    ..Default::default()
  };
  
  eframe::run_simple_native("CD Calculator", options, move |ctx, _frame| {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("CD Calculator");

      egui::ComboBox::from_label("Account Period")
      .selected_text(format!("{}", account_period))
      .show_ui(ui, |ui| {
        ui.selectable_value(&mut account_period, Periods::M6,  "6 Months");
        ui.selectable_value(&mut account_period, Periods::M12, "12 Months");
        ui.selectable_value(&mut account_period, Periods::M18, "18 Months");
        ui.selectable_value(&mut account_period, Periods::M24, "24 Months");
        ui.selectable_value(&mut account_period, Periods::M30, "30 Months");
        ui.selectable_value(&mut account_period, Periods::M36, "36 Months");
        ui.selectable_value(&mut account_period, Periods::M48, "48 Months");
        ui.selectable_value(&mut account_period, Periods::M60, "60 Months");
      });

      // Deposit Amount
      ui.horizontal(|ui| {
        ui.label("Deposit Amount:");
        ui.add(egui::Slider::new(&mut deposit_amount, 0.01f32..=10000.00f32).text("$"));
      });

      // Should deposit interest?
      ui.horizontal(|ui| {
        ui.label("Deposit Interest?:");
        ui.checkbox(&mut deposit_interest, "");
      });

      // Months to Elapse
      ui.horizontal(|ui| {
        ui.label("Months to elapse:");
        ui.add(egui::Slider::new(&mut months_to_elapse, 1u32..=240u32).text("months"));
      });

      // Paycheck period
      ui.horizontal(|ui| {
        ui.label("Paycheck Period:");
        ui.add(egui::Slider::new(&mut paycheck_period, 1u32..=60u32).text("weeks"));
      });

      // Rate
      ui.horizontal(|ui| {
        ui.label("CD Account Interest Rate (yearly):");
        ui.add(egui::Slider::new(&mut rate, 0.01f32..=1.00f32));
      });
   
      if ui.button("Calculate Case").clicked() {
        let mut ui_case: Case = Case::new(
          account_period.clone() as u32, 
          deposit_amount, 
          deposit_interest, 
          months_to_elapse, 
          paycheck_period, 
          rate
        );

        ui_case.calculate();

        // Print out case settings
        println!(
          "\n\nAccount Period: {} | Deposit Amount: ${:.2} | Deposit Interest: {:?} | 
          Months to Elapse: {} | Paycheck Period: {} | Yearly Interest Rate: {:.3}",
          account_period,
          deposit_amount,
          deposit_interest,
          months_to_elapse,
          paycheck_period,
          rate
        );

        ui_case.print_accounts();
      }

      // ui.label(format!("Total Saved: ${:.2}", case.total_saved))
    });
  })  
}
