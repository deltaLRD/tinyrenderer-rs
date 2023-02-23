extern crate stopwatch;

use eframe::egui;
use egui_extras::RetainedImage;
use image::{imageops::flip_vertical_in_place, ImageBuffer, Rgb};
use stopwatch::Stopwatch;
use tinyrenderer::light::Light;
use tinyrenderer::model::*;
use tinyrenderer::renderer::*;
use tinyrenderer::vec::Vec3;

fn main() {
    let sw = Stopwatch::start_new();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    const imgw: i32 = 400;
    const imgh: i32 = 400;
    let tinyrenderer = TinyRender {
        image_buffer: ImageBuffer::<Rgb<u8>, Vec<u8>>::new(imgw as u32, imgh as u32),
        size: [imgw as usize, imgh as usize],
        image: RetainedImage::from_color_image(
            "rendered image",
            egui::ColorImage::from_rgb(
                [400, 400],
                ImageBuffer::<Rgb<u8>, Vec<u8>>::new(imgw as u32, imgh as u32)
                    .as_flat_samples()
                    .as_slice(),
            ),
        ),
        time_recorder: sw,
        last_time: 0,
        model: Model::from("./assets/obj/african_head.obj").unwrap(),
        z_buffer: vec![vec![f32::MIN;imgw as usize];imgh as usize],
        imgw: imgw,
        imgh: imgh,
    };
    eframe::run_native(
        "LRD's TinyRenderer",
        options,
        Box::new(|_| Box::new(tinyrenderer)),
    )
}

struct TinyRender {
    image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
    image: RetainedImage,
    size: [usize; 2],
    time_recorder: Stopwatch,
    model: Model,
    last_time: i64,
    imgw: i32,
    imgh: i32,
    z_buffer: Vec<Vec<f32>>,
}

impl eframe::App for TinyRender {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // fill the target image buff with black
        self.image_buffer.fill(0u8);

        // define the light
        let lights = vec![Light {
            dir: Vec3 {
                x: 0f32,
                y: 0f32,
                z: -1f32,
            },
            pos: Vec3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
        }];

        // the main rendering function
        draw_model_line(&self.model, &mut self.image_buffer, &lights, &mut self.z_buffer);

        // setup a ui frame to show the render time and save the picture into tga format
        egui::Window::new("LRD's TinyRenderer").show(ctx, |ui| {
            ui.heading("Setting");
            ui.label(format!(
                "frame time:{}ms",
                self.time_recorder.elapsed_ms() - self.last_time
            ));
            self.last_time = self.time_recorder.elapsed_ms();
            if ui.button("save image").clicked() {
                flip_vertical_in_place(&mut self.image_buffer);
                self.image_buffer.save("test.tga").unwrap();
                flip_vertical_in_place(&mut self.image_buffer);
            }
        });

        // show the render result in a ui frame
        egui::Window::new("Image").show(ctx, |ui| {
            flip_vertical_in_place(&mut self.image_buffer);
            self.image = RetainedImage::from_color_image(
                "Rendered Image",
                egui::ColorImage::from_rgb(
                    self.size,
                    self.image_buffer.as_flat_samples().as_slice(),
                ),
            );
            flip_vertical_in_place(&mut self.image_buffer);
            self.image.show(ui);
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("all time: {}ms", self.time_recorder.elapsed_ms())
    }
}
