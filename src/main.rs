use std::{env, path::Path};
use image::{Rgb};
use eframe::egui;
use egui_extras;

fn main() -> eframe::Result{
    dbg!(env::args().nth(0));

    let image_path: String = env::args().nth(1).expect("No file path given");

    if !Path::new(&image_path).exists(){
        panic!("File provided does not exisit");
    }

    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Minecraft Colour Mapper",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp{
    colour_channels: Vec<[f32; 3]>
}

impl Default for MyApp {
    fn default() -> Self {
        let mut app = MyApp { 
            colour_channels: Vec::default()
        };
        for i in 0..5 {
            println!("Adding color for channel {}", i);
            app.colour_channels.push([0.0, 0.0, 0.0]);
        };

        return  app;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            egui_extras::install_image_loaders(ctx);
            egui::Grid::new("main_window").show(ui, |ui|{
                editor_display(self, ui);
                image_display(ui);
            })
        });
    }
}

fn editor_display(app: &mut MyApp, ui: &mut egui::Ui){
    egui::Grid::new("editor_window").show(ui,|ui| {
        ui.label("Color 1");
        ui.color_edit_button_rgb(app.colour_channels.get_mut(0).unwrap());
        ui.end_row();
        ui.label("Color 2");
        ui.color_edit_button_rgb(app.colour_channels.get_mut(1).unwrap());
        ui.end_row();
        ui.label("Color 3");
        ui.color_edit_button_rgb(app.colour_channels.get_mut(2).unwrap());
        ui.end_row();
        ui.label("Color 4");
        ui.color_edit_button_rgb(app.colour_channels.get_mut(3).unwrap());
        ui.end_row();
        ui.label("Color 5");
        ui.color_edit_button_rgb(app.colour_channels.get_mut(4).unwrap());
    });
}

fn image_display(ui: &mut egui::Ui){
    ui.add(egui::Image::new(egui::include_image!("../Sample.png")));
}

// fn colour_channel_selection();