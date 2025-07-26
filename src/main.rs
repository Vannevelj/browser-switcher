use std::env;
use std::path::Path;
use std::process::Command;

use eframe::{egui, App};
use eframe::egui::{ImageSource, Ui};
use egui::Frame;
use egui_extras::install_image_loaders;

#[derive(Default)]
struct MyApp {
    url: String,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>, url: String) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        MyApp {
            url
        }
    }

    fn add_browser(&mut self, ui: &mut Ui, icon: ImageSource, paths: Vec<&str>)  {
        let browser = egui::ImageButton::new(icon);
        if ui.add(browser).clicked() {
            match get_path(paths) {
                Some(path) => {
                    Command::new(path)
                        .arg(self.url.clone())
                        .spawn()
                        .expect("Browser failed to start");
                }
                None => eprintln!("Browser not found on this system."),
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());

        egui::CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            // Expand to the full screen
            let available = ui.available_size();

            // Center the buttons
            ui.allocate_ui(
                available,
                |ui| {
                    ui.horizontal(|ui| {
                        ui.columns(2, |cols| {
                            cols[0].vertical_centered_justified(|ui| {
                                self.add_browser(ui, egui::include_image!("firefox.svg"), vec!["C:\\Program Files\\Mozilla Firefox\\firefox.exe"]);
                            });
                            cols[1].vertical_centered_justified(|ui| {
                                self.add_browser(ui, egui::include_image!("chrome.svg"), vec![
                                    "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
                                    "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
                                ]);
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
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(cc, input.to_owned())))
        })
    )
}

fn get_path(paths: Vec<&str>) -> Option<String> {
    for path in paths {
        if Path::new(path).exists() {
            return Some(path.to_owned());
        }
    }

    None
}