use eframe::egui;
use egui::{ColorImage, TextureHandle};
use image::flat;
use log::info;
use uni_ocr::{Language, OcrEngine, OcrOptions, OcrProvider};
use tokio::runtime::Runtime;
use tokio::task;
use tokio::runtime::Handle;

struct MyApp {
    image_path: Option<String>,
    texture: Option<TextureHandle>,
    handle: Handle,
}

impl MyApp {
    fn new(handle: Handle) -> Self {
        Self {
            image_path: None,
            texture: None,
            handle,
        }
    }
}



async fn perform_ocr(filepath: String) {
    println!("Filepath: {}", filepath);
    let options = OcrOptions::default()
        .languages(vec![Language::English])
        .confidence_threshold(0.8)
        .timeout(std::time::Duration::from_secs(30));

    let engine = OcrEngine::new(OcrProvider::Auto).expect("Could not get OCR Engine").with_options(options);
    match engine.recognize_file(&filepath).await {
        Ok(text) => {
            let (str1, str2, flt) = text;
            println!("Extracted str1: {}", str1);
            println!("Extracted str2: {}", str2);
            if let Some(flt) = flt {
                println!("Extracted float: {:.2}%", flt);
            } else {
                println!("No float extracted");
            }
        },
        Err(e) => eprintln!("OCR error: {}", e),
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

            if ui.button("Perform OCR").clicked() {
                if let Some(image_path) = &self.image_path {
                    let path_clone = image_path.clone();
                    self.handle.spawn(async move {
                        perform_ocr(path_clone).await;
                    });
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
    env_logger::init();

    // Create the Tokio runtime
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    // Obtain a handle to the runtime
    let handle = rt.handle().clone();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Pass the runtime to your app
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(handle)))),
    )
}