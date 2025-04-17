use eframe::egui;
use egui::{ColorImage, TextureHandle};
use log::info;

// egui: the GUI library.
// run_native: a function to launch a native desktop window.
// HardwareAcceleration, NativeOptions: used to configure how the app window is created (like graphics acceleration).

//You define the state of your app. Here, it just contains one string, name.
struct MyApp {
    image_path: Option<String>,
    texture: Option<TextureHandle>,
}

// This lets you create a default instance of MyApp. It sets the initial name to "World".
// Rust, core::default is a trait that provides a way to create a default value for a type. The trait defines a single method, default, that is used to generate a "default" instance of a type 


impl Default for MyApp {
    fn default() -> Self {
        Self {
            image_path: None,
            texture: None
        }
    }
}


//This is where your UI is built and updated every frame.
// ctx: This is the egui::Context—your entry point to render and manage UI.
// CentralPanel::default().show(...): Creates the central area of the app window.
// Inside the panel:
//     A heading displays "Hello, {name}!"
//     A text input allows the user to edit name.
// Since egui is an immediate mode GUI, the entire UI is redrawn every frame using the current state.
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        info!("painting");
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.image_path = Some(path.display().to_string());

                    // Load and process image
                    if let Ok(img) = image::open(&path) {
                        let mut rgba_img = img.to_rgba8();
                        let (width, height) = rgba_img.dimensions();

                        // Draw a red square (e.g. top-left corner)
                        for y in 0..50.min(height) {
                            for x in 0..50.min(width) {
                                rgba_img.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
                            }
                        }

                        // Convert to egui::ColorImage
                        let color_image = ColorImage::from_rgba_unmultiplied(
                            [width as usize, height as usize],
                            &rgba_img,
                        );

                        // Create a texture
                        self.texture = Some(ctx.load_texture(
                            "edited_image",
                            color_image,
                            egui::TextureOptions::default(),
                        ));
                    }
                }
            }

            if let Some(texture) = &self.texture {
                if let Some(image_path) = &self.image_path {
                    ui.image(texture)
                    .on_hover_text_at_pointer(image_path);
                }
            }
        });
    }
}


// You create options for the app:
//     Enable hardware acceleration (if available).
//     Other window options (like size, decorations) use the default
// This line starts the app:
//     "My Egui App": The window title.
//     options: The NativeOptions you set.
//     Box::new(|_cc| Ok(Box::<MyApp>::default())): A closure that constructs your app. _cc stands for CreationContext, which you’re not using in this case.

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}