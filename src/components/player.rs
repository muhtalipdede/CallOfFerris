use ggez::{graphics::Image, Context, GameResult};
use ggez_goodies::{
    camera::{Camera, CameraDraw},
    nalgebra_glm::Vec2,
};

use crate::HEIGHT;

pub struct Player {
    pub pos_x: f32,
    pub pos_y: f32,
    pub ammo: i32,

    gravity: f32,
    velocity: f32,
    going_boom: bool,
}

impl Player {
    pub fn new(pos_x: f32) -> Self {
        Self {
            pos_x,
            ammo: 10,
            pos_y: 0.,
            gravity: 0.1,
            velocity: 0.,
            going_boom: false,
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        resources: &Vec<Image>,
    ) -> GameResult<()> {
        const HEIGHT2: f32 = HEIGHT / 2.;

        &resources[0].draw_camera(
            &camera,
            ctx,
            Vec2::new(self.pos_x, (-HEIGHT2 + 155.) + self.pos_y),
            0.0,
        );

        &resources[1].draw_camera(
            &camera,
            ctx,
            Vec2::new(self.pos_x - 50., (-HEIGHT2 + 150.) + self.pos_y),
            0.0,
        );

        Ok(())
    }

    pub fn go_boom(&mut self) {
        self.velocity -= 2.5;
        self.going_boom = true;
    }

    pub fn update(&mut self, gonna_boom: bool) {
        if self.going_boom {
            self.pos_y -= self.velocity;
            
            if self.pos_y < 0. {
                self.going_boom = false;
                self.pos_y = 0.;
            }
        }
        
        if self.pos_y > 0. || gonna_boom {
            self.velocity += self.gravity;
            self.pos_y -= self.velocity;
        }
    }
}