use std::env;
use std::path::Path;
use std::process::Command;

use eframe::egui;
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
                                let firefox = egui::ImageButton::new(egui::include_image!("firefox.svg"));
                                if ui.add(firefox).clicked() {
                                    Command::new("C:\\Program Files\\Mozilla Firefox\\firefox.exe").arg(self.url.clone()).spawn().expect("Firefox failed to start");
                                }
                            });
                            cols[1].vertical_centered_justified(|ui| {
                                let chrome = egui::ImageButton::new(egui::include_image!("chrome.svg"));
                                if ui.add(chrome).clicked() {
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
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(cc, input.to_owned())))
        })
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