use crate::values::Values;
use ewebsock::{WsMessage, WsReceiver, WsSender};
use std::collections::HashMap;

pub struct App {
    server: String,
    ws: Option<(WsSender, WsReceiver)>,
    values: Values,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            server: "ws://127.0.0.1:8080/socket".into(),
            ws: None,
            values: Default::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some((_, rx)) = self.ws.as_ref() {
            while let Some(e) = rx.try_recv() {
                match e {
                    ewebsock::WsEvent::Opened => {}
                    ewebsock::WsEvent::Message(WsMessage::Text(m)) => {
                        match serde_json::from_str::<HashMap<String, f32>>(&m) {
                            Ok(v) => {
                                for (k, v) in v {
                                    self.values.push(k, v);
                                }
                            }
                            Err(e) => {
                                log::error!("failed to parse: {}", e);
                            }
                        }
                    }
                    ewebsock::WsEvent::Message(_) => {}
                    ewebsock::WsEvent::Error(e) => log::error!("{}", e),
                    ewebsock::WsEvent::Closed => {
                        let ctx = ctx.clone();
                        let wakeup = move || ctx.request_repaint();
                        self.ws = ewebsock::connect_with_wakeup(&self.server, wakeup)
                            .map_err(|e| log::error!("failed to init websocket {}", e))
                            .ok();
                        break;
                    }
                }
            }
        }
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                #[cfg(not(target_arch = "wasm32"))]
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.separator();
                }
                if self.ws.is_none() {
                    if ui.button("connect").clicked() {
                        let ctx = ctx.clone();
                        let wakeup = move || ctx.request_repaint();
                        self.ws = ewebsock::connect_with_wakeup(&self.server, wakeup)
                            .map_err(|e| log::error!("failed to init websocket {}", e))
                            .ok();
                    }
                } else {
                    if ui.button("disconnect").clicked() {
                        self.ws = None;
                    }
                }
                egui::widgets::reset_button(ui, &mut self.values);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.server);
            table(ui, &self.values);
        });
    }
}

fn table(ui: &mut egui::Ui, values: &Values) {
    let mut keys: Vec<_> = values.keys().collect();
    keys.sort();
    use egui_extras::{Column, TableBuilder};
    let table = TableBuilder::new(ui)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto())
        .column(Column::auto());

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Key");
            });
            header.col(|ui| {
                ui.strong("Last Value");
            });
        })
        .body(|mut body| {
            for k in keys {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(k);
                    });
                    row.col(|ui| {
                        if let Some(v) = values.get_last_value_for_key(k) {
                            ui.label(v.to_string());
                        }
                    });
                });
            }
        });
}