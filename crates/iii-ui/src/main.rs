use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "III VPN",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    sni_domain: String,
    mode: String,
    connected: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            sni_domain: "fronting.cdn.com".to_string(),
            mode: "Tor".to_string(),
            connected: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("III – Military‑Grade VPN");
            ui.horizontal(|ui| {
                ui.label("SNI Domain:");
                ui.text_edit_singleline(&mut self.sni_domain);
            });
            ui.horizontal(|ui| {
                ui.label("Mode:");
                egui::ComboBox::from_label("")
                    .selected_text(&self.mode)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.mode, "Tor", "Tor");
                        ui.selectable_value(&mut self.mode, "I2P", "I2P");
                        ui.selectable_value(&mut self.mode, "Both", "Both");
                    });
            });
            if ui.button(if self.connected { "Disconnect" } else { "Connect" }).clicked() {
                self.connected = !self.connected;
            }
            if self.connected {
                ui.colored_label(egui::Color32::GREEN, "● Connected via SNI tunnel");
            } else {
                ui.colored_label(egui::Color32::RED, "● Disconnected");
            }
        });
    }
}
