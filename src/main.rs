use image::RgbImage;
use log::info;
use softbuffer::Surface;
use std::collections::HashSet;
use std::num::NonZeroU32;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalSize, Size};
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey, SmolStr};
use winit::window::{Fullscreen, Window, WindowId};

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;
const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const TEX_WIDTH: u32 = 64;
const TEX_HEIGHT: u32 = 64;
static WORLD_MAP: [[u8; MAP_HEIGHT]; MAP_WIDTH] = [
    [
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 7, 7, 7, 7, 7, 7, 7, 7,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 4, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 0, 7, 7, 7, 7, 7,
    ],
    [
        4, 0, 5, 0, 0, 0, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 7, 0, 0, 0, 7, 7, 7, 1,
    ],
    [
        4, 0, 6, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 0, 0, 0, 8,
    ],
    [
        4, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7, 7, 1,
    ],
    [
        4, 0, 8, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 0, 0, 0, 8,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 7, 7, 7, 1,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 0, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, 7, 1,
    ],
    [
        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    ],
    [
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    ],
    [
        6, 6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    ],
    [
        4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 6, 0, 6, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 2, 0, 0, 5, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2,
    ],
    [
        4, 0, 6, 0, 6, 0, 0, 0, 0, 4, 6, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 5, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2,
    ],
    [
        4, 0, 6, 0, 6, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 5, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
    ],
];

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = Application {
        window: None,
        surface: None,
        data: PositionalData::default(),
        keys: HashSet::new(),
        textures: vec![],
    };

    app.load_texture("textures/eagle.png")?;
    app.load_texture("textures/redbrick.png")?;
    app.load_texture("textures/purplestone.png")?;
    app.load_texture("textures/greystone.png")?;
    app.load_texture("textures/bluestone.png")?;
    app.load_texture("textures/mossy.png")?;
    app.load_texture("textures/wood.png")?;
    app.load_texture("textures/colorstone.png")?;

    Ok(event_loop.run_app(app)?)
}

struct Application {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    data: PositionalData,
    keys: HashSet<Key>,
    textures: Vec<RgbImage>,
}

impl Application {
    fn load_texture<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<()> {
        let img = image::open(path)?;
        self.textures.push(img.into_rgb8());
        Ok(())
    }
}

struct PositionalData {
    pub pos_x: f32,
    pub pos_y: f32,
    pub dir_x: f32,
    pub dir_y: f32,
    pub plane_x: f32,
    pub plane_y: f32,
    pub old_time: Instant,
    pub rotation_speed: f32,
    pub move_speed: f32,
}

impl Default for PositionalData {
    fn default() -> Self {
        Self {
            pos_x: 22.0,
            pos_y: 11.5,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: 0.66,
            old_time: Instant::now(),
            rotation_speed: 0.0,
            move_speed: 0.0,
        }
    }
}

impl ApplicationHandler for Application {
    fn can_create_surfaces(&mut self, event_loop: &ActiveEventLoop) {
        info!("Ready to create surfaces");
        let mut attributes = Window::default_attributes();
        attributes.resizable = true;
        attributes.title = String::from("Raycaster");
        attributes.decorations = true;
        let window = Rc::new(
            event_loop
                .create_window(attributes)
                .expect("Failed to create window"),
        );
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();
        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = &mut self.window {
            let surface = self.surface.as_mut().unwrap();

            if self.keys.contains(&Key::Character(SmolStr::from("q")))
                && self.keys.contains(&Key::Named(NamedKey::Control))
            {
                event_loop.exit();
            }

            for key in &self.keys {
                match key {
                    Key::Named(NamedKey::ArrowUp) => {
                        if WORLD_MAP
                            [(self.data.pos_x + self.data.dir_x * self.data.move_speed) as usize]
                            [self.data.pos_y as usize]
                            == 0
                        {
                            self.data.pos_x += self.data.dir_x * self.data.move_speed;
                        }
                        if WORLD_MAP[self.data.pos_x as usize]
                            [(self.data.pos_y + self.data.dir_y * self.data.move_speed) as usize]
                            == 0
                        {
                            self.data.pos_y += self.data.dir_y * self.data.move_speed;
                        }
                    }
                    Key::Named(NamedKey::ArrowDown) => {
                        if WORLD_MAP
                            [(self.data.pos_x - self.data.dir_x * self.data.move_speed) as usize]
                            [self.data.pos_y as usize]
                            == 0
                        {
                            self.data.pos_x -= self.data.dir_x * self.data.move_speed;
                        }
                        if WORLD_MAP[self.data.pos_x as usize]
                            [(self.data.pos_y - self.data.dir_y * self.data.move_speed) as usize]
                            == 0
                        {
                            self.data.pos_y -= self.data.dir_y * self.data.move_speed;
                        }
                    }
                    Key::Named(NamedKey::ArrowRight) => {
                        let old_dir_x = self.data.dir_x;
                        self.data.dir_x = self.data.dir_x * f32::cos(-self.data.rotation_speed)
                            - self.data.dir_y * f32::sin(-self.data.rotation_speed);
                        self.data.dir_y = old_dir_x * f32::sin(-self.data.rotation_speed)
                            + self.data.dir_y * f32::cos(-self.data.rotation_speed);
                        let old_plane_x = self.data.plane_x;
                        self.data.plane_x = self.data.plane_x * f32::cos(-self.data.rotation_speed)
                            - self.data.plane_y * f32::sin(-self.data.rotation_speed);
                        self.data.plane_y = old_plane_x * f32::sin(-self.data.rotation_speed)
                            + self.data.plane_y * f32::cos(-self.data.rotation_speed);
                    }
                    Key::Named(NamedKey::ArrowLeft) => {
                        let old_dir_x = self.data.dir_x;
                        self.data.dir_x = self.data.dir_x * f32::cos(self.data.rotation_speed)
                            - self.data.dir_y * f32::sin(self.data.rotation_speed);
                        self.data.dir_y = old_dir_x * f32::sin(self.data.rotation_speed)
                            + self.data.dir_y * f32::cos(self.data.rotation_speed);
                        let old_plane_x = self.data.plane_x;
                        self.data.plane_x = self.data.plane_x * f32::cos(self.data.rotation_speed)
                            - self.data.plane_y * f32::sin(self.data.rotation_speed);
                        self.data.plane_y = old_plane_x * f32::sin(self.data.rotation_speed)
                            + self.data.plane_y * f32::cos(self.data.rotation_speed);
                    }
                    _ => {}
                }
            }

            match event {
                WindowEvent::KeyboardInput { event, .. } => match event.state {
                    ElementState::Pressed => {
                        self.keys.insert(event.logical_key);
                    }
                    ElementState::Released => {
                        self.keys.remove(&event.logical_key);
                    }
                },
                WindowEvent::RedrawRequested => {
                    if let (Some(width), Some(height)) = {
                        let size = window.inner_size();
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                    } {
                        surface.resize(width, height).unwrap();

                        let mut buffer = surface.buffer_mut().unwrap();

                        buffer.fill(0);

                        for x in 0..width.get() {
                            let camera_x = 2.0 * (x as f32) / width.get() as f32 - 1.0;
                            let ray_dir_x = self.data.dir_x + self.data.plane_x * camera_x;
                            let ray_dir_y = self.data.dir_y + self.data.plane_y * camera_x;

                            let mut map_x = self.data.pos_x as i32;
                            let mut map_y = self.data.pos_y as i32;

                            let mut side_dist_x;
                            let mut side_dist_y;

                            let delta_dist_x = if ray_dir_x == 0.0 {
                                1e30
                            } else {
                                (1.0 / ray_dir_x).abs()
                            };
                            let delta_dist_y = if ray_dir_y == 0.0 {
                                1e30
                            } else {
                                (1.0 / ray_dir_y).abs()
                            };

                            let step_x;
                            let step_y;

                            let mut hit = false;
                            let mut side = 0;

                            if ray_dir_x < 0.0 {
                                step_x = -1;
                                side_dist_x = (self.data.pos_x - map_x as f32) * delta_dist_x;
                            } else {
                                step_x = 1;
                                side_dist_x = (map_x as f32 + 1.0 - self.data.pos_x) * delta_dist_x;
                            }

                            if ray_dir_y < 0.0 {
                                step_y = -1;
                                side_dist_y = (self.data.pos_y - map_y as f32) * delta_dist_y;
                            } else {
                                step_y = 1;
                                side_dist_y = (map_y as f32 + 1.0 - self.data.pos_y) * delta_dist_y;
                            }

                            while !hit {
                                if side_dist_x < side_dist_y {
                                    side_dist_x += delta_dist_x;
                                    map_x += step_x;
                                    side = 0;
                                } else {
                                    side_dist_y += delta_dist_y;
                                    map_y += step_y;
                                    side = 1;
                                }

                                if WORLD_MAP[map_x as usize][map_y as usize] > 0 {
                                    hit = true;
                                }
                            }

                            let perp_wall_dist = if side == 0 {
                                side_dist_x - delta_dist_x
                            } else {
                                side_dist_y - delta_dist_y
                            };

                            let line_height = (height.get() as f32 / perp_wall_dist) as i32;
                            let mut draw_start = -line_height / 2 + (height.get() as i32) / 2;
                            if draw_start < 0 {
                                draw_start = 0;
                            }
                            let mut draw_end = line_height / 2 + (height.get() as i32) / 2;
                            if draw_end >= height.get() as i32 {
                                draw_end = height.get() as i32 - 1;
                            }

                            let tex_num = WORLD_MAP[map_x as usize][map_y as usize] - 1;
                            let mut wall_x = if side == 0 {
                                self.data.pos_y + perp_wall_dist * ray_dir_y
                            } else {
                                self.data.pos_x + perp_wall_dist * ray_dir_x
                            };
                            wall_x -= wall_x.floor();

                            let mut tex_x = (wall_x * TEX_WIDTH as f32) as u32;
                            if side == 0 && ray_dir_x > 0.0 {
                                tex_x = TEX_WIDTH - tex_x - 1
                            };
                            if side == 1 && ray_dir_y < 0.0 {
                                tex_x = TEX_WIDTH - tex_x - 1
                            };

                            let step = 1.0 * TEX_HEIGHT as f32 / line_height as f32;
                            let mut tex_pos = (draw_start as f32 - height.get() as f32 / 2.0
                                + line_height as f32 / 2.0)
                                * step;

                            for y in 0..height.get() {
                                if !(draw_start..=draw_end).contains(&(y as i32)) {
                                    continue;
                                }
                                let tex_y = tex_pos as u32 & (TEX_HEIGHT - 1);
                                tex_pos += step;
                                let mut color =
                                    *self.textures[tex_num as usize].get_pixel(tex_x, tex_y);
                                if side == 0 {
                                    color.0[0] /= 2;
                                    color.0[1] /= 2;
                                    color.0[2] /= 2;
                                }
                                let index = y as usize * width.get() as usize + x as usize;
                                buffer[index] = color.0[2] as u32
                                    | ((color.0[1] as u32) << 8)
                                    | ((color.0[0] as u32) << 16);
                            }
                        }
                        let frame_time = self.data.old_time.elapsed().as_secs_f32();
                        self.data.old_time = Instant::now();

                        self.data.move_speed = frame_time * 5.0;
                        self.data.rotation_speed = frame_time * 3.0;

                        println!("FPS: {}", 1.0 / frame_time);
                        window.pre_present_notify();
                        buffer.present().unwrap();
                    }
                    window.request_redraw();
                }
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                _ => {}
            }
        }
    }
}
