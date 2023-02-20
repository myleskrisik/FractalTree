use nannou::{color::rgb_u32, geom::*};
use nannou::{prelude::*};
use nannou_egui::{self, egui, Egui};

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;
const BRANCH_LENGTH: f32 = 20.0;
const DEFAULT_ANGLE: f32 = 60.0;
const BRANCHING_BASE: (f32, f32) = (60.0, 0.);

fn main() {
    nannou::app(model).update(update).run();
}

struct Branch {
    start: Vec2,
    end: Vec2,
}

struct Settings {
    angle: f32,
    length: f32,
}

struct Model {
    branches: Vec<Branch>,
    settings: Settings,
    egui: Egui,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    Model {
        branches: Vec::new(),
        egui,
        settings: Settings {
            angle: DEFAULT_ANGLE,
            length: BRANCH_LENGTH,
        },
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut settings,
        ref mut branches,
        ..
    } = *model;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Workshop window").show(&ctx, |ui| {
        let mut changed = false;
        changed |= ui
            .add(egui::Slider::new(&mut settings.angle, 0.0..=20.0).text("angle"))
            .changed();
        if changed {
            *branches = generate_tree(settings);
        }
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for branch in model.branches.iter() {
        draw.line()
            .start(branch.start)
            .end(branch.end)
            .weight(4.)
            .color(STEELBLUE);
    }

    draw.to_frame(app, &frame).unwrap();

    model.egui.draw_to_frame(&frame).unwrap();
}

fn generate_tree(settings: &mut Settings) -> Vec<Branch> {
    generate_tree_helper(settings, 0, None)
}

fn generate_tree_helper(settings: &mut Settings, levels: u32, base: Option<Vec2>) -> Vec<Branch> { 
    match base {
        Some(base) => base,
        None => Vec2::new(BRANCHING_BASE.0, BRANCHING_BASE. 1),
    }
    Vec::new()

}

fn hsv_from_hex_rgb(color: u32) -> Hsv {
    let color = rgb_u32(color);
    rgba(
        color.red as f32 / 255.0,
        color.green as f32 / 255.0,
        color.blue as f32 / 255.0,
        1.0,
    )
    .into()
}
