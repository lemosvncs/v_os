use eframe::glow::Context;
use egui::{Vec2, InnerResponse, Window, Frame};
use egui_extras::{RetainedImage, image::load_image_bytes};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    logo_size: Vec2,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    vos: RetainedImage,
    
    #[serde(skip)]
    bg: RetainedImage,
}

// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
// struct Vapp {
//     #[serde(skip)]
//     icon: RetainedImage,
    
//     // #[serde(skip)]
//     label: String,
//     size: Vec2,

// }

//  Default desktop app
// impl Default for Vapp {
//     fn default() -> Self {
//         Self {
//             icon: RetainedImage::from_image_bytes(
//                 "vos.png", 
//                 include_bytes!("../assets/vos.png"),
//             ).unwrap(),
//             label: "App.vex".to_string(),
//             size: Vec2::new(50.0, 50.0), 
//         }
//     }
// }

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            logo_size: Vec2::new(50.0, 50.0),
            vos: RetainedImage::from_image_bytes(
                "vos.png", 
                include_bytes!("../assets/vos.png"),
            ).unwrap(),
            bg: RetainedImage::from_image_bytes(
                "bg.png", 
                include_bytes!("../assets/bg.png"),
            ).unwrap(),
        }
    }
}

// impl Vapp {
//     fn desktopButton(self, ctx: &egui::Context) -> Option<InnerResponse<Option<Window>>> {
//         let desktopButton = 
//             egui::Window::new("desktopVapp").show(ctx, |ui| {
//                 egui::widgets::ImageButton::new(self.icon.texture_id(&ctx), self.size);
//                 ui.label(self.label);
//             });
//         return desktopButton;
//     }

//     fn label(mut self, label: String) -> Vapp{
//         self.label = label;
//         return self;
//     }
// }

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { logo_size: _,  vos: _, bg:_,  } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        // let size = eframe::Frame::info(&self).window_info.size;
        // let size = eframe::Frame::info(&_frame);
        println!("{:?}", &_frame.info());
        // _frame.info();

        egui::CentralPanel::default()
            // .frame(egui::Frame::)
            .show(ctx, |ui| {
            // ui.label(format!("{:?}", &_frame.info()));
            // ui.label("A");

            ui.add(egui::widgets::Image::new(self.bg.texture_id(ctx), ctx.available_rect().size()));
            egui::Grid::new("desktop").show(ui, |ui| {
                // let d_one = Vapp::default();

                egui::Window::new("Contato.vex")
                    .frame(Frame::none())
                    .resizable(false)
                    .collapsible(false)
                    .title_bar(false)
                    .show(ctx, |ui| {
                    ui.add(egui::ImageButton::new(self.vos.texture_id(ctx), Vec2::new(50.0, 50.0)));
                    ui.label("Contato.vex");
                });
                // ui.end_row();                
                // ui.add(egui::Image::new(self.vos.texture_id(ctx), self.logo_size));
                // ui.end_row();                
                // ui.add(egui::Image::new(self.vos.texture_id(ctx), self.logo_size));
                // ui.end_row();                
                // ui.add(egui::Image::new(self.vos.texture_id(ctx), self.logo_size));
                // ui.end_row();                
                // ui.add(egui::Image::new(self.vos.texture_id(ctx), self.logo_size));
                // ui.end_row();                
                // ui.add(egui::Image::new(self.vos.texture_id(ctx), self.logo_size));


            });
            

            egui::warn_if_debug_build(ui);
        });

        egui::TopBottomPanel::bottom("menu_de_baixo").min_height(50.0).show(ctx, |ui| {
            egui::Grid::new("menu_de_baixo_grid").show(ui, |ui| {
                // ui.label("text");
                if ui.add(egui::widgets::ImageButton::new(self.vos.texture_id(ctx), Vec2::new(50.0, 50.0))).clicked() {
                    ui.label("...");
                }
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
