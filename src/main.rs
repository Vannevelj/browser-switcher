use std::env;
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::Command;

use eframe::{egui};
use egui::Frame;

#[derive(Default)]
struct MyApp {
    url: String,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>, toUrl: String) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        MyApp {
            url: toUrl
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());

        egui::CentralPanel::default().frame(Frame::none()).show(ctx, |ui| {
            // Expand to the full screen
            let available = ui.available_size();

            // Center the buttons
            ui.allocate_ui(
                available,
                |ui| {
                    ui.horizontal(|ui| {
                        ui.columns(2, |cols| {
                            cols[0].vertical_centered_justified(|ui| {
                                let firefox_style = egui::Button::new(
                                    egui::RichText::new("🦊 Firefox")
                                        .heading()
                                        .strong()
                                        .size(28.0)
                                        .color(egui::Color32::WHITE),
                                ).fill(egui::Color32::from_rgb(255, 69, 0)) // Firefox orange
                                    .min_size(egui::vec2(250.0, 80.0));

                                if ui.add(firefox_style).clicked() {
                                    Command::new("C:\\Program Files\\Mozilla Firefox\\firefox.exe").arg(self.url.clone()).spawn().expect("Firefox failed to start");
                                }
                            });
                            cols[1].vertical_centered_justified(|ui| {
                                let chrome_style = egui::Button::new(
                                    egui::RichText::new("🌐 Chrome")
                                        .heading()
                                        .strong()
                                        .size(28.0)
                                        .color(egui::Color32::WHITE),
                                ).fill(egui::Color32::from_rgb(66, 133, 244)) // Google blue
                                    .min_size(egui::vec2(250.0, 80.0));

                                if ui.add(chrome_style).clicked() {
                                    match get_chrome_path() {
                                        Some(chrome_path) => {
                                            Command::new(chrome_path)
                                                .arg(self.url.clone())
                                                .spawn()
                                                .expect("Chrome failed to start");
                                        }
                                        None => eprintln!("Chrome not found on this system."),
                                    }
                                }
                            });
                        });
                    })
                },
            );
        });
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <string>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    println!("Received argument: {}", input);

    let options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default()
            .with_maximized(false)
            .with_decorations(true)
            .with_transparent(true)
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    _ = eframe::run_native(
        "Browser Launcher",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc, input.to_owned()))))
    )
}

fn get_chrome_path() -> Option<&'static str> {
    let paths = [
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
    ];

    for path in paths {
        if Path::new(path).exists() {
            return Some(path);
        }
    }

    None
}