use std::rc::Rc;
use std::cell::RefCell;
use glayout::frame;
use glayout::canvas::{Canvas, CanvasContext};

use super::level;

struct MainLoop {
    _canvas: Canvas,
    ctx: Rc<RefCell<CanvasContext>>,
    rel_time: f64,
    level_status: i32,
    level_controller: Option<level::LevelController>,
}

impl frame::Frame for MainLoop {
    fn frame(&mut self, timestamp: f64) -> bool {
        let mut context = self.ctx.borrow_mut();

        if self.level_status < 0 {
            // game just started
            self.level_status = 0;
        }
        if self.level_controller.is_none() {
            self.rel_time = timestamp;
            self.level_controller = Some(level::LevelController::new(self.level_status, &mut context));
        }
        let frame_ret = self.level_controller.as_mut().unwrap().frame(&mut context, timestamp - self.rel_time);
        if frame_ret >= 0 {
            self.level_status = frame_ret;
            self.rel_time = timestamp;
            self.level_controller = Some(level::LevelController::new(self.level_status, &mut context));
        }

        // force redraw every frame
        context.redraw();
        return true;
    }
}

pub fn init() {
    let mut canvas = Canvas::new(0);
    let ctx = canvas.context().clone();

    canvas.ctx(|context| {
        let pixel_ratio = context.device_pixel_ratio();
        context.set_canvas_size(1280, 720, pixel_ratio);
        context.set_clear_color(0.25, 0.25, 0.25, 1.);
    });

    frame::bind(Rc::new(RefCell::new(MainLoop {
        _canvas: canvas,
        ctx,
        rel_time: 0.,
        level_status: -1,
        level_controller: None,
    })), frame::FramePriority::Normal);
}
