use eframe::emath::Align;
use egui::{ColorImage, Direction, Layout, ScrollArea};
use egui::TextEdit;
use egui_extras::{RetainedImage, Size, StripBuilder};
use logo_renderer::Context;

pub struct LogoApp {
    context: Context,
    proc_text: String,
    cmd_log: String,
    cur_cmd: String,
    img: Option<RetainedImage>
}

impl Default for LogoApp {
    fn default() -> Self {
        let mut res = Self {
            context: Context::new(800, 450),
            proc_text: "".to_owned(),
            cmd_log: "".to_owned(),
            cur_cmd: "".to_owned(),
            img: None
        };
        res.run_code();
        res
    }
}

impl LogoApp {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    pub fn run_code(&mut self) {
        self.cmd_log += "\n";
        self.cmd_log += self.cur_cmd.as_str();
        let bytes = self.context.render(&self.proc_text, &self.cur_cmd);
        self.cur_cmd.clear();
        match bytes {
            Ok(bytes) => {
                self.img = Some(RetainedImage::from_color_image("name", ColorImage::from_rgba_unmultiplied([800, 450], bytes.as_slice())));
            }
            Err(e) => {
                self.cmd_log += "\n";
                self.cmd_log += e.as_str();
            }
        }
    }
}

impl eframe::App for LogoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.25))
                .size(Size::remainder())
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ScrollArea::vertical().show(ui, |ui| {
                            ui.label("Procedures");
                            ui.add_sized(ui.available_size(),
                                TextEdit::multiline(&mut self.proc_text)
                                    .font(egui::TextStyle::Monospace)
                                    .lock_focus(true)
                                    .desired_width(f32::INFINITY)
                            );
                        });
                    });
                    strip.strip(|builder| {
                        builder
                            .size(Size::relative(0.7))
                            .size(Size::remainder())
                            .size(Size::exact(30.0))
                            .vertical(|mut strip| {
                                strip.cell(|ui| {
                                    if let Some(img) = &self.img {
                                        let mut size = img.size_vec2();
                                        size *= (ui.available_width() / size.x).min(ui.available_height() / size.y);
                                        img.show_size(ui, size);
                                    }
                                });
                                strip.cell(|ui| {
                                    ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                                        let layout = Layout::from_main_dir_and_cross_align(Direction::BottomUp, Align::Min);
                                        ui.with_layout(layout, |ui| {
                                            ui.label(&self.cmd_log);
                                        });
                                    });
                                });
                                strip.cell(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("Command:");
                                        let edit = TextEdit::singleline(&mut self.cur_cmd)
                                            .desired_width(f32::INFINITY);
                                        let out = edit.show(ui);
                                        let re = out.response;
                                        if re.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                            self.run_code();
                                            re.request_focus();
                                        }
                                    });
                                });
                            });
                    });
                });
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Logo",
        native_options,
        Box::new(|cc| Box::new(LogoApp::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(LogoApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
