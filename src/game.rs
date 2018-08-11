use std::rc::Rc;
use std::cell::RefCell;
use glayout::frame;
use glayout::canvas::{Canvas, CanvasContext};
use glayout::canvas::element::{Element, Empty, Image, Text};
use glayout::canvas::element::style::{DisplayType, PositionType};

struct MainLoop {
    _canvas: Canvas,
    ctx: Rc<RefCell<CanvasContext>>,
}

impl frame::Frame for MainLoop {
    fn frame(&mut self, timestamp: f64) -> bool {
        let mut context = self.ctx.borrow_mut();
        let root = context.root();

        if context.touching() {
            match root.elem().node_under_point(context.touch_point()) {
                Some(x) => {
                    println!("Touching: {:?}", x.elem().style().get_id());
                },
                None => {
                    println!("Touching nothing");
                }
            }
        }

        let f = context.node_by_id("f").unwrap();
        f.elem().style_mut().transform_mut().reset().offset(timestamp / 1000. % 4. * 400., 0.);
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
        context.set_clear_color(0., 0., 0., 1.);

        let elem = {
            let cfg = context.canvas_config();
            let elem = element! {
                [&cfg] Empty {
                    font_family: String::from("\"Muli\"");
                    Text {
                        id: String::from("a");
                        position: PositionType::Absolute;
                        left: 10.;
                        top: 10.;
                        width: 50.;
                        set_text("Absolute Positioning");
                    };
                    color: (0., 0., 1., 0.5);
                    Empty {
                        id: String::from("b");
                        display: DisplayType::Block;
                        position: PositionType::Absolute;
                        top: 100.;
                        left: 200.;
                        Text {
                            id: String::from("c");
                            font_size: 24.;
                            set_text("LARGE TEXT");
                        };
                        Image {
                            id: String::from("d");
                            width: 400.;
                            height: 400.;
                            load("resources/test.png");
                        };
                    };
                    Empty {
                        id: String::from("e");
                        position: PositionType::Absolute;
                        top: 100.;
                        left: 200.;
                        Text {
                            id: String::from("f");
                            font_size: 16.;
                            set_text("hahaha");
                        };
                    };
                }
            };
            elem
        };
        let mut root_elem = context.root();
        root_elem.append(elem);
    });

    frame::bind(Rc::new(RefCell::new(MainLoop {
        _canvas: canvas,
        ctx,
    })), frame::FramePriority::Normal);
}
