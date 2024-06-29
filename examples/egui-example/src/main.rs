fn main() {
    eframe::run_native(
        "egui fontawesome icons demo",
        Default::default(),
        Box::new(|cc| Ok(Box::new(Demo::new(cc)))),
    )
    .unwrap();
}

struct Demo {}

impl Demo {
    fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self {}
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Image::from_uri(
                rust_fontawesome_icons::Icon::RUST.url(),
            ));
        });
    }
}
