use std::rc::Rc;
use std::cell::RefCell;
use glayout::tree::{TreeNodeRc};
use glayout::canvas::{CanvasContext};
use glayout::canvas::element::{Element, Empty, Image, Text, Transform, ImageLoader};
use glayout::canvas::element::style::{DisplayType, PositionType};

use super::play_audio;
use super::levels;
use levels::LevelData;

const PLAYER: i32 = 9;
const SHE: i32 = 8;
const EMPTY: i32 = 0;
const OCCUPIED: i32 = 1;
const BOX_1: i32 = 2;
const BOX_2_H: i32 = 3;
const BOX_2_V: i32 = 4;
const BOX_4: i32 = 5;

const ATTRACTION_DIST: f64 = 20.;

#[derive(Debug, PartialEq)]
enum LevelStatus {
    Normal,
    Starting,
    Ending,
    Ended,
}

#[derive(Debug)]
struct ObjectInfo {
    mark_type: i32,
    is_player: bool,
    grid: (i32, i32),
    pos: (f64, f64, f64, f64),
    size: (i32, i32),
}

#[derive(Debug, Clone)]
struct AnimationInfo {
    target: usize,
    cur_x: f64,
    cur_y: f64,
    end_x: f64,
    end_y: f64,
    a_x: f64,
    a_y: f64,
    v_x: f64,
    v_y: f64,
}

pub struct LevelController {
    num: i32,
    prev_used_time: f64,
    level_status: LevelStatus,
    wrapper_node: TreeNodeRc<Element>,
    data: LevelData,

    player_index: usize,
    block_size: f64,
    block_size_with_padding: f64,
    main_area: (f64, f64, f64, f64),
    map_node: TreeNodeRc<Element>,

    touch_target: Option<usize>,
    latest_touch_point: (f64, f64),
    ignore_grids: bool,
    objects: Vec<ObjectInfo>,

    animations: Vec<AnimationInfo>,
    me_node: (TreeNodeRc<Element>, TreeNodeRc<Element>),
    me_node_state: i32,
    she_node: Option<(TreeNodeRc<Element>, TreeNodeRc<Element>)>,
    she_node_state: i32,
    she_node_reverted: bool,
    need_check_gravity: bool,
}

impl LevelController {
    pub fn new(num: i32, image_loaders: &Vec<Rc<RefCell<ImageLoader>>>, context: &mut CanvasContext) -> Self {
        let mut root = context.root();
        let canvas_size = context.canvas_size();
        let vertical_mode = canvas_size.1 >= canvas_size.0;
        let cfg = context.canvas_config();
        let data = levels::get_level_data(num).unwrap();
        unsafe { play_audio(data.audio); }

        // basic size
        let block_size_with_padding;
        let block_size;
        let main_area;
        if vertical_mode {
            block_size_with_padding = 630. / data.width as f64;
            block_size = block_size_with_padding - 10.;
            let total_height = block_size_with_padding * data.height as f64 - 10.;
            main_area = (
                50.,
                (1280. - total_height) / 2.,
                block_size_with_padding * data.width as f64 - 10.,
                total_height,
            );
        } else {
            block_size_with_padding = 540. / data.height as f64;
            block_size = block_size_with_padding - 10.;
            let total_width = block_size_with_padding * data.width as f64 - 10.;
            main_area = (
                (1280. - total_width) / 2.,
                70.,
                total_width,
                block_size_with_padding * data.height as f64 - 10.,
            );
        }

        // exit position
        let exit_pos = match data.exit_direction {
            "left" => {
                (-30., data.exit.1 as f64 * block_size_with_padding)
            },
            "right" => {
                (main_area.2 + 10., data.exit.1 as f64 * block_size_with_padding)
            },
            _ => {
                panic!();
            }
        };

        // basic structure
        let elem = element! {
            [&cfg] Empty {
                id: String::from("wrapper");
                font_family: String::from("Muli");
                position: PositionType::Absolute;
                opacity: 0.;
                left: 0.;
                top: 0.;
                width: 1280.;
                height: 720.;
                Empty {
                    position: PositionType::Absolute;
                    left: main_area.0;
                    top: main_area.1;
                    width: main_area.2;
                    height: main_area.3;
                    Empty {
                        position: PositionType::Absolute;
                        left: -30.;
                        top: -30.;
                        width: main_area.2 + 60.;
                        height: main_area.3 + 60.;
                        background_color: (0.6, 0.6, 0.6, 1.);
                    };
                    Empty {
                        position: PositionType::Absolute;
                        left: -10.;
                        top: -10.;
                        width: main_area.2 + 20.;
                        height: main_area.3 + 20.;
                        background_color: (0.1, 0.1, 0.1, 1.);
                    };
                    Empty {
                        position: PositionType::Absolute;
                        left: exit_pos.0;
                        top: exit_pos.1;
                        width: 20.;
                        height: block_size_with_padding + block_size;
                        background_color: (0.1, 0.1, 0.1, 1.);
                    };
                    Empty {
                        position: PositionType::Absolute;
                        left: 0.;
                        top: 0.;
                        width: main_area.2;
                        height: main_area.3;
                        id: String::from("map");
                    };
                };
                Empty {
                    position: PositionType::Absolute;
                    left: main_area.0 - 30.;
                    top: main_area.1 + main_area.3 + 45.;
                    width: 1280.;
                    Text {
                        id: String::from("words");
                        color: (0.5, 0.7, 0.8, 1.);
                        font_size: if vertical_mode { 30. } else { 24. };
                        set_text(data.words);
                    };
                };
            }
        };
        root.append(elem);
        let wrapper_node = context.node_by_id("wrapper").unwrap();
        let mut map_node = context.node_by_id("map").unwrap();

        // parse map and create nodes
        let mut objects = vec![];
        let mut player_index = 0;
        for r in 0..data.height {
            for c in 0..data.width {
                let state = data.map[(r * data.width + c) as usize];
                match state {
                    BOX_1 | BOX_2_H | BOX_2_V | BOX_4 | PLAYER | SHE => {
                        let x = c as f64 * block_size_with_padding;
                        let y = r as f64 * block_size_with_padding;
                        let mut oi = ObjectInfo {
                            mark_type: state,
                            is_player: false,
                            grid: (c, r),
                            size: (1, 1),
                            pos: (x, y, block_size, block_size),
                        };
                        {
                            let pos = &mut oi.pos;
                            let mut node = element! {
                                [&cfg] Empty {
                                    position: PositionType::Absolute;
                                    left: 0.;
                                    top: 0.;
                                    width: block_size;
                                    height: block_size;
                                    background_color: (0.7, 0.7, 0.7, 1.);
                                    transform: *Transform::new().offset(x, y);
                                }
                            };
                            match state {
                                BOX_1 => {
                                    map_node.append(node);
                                },
                                BOX_2_H => {
                                    oi.size.0 = 2;
                                    pos.2 += block_size_with_padding;
                                    node.elem().style_mut().width(block_size_with_padding + block_size);
                                    map_node.append(node);
                                },
                                BOX_2_V => {
                                    oi.size.1 = 2;
                                    pos.3 += block_size_with_padding;
                                    node.elem().style_mut().height(block_size_with_padding + block_size);
                                    map_node.append(node);
                                },
                                BOX_4 => {
                                    oi.size.0 = 2;
                                    oi.size.1 = 2;
                                    pos.2 += block_size_with_padding;
                                    pos.3 += block_size_with_padding;
                                    node.elem().style_mut().width(block_size_with_padding + block_size);
                                    node.elem().style_mut().height(block_size_with_padding + block_size);
                                    map_node.append(node);
                                },
                                PLAYER => {
                                    player_index = objects.len();
                                    oi.is_player = true;
                                    oi.size.0 = 2;
                                    oi.size.1 = 2;
                                    pos.2 += block_size_with_padding;
                                    pos.3 += block_size_with_padding;
                                    node.elem().style_mut().width(block_size_with_padding + block_size);
                                    node.elem().style_mut().height(block_size_with_padding + block_size);
                                    node.elem().style_mut().background_color((0.5, 0.7, 0.8, 1.));
                                    let need_reverse = data.exit_direction == "left";
                                    let image_node_0 = element! {
                                        [&cfg] Image {
                                            id: String::from("me_0");
                                            position: PositionType::Absolute;
                                            left: if need_reverse { block_size_with_padding + block_size } else { 0. };
                                            top: 0.;
                                            width: block_size_with_padding + block_size;
                                            height: block_size_with_padding + block_size;
                                            transform: *Transform::new().scale(if need_reverse { -1. } else { 1. }, 1.);
                                            set_loader(image_loaders[if need_reverse { 5 } else { 1 }].clone());
                                        }
                                    };
                                    let image_node_1 = element! {
                                        [&cfg] Image {
                                            id: String::from("me_1");
                                            display: DisplayType::None;
                                            position: PositionType::Absolute;
                                            left: if need_reverse { block_size_with_padding + block_size } else { 0. };
                                            top: 0.;
                                            width: block_size_with_padding + block_size;
                                            height: block_size_with_padding + block_size;
                                            transform: *Transform::new().scale(if need_reverse { -1. } else { 1. }, 1.);
                                            set_loader(image_loaders[if need_reverse { 5 } else { 2 }].clone());
                                        }
                                    };
                                    node.append(image_node_0);
                                    node.append(image_node_1);
                                    map_node.append(node);
                                },
                                SHE => {
                                    oi.size.0 = 2;
                                    oi.size.1 = 2;
                                    pos.2 += block_size_with_padding;
                                    pos.3 += block_size_with_padding;
                                    node.elem().style_mut().width(block_size_with_padding + block_size);
                                    node.elem().style_mut().height(block_size_with_padding + block_size);
                                    node.elem().style_mut().background_color((0.9, 0.7, 0.7, 1.));
                                    let image_node_0 = element! {
                                        [&cfg] Image {
                                            id: String::from("she_0");
                                            position: PositionType::Absolute;
                                            left: 0.;
                                            top: 0.;
                                            width: block_size_with_padding + block_size;
                                            height: block_size_with_padding + block_size;
                                            set_loader(image_loaders[3].clone());
                                        }
                                    };
                                    let image_node_1 = element! {
                                        [&cfg] Image {
                                            id: String::from("she_1");
                                            display: DisplayType::None;
                                            position: PositionType::Absolute;
                                            left: 0.;
                                            top: 0.;
                                            width: block_size_with_padding + block_size;
                                            height: block_size_with_padding + block_size;
                                            set_loader(image_loaders[4].clone());
                                        }
                                    };
                                    node.append(image_node_0);
                                    node.append(image_node_1);
                                    map_node.append(node);
                                },
                                _ => { }
                            }
                        }
                        objects.push(oi);
                    },
                    _ => { }
                }
            }
        }

        Self {
            num,
            prev_used_time: 0.,
            level_status: LevelStatus::Starting,
            wrapper_node,
            data,

            player_index,
            block_size,
            block_size_with_padding,
            main_area,
            map_node,

            touch_target: None,
            latest_touch_point: (0., 0.),
            ignore_grids: true,
            objects,

            animations: vec![],
            me_node: (context.node_by_id("me_0").unwrap(), context.node_by_id("me_1").unwrap()),
            me_node_state: 0,
            she_node: match context.node_by_id("she_0") {
                Some(_) => {
                    Some((context.node_by_id("she_0").unwrap(), context.node_by_id("she_1").unwrap()))
                },
                None => None
            },
            she_node_state: 0,
            she_node_reverted: false,
            need_check_gravity: true,
        }
    }

    pub fn frame(&mut self, context: &mut CanvasContext, used_time: f64) -> i32 {
        let mut frame_length = used_time - self.prev_used_time;
        self.prev_used_time = used_time;

        if self.need_check_gravity {
            self.need_check_gravity = false;
            self.check_gravity_moves();
        }

        while frame_length > 0. {

            // level animation
            match self.level_status {
                LevelStatus::Starting => {
                    let mut style = self.wrapper_node.elem().style_mut();
                    let mut o = style.get_opacity() + 0.02;
                    if o >= 1. {
                        o = 1.;
                        self.level_status = LevelStatus::Normal;
                    }
                    style.opacity(o);
                },
                LevelStatus::Ending => {
                    let mut style = self.wrapper_node.elem().style_mut();
                    let mut o = style.get_opacity() - 0.01;
                    if o < 0. {
                        o = 0.;
                        self.level_status = LevelStatus::Ended;
                    };
                    style.opacity(o);
                },
                _ => { }
            }

            // block animations
            for ani in self.animations.iter_mut() {
                ani.v_x += ani.a_x;
                ani.v_y += ani.a_y;
                if (ani.cur_x - ani.end_x) * (ani.cur_x + ani.v_x - ani.end_x) <= 0. {
                    ani.cur_x = ani.end_x;
                } else {
                    ani.cur_x += ani.v_x;
                }
                if (ani.cur_y - ani.end_y) * (ani.cur_y + ani.v_y - ani.end_y) <= 0. {
                    ani.cur_y = ani.end_y;
                } else {
                    ani.cur_y += ani.v_y;
                }
                let child = self.map_node.child(ani.target);
                child.elem().style_mut().transform_mut().reset().offset(ani.cur_x, ani.cur_y);
            }
            self.animations = self.animations.clone().into_iter().filter(|x| { x.cur_x != x.end_x || x.cur_y != x.end_y }).collect();

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

            // she animation
            match self.she_node {
                Some(ref mut she_node) => {
                    if used_time as i32 % 5000 > 4200 {
                        if self.she_node_state == 0 {
                            self.she_node_state = 1;
                            she_node.1.elem().style_mut().display(DisplayType::Block);
                            she_node.0.elem().style_mut().display(DisplayType::None);
                        }
                    } else {
                        if self.she_node_state == 1 {
                            self.she_node_state = 0;
                            she_node.0.elem().style_mut().display(DisplayType::Block);
                            she_node.1.elem().style_mut().display(DisplayType::None);
                        }
                    }
                    let she_node_p = she_node.0.parent().unwrap();
                    let she_x = she_node_p.elem().style().transform_ref().get_offset().0;
                    let me_node_p = self.me_node.0.parent().unwrap();
                    let me_x = me_node_p.elem().style().transform_ref().get_offset().0;
                    if she_x > me_x && !self.she_node_reverted {
                        self.she_node_reverted = true;
                        she_node_p.elem().style_mut().transform_mut().offset(self.block_size + self.block_size_with_padding, 0.).reset_scale(-1., 1.);
                    } else if she_x < me_x && self.she_node_reverted {
                        self.she_node_reverted = false;
                        she_node_p.elem().style_mut().transform_mut().offset(- (self.block_size + self.block_size_with_padding), 0.).reset_scale(1., 1.);
                    }
                },
                None => { }
            }

            frame_length -= 25.;
        }

        // keyboard tricks
        let key_code = context.fetch_last_key_code();
        match key_code {
            78 => {
                context.root().remove(0);
                return self.num + 1;
            },
            80 => {
                context.root().remove(0);
                return self.num - 1;
            },
            82 => {
                context.root().remove(0);
                return self.num;
            },
            _ => { }
        }

        if self.animations.len() == 0 {
            if self.level_status == LevelStatus::Normal {
                self.check_level_end();
            }
            if self.level_status == LevelStatus::Ended {
                context.root().remove(0);
                return self.num + 1;
            }
            self.handle_touch(context);
        }

        return -1;
    }

    fn check_level_end(&mut self) {
        let i = self.player_index;
        if self.objects[i].grid == self.data.exit {
            self.level_status = LevelStatus::Ending;
            let is_left = self.data.exit_direction == "left";
            let ai = AnimationInfo {
                target: i,
                cur_x: self.objects[i].pos.0,
                cur_y: self.objects[i].pos.1,
                end_x: self.objects[i].pos.0 + if is_left { -200. } else { 200. },
                end_y: self.objects[i].pos.1,
                a_x: 0.,
                a_y: 0.,
                v_x: 2. * if is_left { -1. } else { 1. },
                v_y: 0.,
            };
            self.animations.push(ai);
        }
    }

    fn handle_touch(&mut self, context: &mut CanvasContext) {
        if context.touching() {
            match self.touch_target {
                None => {
                    // start move
                    let root = context.root();
                    let p = context.touch_point();
                    let s = root.elem().style();
                    let t = s.transform_ref();
                    let new_touch_point = ((p.0 - t.get_offset().0) / t.get_scale().0, (p.1 - t.get_offset().1) / t.get_scale().1);
                    let touch_x = new_touch_point.0 - self.main_area.0;
                    let touch_y = new_touch_point.1 - self.main_area.1;
                    for i in 0..self.objects.len() {
                        if self.objects[i].mark_type == SHE {
                            continue;
                        }
                        let pos = self.objects[i].pos;
                        if touch_x < pos.0 || touch_x >= pos.0 + pos.2 || touch_y < pos.1 || touch_y >= pos.1 + pos.3 {
                            /* empty */
                        } else {
                            self.touch_target = Some(i);
                            self.latest_touch_point = new_touch_point;
                            self.ignore_grids = true;
                            self.remove_mark_on_map(i);
                            break;
                        }
                    }
                },
                Some(touch_target) => {
                    // continue move
                    let root = context.root();
                    let p = context.touch_point();
                    let s = root.elem().style();
                    let t = s.transform_ref();
                    let new_touch_point = ((p.0 - t.get_offset().0) / t.get_scale().0, (p.1 - t.get_offset().1) / t.get_scale().1);
                    let moving_delta = (
                        new_touch_point.0 - self.latest_touch_point.0,
                        new_touch_point.1 - self.latest_touch_point.1,
                    );
                    self.latest_touch_point = new_touch_point;

                    // find legal moving pos
                    let touch_node = self.map_node.child(touch_target);
                    let old_pos = self.objects[touch_target].pos;
                    let (x, y) = self.normalize_position(&self.objects[touch_target], (old_pos.0 + moving_delta.0, old_pos.1 + moving_delta.1), (old_pos.0, old_pos.1));
                    let (grid_x, grid_y, in_grid) = self.match_grids((x, y), ATTRACTION_DIST);
                    touch_node.elem().style_mut().transform_mut().reset().offset(grid_x, grid_y);

                    {
                        let pos = &mut self.objects[touch_target].pos;
                        // release if pointer is not in object now
                        // let touch_x = new_touch_point.0 - self.main_area.0;
                        // let touch_y = new_touch_point.1 - self.main_area.1;
                        // if touch_x < pos.0 || touch_x >= pos.0 + pos.2 || touch_y < pos.1 || touch_y >= pos.1 + pos.3 {
                        //     self.touch_target = None;
                        //     // TODO
                        // }
                        pos.0 = x;
                        pos.1 = y;
                    }

                    // do gravity moves
                    if in_grid {
                        if !self.ignore_grids {
                            let (c, r) = self.nearest_grid(x, y);
                            self.objects[touch_target].grid = (c as i32, r as i32);
                            self.mark_on_map(touch_target);
                            self.check_gravity_moves();
                            self.remove_mark_on_map(touch_target);
                        }
                        self.ignore_grids = true;
                    } else {
                        self.ignore_grids = false;
                    }
                }
            }
        } else {
            match self.touch_target {
                Some(touch_target) => {
                    self.touch_target = None;
                    let (c, r) = self.nearest_grid(self.objects[touch_target].pos.0, self.objects[touch_target].pos.1);
                    self.objects[touch_target].grid = (c as i32, r as i32);
                    self.match_pos_for_grid(touch_target);
                    // TODO better animation: x-axis; from cur y
                    self.mark_on_map(touch_target);
                    self.check_gravity_moves();
                },
                None => { }
            }
        }
    }

    fn check_gravity_moves(&mut self) {
        // record original r
        let target_grid_ys: Vec<i32> = self.objects.iter().map(|oi| {
            oi.grid.1
        }).collect();

        // loop
        let mut need_loop = true;
        while need_loop {
            need_loop = false;
            for i in 0..self.objects.len() {
                if self.objects[i].mark_type == SHE { continue; }
                let mut has_space = false;
                {
                    let oi = &mut self.objects[i];
                    let r = oi.grid.1 + oi.size.1;
                    if r < self.data.height {
                        has_space = true;
                        for c in oi.grid.0..(oi.grid.0 + oi.size.0) {
                            if self.data.map[(r * self.data.width + c) as usize] > 0 {
                                has_space = false;
                                break;
                            }
                        }
                    }
                }
                if has_space {
                    self.remove_mark_on_map(i);
                    let r = self.objects[i].grid.1;
                    self.objects[i].grid.1 = r + 1;
                    self.match_pos_for_grid(i);
                    self.mark_on_map(i);
                    need_loop = true;
                }
            }
        }

        // revert changes to touch_target
        let mut touch_target_moved = false;
        match self.touch_target {
            Some(touch_target) => {
                for r in (target_grid_ys[touch_target]..self.objects[touch_target].grid.1).rev() {
                    let mut has_space = true;
                    {
                        let oi = &mut self.objects[touch_target];
                        for c in oi.grid.0..(oi.grid.0 + oi.size.0) {
                            if self.data.map[(r * self.data.width + c) as usize] > 0 {
                                has_space = false;
                                break;
                            }
                        }
                    }
                    if has_space {
                        self.remove_mark_on_map(touch_target);
                        let r = self.objects[touch_target].grid.1;
                        self.objects[touch_target].grid.1 = r - 1;
                        self.match_pos_for_grid(touch_target);
                        self.mark_on_map(touch_target);
                    } else {
                        touch_target_moved = true;
                        break;
                    }
                }
            },
            None => { }
        }

        self.generate_ani(!touch_target_moved);
    }

    fn generate_ani(&mut self, ignore_touch_target: bool) {
        // do animation
        for i in 0..self.objects.len() {
            if self.objects[i].mark_type == SHE { continue; }
            if ignore_touch_target && self.touch_target.is_some() && i == self.touch_target.unwrap() {
                continue;
            }
            let child = self.map_node.child(i);
            let ori_pos = child.elem().style().transform_ref().get_offset();
            if ori_pos.0 != self.objects[i].pos.0 || ori_pos.1 != self.objects[i].pos.1 {
                let ai = AnimationInfo {
                    target: i,
                    cur_x: ori_pos.0,
                    cur_y: ori_pos.1,
                    end_x: self.objects[i].pos.0,
                    end_y: self.objects[i].pos.1,
                    a_x: 10. * if self.objects[i].pos.0 > ori_pos.0 { 1. } else { -1. },
                    a_y: 7. * if self.objects[i].pos.1 > ori_pos.1 { 1. } else { -1. },
                    v_x: 0.,
                    v_y: 0.,
                };
                self.animations.push(ai);
            }
        }
    }

    #[inline]
    fn match_pos_for_grid(&mut self, i: usize) {
        let s = self.block_size_with_padding;
        let oi = &mut self.objects[i];
        oi.pos.0 = oi.grid.0 as f64 * s;
        oi.pos.1 = oi.grid.1 as f64 * s;
    }

    fn mark_on_map(&mut self, touch_target: usize) {
        let oi = &self.objects[touch_target];
        for r in (oi.grid.1)..(oi.grid.1 + oi.size.1) {
            for c in (oi.grid.0)..(oi.grid.0 + oi.size.0) {
                self.data.map[(r * self.data.width + c) as usize] = 1;
            }
        }
        self.data.map[(oi.grid.1 * self.data.width + oi.grid.0) as usize] = oi.mark_type;
    }
    fn remove_mark_on_map(&mut self, touch_target: usize) {
        let oi = &self.objects[touch_target];
        for r in (oi.grid.1)..(oi.grid.1 + oi.size.1) {
            for c in (oi.grid.0)..(oi.grid.0 + oi.size.0) {
                self.data.map[(r * self.data.width + c) as usize] = 0;
            }
        }
    }

    fn normalize_position(&self, oi: &ObjectInfo, (mut x, mut y): (f64, f64), (old_x, old_y): (f64, f64)) -> (f64, f64) {
        if !self.is_grid_empty(oi, x, y) {
            let (c, r) = self.nearest_grid(old_x, old_y);
            if (old_x - x).abs() > (old_y - y).abs() {
                if self.is_grid_empty(oi, x, r * self.block_size_with_padding) {
                    y = r * self.block_size_with_padding;
                } else if self.is_grid_empty(oi, c * self.block_size_with_padding, y) {
                    x = c * self.block_size_with_padding;
                } else {
                    x = c * self.block_size_with_padding;
                    y = r * self.block_size_with_padding;
                }
            } else {
                if self.is_grid_empty(oi, c * self.block_size_with_padding, y) {
                    x = c * self.block_size_with_padding;
                } else if self.is_grid_empty(oi, x, r * self.block_size_with_padding) {
                    y = r * self.block_size_with_padding;
                } else {
                    x = c * self.block_size_with_padding;
                    y = r * self.block_size_with_padding;
                }
            }
        }
        (x, y)
    }
    fn match_grids(&self, (x, y): (f64, f64), dist: f64) -> (f64, f64, bool) {
        let (c, r) = self.nearest_grid(x, y);
        return if (c * self.block_size_with_padding - x).abs() < dist && (r * self.block_size_with_padding - y).abs() < dist {
            (c * self.block_size_with_padding, r * self.block_size_with_padding, true)
        } else {
            (x, y, false)
        }
    }
    fn nearest_grid(&self, x: f64, y: f64) -> (f64, f64) {
        let c = (x / self.block_size_with_padding).round();
        let r = (y / self.block_size_with_padding).round();
        (c, r)
    }
    fn is_grid_empty(&self, oi: &ObjectInfo, x: f64, y: f64) -> bool {
        let min_c = (x / self.block_size_with_padding + 1e-4).floor() as i32;
        let min_r = (y / self.block_size_with_padding + 1e-4).floor() as i32;
        let max_c = (x / self.block_size_with_padding - 1e-4).ceil() as i32 + oi.size.0;
        let max_r = (y / self.block_size_with_padding - 1e-4).ceil() as i32 + oi.size.1;
        if min_c < 0 || max_c > self.data.width || min_r < 0 || max_r > self.data.height {
            return false;
        }
        for r in min_r..max_r {
            for c in min_c..max_c {
                if self.data.map[(r * self.data.width + c) as usize] != 0 {
                    return false;
                }
            }
        }
        return true;
    }

    fn debug_map(&self) {
        println!("Map state:");
        for r in 0..self.data.height {
            for c in 0..self.data.width {
                print!("{} ", self.data.map[(r * self.data.width + c) as usize]);
            }
            print!("\n");
        }
        println!("Object state:");
        for oi in self.objects.iter() {
            println!("{:?}", oi);
        }
    }
}



