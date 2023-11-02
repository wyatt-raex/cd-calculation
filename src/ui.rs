use eframe::egui;

use crate::app::{ Case, Periods };

pub fn run(case: Case) -> Result<(), eframe::Error> {

  // TEMP APP STATE
  let mut name = "Wyatt".to_owned();
  let mut age = 24;

  let mut saved = 0.00f32;

  let mut selected = Periods::M6;

  let options = eframe::NativeOptions {
    initial_window_size: Some(egui::vec2(320.0, 240.0)),
    ..Default::default()
  };
  
  eframe::run_simple_native("CD Calculator", options, move |ctx, _frame| {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("CD Calculator");
      ui.horizontal(|ui| {
        let name_label = ui.label("My Label: ");
        ui.text_edit_singleline(&mut name).labelled_by(name_label.id);
      });
      ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
      if ui.button("Click each year").clicked() {
        age += 1;
      }
      ui.label(format!("Hello '{name}', age {age}"));


      egui::ComboBox::from_label("Account Period:")
      .selected_text(format!("{}", selected))
      .show_ui(ui, |ui| {
        ui.selectable_value(&mut selected, Periods::M6,  "6 Months");
        ui.selectable_value(&mut selected, Periods::M12, "12 Months");
        ui.selectable_value(&mut selected, Periods::M18, "18 Months");
        ui.selectable_value(&mut selected, Periods::M24, "24 Months");
        ui.selectable_value(&mut selected, Periods::M30, "30 Months");
        ui.selectable_value(&mut selected, Periods::M36, "36 Months");
        ui.selectable_value(&mut selected, Periods::M48, "48 Months");
        ui.selectable_value(&mut selected, Periods::M60, "60 Months");
      });
   
      if ui.button("Calculate Case").clicked() {
        let mut ui_case: Case = Case::new(
          selected.clone() as u32, 250.00f32, true, 24u32, 4u32, 0.051f32
        );

        ui_case.calculate();
        ui_case.print_accounts();
        saved = ui_case.total_saved;
      }

      ui.label(format!("Total Saved: ${:.2}", saved));

      // let mut ui_case: Case = Case::new(
      //   selected.clone() as u32, 250.00f32, true, 24u32, 4u32, 0.051f32
      // );

      // ui_case.calculate();

      // ui.label(format!("Total Saved: ${:.2}", ui_case.total_saved));

      // if ui.button("Log Account Period to Console").clicked() {
      //   println!("{:?}", selected);
      // }

      // ui.label(format!("Total Saved: ${:.2}", case.total_saved))
    });
  })  
}
