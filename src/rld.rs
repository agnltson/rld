mod arm;

use raylib::prelude::*;

pub struct Rld {
    screen_width: i32,
    screen_height: i32,
    arms: Vec<arm::Arm>,
    image: Image,
}

impl Rld {
    pub fn new(image_path: &str, screen_width: i32, screen_height: i32, nb_arms: i32, nb_led: u8) -> Self {
        let center = Vector2::new((screen_width/2) as f32, (screen_height/2) as f32);
        let arm_len = screen_height/2;

        let image = Image::load_image(image_path)
            .expect("Failed to load image");

        let mut rld = Rld {
            screen_width: screen_width,
            screen_height: screen_height,
            arms: Vec::new(),
            image: image,
        };
        let angle_cut = (std::f64::consts::PI * 2.0) as f32 / nb_arms as f32;
        for i in 0..nb_arms {
            let idx = i as f32;
            rld.arms.push(arm::Arm::new(center, angle_cut * idx, arm_len, nb_led));
        }
        rld
    }

    pub fn start(&mut self) {
        let width = self.screen_width;
        let height = self.screen_height;
        let (mut rl, thread) = raylib::init()
            .size(width, height)
            .title("RLD")
            .build();

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::BLACK);
            self.arms.iter()
                .for_each(|arm| arm.draw(&mut d, &self.image));
            self.arms.iter_mut()
                .for_each(|arm| arm.update());
        }
    }
}
