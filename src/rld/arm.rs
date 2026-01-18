use raylib::prelude::*;

#[derive(Debug)]
pub struct Arm {
    arm_center: Vector2,
    arm_radius: i32,
    theta: f32,
    angle: f32,
    leds: Vec<f32>,
    leds_pos: Vec<Vector2>,
}

impl Arm {
    pub fn new(rotating_point: Vector2, start_angle: f32, arm_radius: i32, nb_led: u8) -> Self {
        let mut arm = Arm {
            arm_center: rotating_point,
            arm_radius: arm_radius,
            theta: 0.1,
            angle: start_angle,
            leds: Vec::new(),
            leds_pos: Vec::new(),
        };

        let radius_cut = (arm_radius as f32)/((nb_led - 1) as f32);
        for i in 0..nb_led {
            arm.leds.push(radius_cut*(i as f32));
        }

        let current_angle = start_angle;
        for led_dist_r in arm.leds.iter() {
            let rest_pos = Vector2::new(*led_dist_r, 0.0);
            let current_pos = Vector2::new(
                rest_pos.x*current_angle.cos() - rest_pos.y*current_angle.sin(),
                rest_pos.x*current_angle.sin() + rest_pos.y*current_angle.cos()
            );
            arm.leds_pos.push(current_pos);
        }

        arm
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle, image_r: &Image) {
        //handle.draw_line_ex(self.arm_center, self.arm_center + self.arm_start, 5.0, Color::WHITE);
        let width = image_r.width;
        let height = image_r.height;
        let pixels = image_r.get_image_data();

        for leds_pos_r in self.leds_pos.iter() {
            let x = ((width as f32 / (2.0 * self.arm_radius as f32)) * leds_pos_r.x) as i32 + width/2;
            if x >= width {
                break;
            }
            let y = ((width as f32 / (2.0 * self.arm_radius as f32)) * leds_pos_r.y) as i32 + width/2;
            if y >= height {
                break;
            }
            let idx = (y * width + x) as usize;
            let color = pixels[idx];
            handle.draw_circle_v(*leds_pos_r + self.arm_center, 4.0, color);
        }
    }

    pub fn update(&mut self) {
        let theta = self.theta;
        self.angle += theta;
        let mut new_leds_pos = Vec::new();
        for led_pos_r in self.leds_pos.iter() {
            let new_pos = Vector2::new(
                led_pos_r.x*theta.cos() - led_pos_r.y*theta.sin(),
                led_pos_r.x*theta.sin() + led_pos_r.y*theta.cos()
            );
            new_leds_pos.push(new_pos);
        }
        self.leds_pos = new_leds_pos;
    }
}

