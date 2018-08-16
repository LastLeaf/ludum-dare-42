use std::rc::Rc;
use std::cell::RefCell;
use glayout::tree::{TreeNodeRc};
use glayout::canvas::{CanvasContext};
use glayout::canvas::element::{Element, Empty, Image, Text, Transform, ImageLoader};
use glayout::canvas::element::style::{DisplayType, PositionType};

pub struct CoverController {
    wrapper_node: TreeNodeRc<Element>,
    play_wrapper_node: TreeNodeRc<Element>,

    me_node: (TreeNodeRc<Element>, TreeNodeRc<Element>),
    me_node_state: i32,
}

impl CoverController {
    pub fn new(is_replay: bool, image_loaders: &Vec<Rc<RefCell<ImageLoader>>>, context: &mut CanvasContext) -> Self {
        let mut root = context.root();
        let canvas_size = context.canvas_size();
        let vertical_mode = canvas_size.1 >= canvas_size.0;
        let cfg = context.canvas_config();

        // basic structure
        let elem = element! {
            [&cfg] Empty {
                id: String::from("wrapper");
                font_family: String::from("Muli");
                position: PositionType::Absolute;
                opacity: 1.;
                left: if vertical_mode { 120. } else { 400. };
                top: if vertical_mode { 280. } else { 0. };
                width: 1280.;
                height: 720.;
                Text {
                    position: PositionType::Absolute;
                    left: 0.;
                    top: 680.;
                    height: 24.;
                    font_size: 16.;
                    color: (0.6, 0.6, 0.6, 1.);
                    set_text("A game for Ludum Dare 42 by LastLeaf");
                };
                Image {
                    id: String::from("me_0");
                    position: PositionType::Absolute;
                    left: 0.;
                    top: 150.;
                    width: 300.;
                    height: 300.;
                    set_loader(image_loaders[if is_replay { 5 } else { 1 }].clone());
                };
                Image {
                    id: String::from("me_1");
                    display: DisplayType::None;
                    position: PositionType::Absolute;
                    left: 0.;
                    top: 150.;
                    width: 300.;
                    height: 300.;
                    set_loader(image_loaders[if is_replay { 5 } else { 2 }].clone());
                };
                Text {
                    position: PositionType::Absolute;
                    left: 0.;
                    top: 500.;
                    height: 300.;
                    font_size: 36.;
                    set_text("Leaving Room");
                };
                Empty {
                    id: String::from("play_wrapper");
                    position: PositionType::Absolute;
                    left: 0.;
                    top: 560.;
                    width: 150.;
                    height: 40.;
                    color: (0.5, 0.7, 0.8, 1.);
                    background_color: (0.4, 0.4, 0.4, 1.);
                    Text {
                        position: PositionType::Absolute;
                        left: 20.;
                        top: 2.;
                        font_size: 24.;
                        set_text(if is_replay { "Replay >" } else { "Play >" });
                    };
                };
            }
        };
        root.append(elem);
        let wrapper_node = context.node_by_id("wrapper").unwrap();
        let play_wrapper_node = context.node_by_id("play_wrapper").unwrap();
        let me_node = (context.node_by_id("me_0").unwrap(), context.node_by_id("me_1").unwrap());

        Self {
            wrapper_node,
            play_wrapper_node,

            me_node,
            me_node_state: 0,
        }
    }

    pub fn frame(&mut self, context: &mut CanvasContext, used_time: f64) -> i32 {
        // fade in
        {
            let mut style = self.wrapper_node.elem().style_mut();
            let mut o = style.get_opacity() + 0.01;
            if o >= 1. {
                o = 1.;
            }
            style.opacity(o);
        }

        // touch handling
        if context.touching() {
            let p = context.touch_point();
            let n = context.root().elem().node_under_point(p);
            match n {
                Some(ref n) => {
                    if TreeNodeRc::ptr_eq(n, &self.play_wrapper_node) || TreeNodeRc::ptr_eq(n, &self.play_wrapper_node.parent().unwrap()) {
                        context.root().remove(0);
                        return 0;
                    }
                },
                _ => { }
            }
        }

        // me animation
        if used_time as i32 % 4000 > 3600 {
            if self.me_node_state == 0 {
                self.me_node_state = 1;
                self.me_node.1.elem().style_mut().display(DisplayType::Block);
                self.me_node.0.elem().style_mut().display(DisplayType::None);
            }
        } else {
            if self.me_node_state == 1 {
                self.me_node_state = 0;
                self.me_node.0.elem().style_mut().display(DisplayType::Block);
                self.me_node.1.elem().style_mut().display(DisplayType::None);
            }
        }

        return -1;
    }
}
