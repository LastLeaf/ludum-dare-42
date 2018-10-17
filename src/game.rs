use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;
use glayout::frame;
use glayout::canvas::{Canvas, CanvasContext};
use glayout::canvas::element::{Element, Empty, Image, Text, Transform, ImageLoader};
use glayout::canvas::element::style::{DisplayType, PositionType};

use super::{cover, level, levels};

const FORCE_WAIT_TIME: f64 = 5000.;

const LOADING: i32 = -1;
const REPLAY: i32 = -2;
const PLAY: i32 = -3;

struct MainLoop {
    _canvas: Canvas,
    ctx: Rc<RefCell<CanvasContext>>,
    rel_time: f64,
    level_status: i32,
    level_controller: Option<level::LevelController>,
    cover_controller: Option<cover::CoverController>,
    image_loaders: Vec<Rc<RefCell<ImageLoader>>>,
}

impl frame::Frame for MainLoop {
    fn frame(&mut self, timestamp: f64) -> bool {
        let mut context = self.ctx.borrow_mut();
        let canvas_size = context.canvas_size();
        let vertical_mode = canvas_size.1 >= canvas_size.0;

        if self.level_status == LOADING {
            // initial interface
            if self.rel_time == 0. {
                self.rel_time = timestamp;
                let cfg = context.canvas_config();
                let node = element! {
                    [&cfg] Text {
                        font_family: String::from("sans-serif");
                        position: PositionType::Absolute;
                        left: if vertical_mode { 120. } else { 400. };
                        top: if vertical_mode { 900. } else { 600. };
                        font_size: 24.;
                        color: (0.6, 0.6, 0.6, 1.);
                        set_text("Loading...");
                    }
                };
                context.root().append(node);
            }
            // check loading status
            let mut is_loading = timestamp - self.rel_time < FORCE_WAIT_TIME;
            for loader in self.image_loaders.iter() {
                if loader.borrow().is_loading() {
                    is_loading = true;
                }
            }
            if !is_loading {
                context.root().remove(0);
                self.level_status = PLAY;
            }
            return true;
        }

        let frame_ret = if self.level_status < 0 {
            if self.cover_controller.is_none() {
                self.rel_time = timestamp;
                self.cover_controller = Some(cover::CoverController::new(self.level_status == REPLAY, &self.image_loaders, &mut context));
            }
            self.cover_controller.as_mut().unwrap().frame(&mut context, timestamp - self.rel_time)
        } else {
            self.level_controller.as_mut().unwrap().frame(&mut context, timestamp - self.rel_time)
        };
        if frame_ret != LOADING {
            self.cover_controller = None;
            self.level_controller = None;
            if levels::get_level_data(frame_ret).is_some() {
                // requested next level
                self.level_status = frame_ret;
                self.rel_time = timestamp;
                self.level_controller = Some(level::LevelController::new(self.level_status, &self.image_loaders, &mut context));
            } else {
                // return to cover
                self.level_status = REPLAY;
                self.level_controller = None;
            }
        }

        // force redraw every frame
        context.redraw();
        return true;
    }
}

fn start_loading(context: &mut CanvasContext) -> Vec<Rc<RefCell<ImageLoader>>> {
    vec![
        "resources/me.png",
        "resources/me.png",
        "resources/me_1.png",
        "resources/she.png",
        "resources/she_1.png",
        "resources/me_old.png",
    ].iter().map(|s| {
        let il_rc = Rc::new(RefCell::new(ImageLoader::new_with_canvas_config(context.canvas_config())));
        il_rc.borrow_mut().set_id(String::from(*s));
        ImageLoader::load(il_rc.clone(), *s);
        il_rc
    }).collect()
}

pub fn init() {
    let mut canvas = Canvas::new(0);
    let ctx = canvas.context().clone();

    canvas.ctx(|context| {
        let pixel_ratio = context.device_pixel_ratio();
        let (w, h) = context.window_size();
        context.set_canvas_size(w, h, pixel_ratio);
        context.set_clear_color(0.25, 0.25, 0.25, 1.);
        let vertical_mode = h >= w;
        let scale = (1 as f64).min(
            if vertical_mode {
                (w as f64 / 720.).min(h as f64 / 1280.)
            } else {
                (w as f64 / 1280.).min(h as f64 / 720.)
            }
        );
        let (offset_w, offset_h) = if vertical_mode {
            ((w as f64 - 720. * scale) / 2., (h as f64 - 1280. * scale) / 2.)
        } else {
            ((w as f64 - 1280. * scale) / 2., (h as f64 - 720. * scale) / 2.)
        };
        println!("Canvas size ({}, {}) offset ({}, {}) scale {} vertical {:?}", w, h, offset_w, offset_h, scale, vertical_mode);
        let root = context.root();
        root.elem().style_mut().transform_mut().offset(offset_w, offset_h).scale(scale, scale);
    });

    let context = canvas.context();
    let image_loaders = start_loading(&mut context.borrow_mut());

    let ml = Rc::new(RefCell::new(MainLoop {
        _canvas: canvas,
        ctx,
        rel_time: 0.,
        level_status: LOADING,
        level_controller: None,
        cover_controller: None,
        image_loaders,
    }));
    frame::bind(ml, frame::FramePriority::Normal);
}
