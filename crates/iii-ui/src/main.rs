use eframe::egui;
use iii_core::Mode;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "III VPN",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(MyApp::default())
        }),
    )
}

struct MyApp {
    sni_domain: String,
    target_relay: String,
    mode: Mode,
    connected: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            sni_domain: "fronting.cdn.com".to_string(),
            target_relay: "1.2.3.4:443".to_string(),
            mode: Mode::SniTor,
            connected: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("III MILITARY‑GRADE VPN").strong().size(24.0));
                ui.label(egui::RichText::new("ANONYMITY VIA SNI + TOR + I2P").italics());
                ui.add_space(20.0);
            });

            ui.group(|ui| {
                ui.label("CORE SETTINGS");
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.label("SNI Domain:  ");
                    ui.text_edit_singleline(&mut self.sni_domain);
                });

                ui.horizontal(|ui| {
                    ui.label("Target Relay:");
                    ui.text_edit_singleline(&mut self.target_relay);
                });

                ui.horizontal(|ui| {
                    ui.label("Chain Mode:  ");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.mode))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.mode, Mode::SniOnly, "SNI Only");
                            ui.selectable_value(&mut self.mode, Mode::SniTor, "SNI + Tor");
                            ui.selectable_value(&mut self.mode, Mode::SniI2p, "SNI + I2P");
                            ui.selectable_value(&mut self.mode, Mode::SniTorI2p, "SNI + Tor + I2P");
                            ui.selectable_value(&mut self.mode, Mode::SniI2pTor, "SNI + I2P + Tor");
                        });
                });
            });

            ui.add_space(20.0);

            let btn_text = if self.connected { "TERMINATE CONNECTION" } else { "ESTABLISH SECURE TUNNEL" };
            let btn_color = if self.connected { egui::Color32::from_rgb(200, 0, 0) } else { egui::Color32::from_rgb(0, 150, 0) };

            ui.vertical_centered(|ui| {
                if ui.add(egui::Button::new(egui::RichText::new(btn_text).strong().color(egui::Color32::WHITE)).fill(btn_color).min_size(egui::vec2(200.0, 40.0))).clicked() {
                    self.connected = !self.connected;
                }
            });

            ui.add_space(20.0);
            
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("STATUS:");
                    if self.connected {
                        ui.colored_label(egui::Color32::GREEN, "● ENCRYPTED CHAIN ACTIVE");
                    } else {
                        ui.colored_label(egui::Color32::GRAY, "○ IDLE");
                    }
                });

                if self.connected {
                    ui.add_space(5.0);
                    ui.small(format!("Routing: App -> TUN -> {:?} -> Relay", self.mode));
                }
            });
        });
    }
}
