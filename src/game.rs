use crate::gfx;
use crate::Player;
use cgmath::{perspective, vec3, vec4, Deg, Matrix4, Point3, Rad};
use glium::{
    glutin::event::{KeyboardInput, VirtualKeyCode},
    Display, Program, Surface, VertexBuffer,
};
use std::time::Duration;

pub struct Game {
    t: f32,
    w: f32,
    disp: Display,
    vbo: Option<VertexBuffer<gfx::Vertex>>,
    program: Option<Program>,
    indicies: Option<glium::index::IndexBuffer<u16>>,
    projection: Matrix4<f32>,
    modelview: Matrix4<f32>,
    player: Player,
}

impl Game {
    pub fn new(disp: Display) -> Self {
        let (width, height) = disp.get_framebuffer_dimensions();
        let shape: Vec<gfx::Vertex> = vec![
            vec4(0.0, 0.0, 0.0, 1.0),
            vec4(0.0, 0.0, 1.0, 1.0),
            vec4(0.0, 1.0, 0.0, 1.0),
            vec4(0.0, 1.0, 1.0, 1.0),
            vec4(1.0, 0.0, 0.0, 1.0),
            vec4(1.0, 0.0, 1.0, 1.0),
            vec4(1.0, 1.0, 0.0, 1.0),
            vec4(1.0, 1.0, 1.0, 1.0),
        ]
        .into_iter()
        .map(Into::into)
        .collect();

        let vertex_src = r#"
            #version 140

            in vec4 position;
            uniform mat4 projection_worldview;

            void main() {
                gl_Position = projection_worldview * position;
            }
        "#;

        let fragment_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let mut game = Game {
            t: 0.0,
            w: 0.0,
            disp,
            vbo: None,
            program: None,
            indicies: None,
            projection: perspective(Deg(90.0), (width as f32) / (height as f32), 0.1, 100.0),
            modelview: Matrix4::from_scale(0.5) * Matrix4::from_translation(vec3(-0.5, -0.5, 0.0)),
            player: Player::new(0, 0, 0, '#'),
        };

        game.vbo = Some(VertexBuffer::new(&game.disp, &shape).unwrap());
        game.indicies = Some(
            glium::index::IndexBuffer::new(
                &game.disp,
                glium::index::PrimitiveType::TrianglesList,
                &[
                    0, 2, 1, 2, 1, 3, 4, 5, 6, 6, 5, 7, 0, 4, 2, 4, 6, 2, 0, 1, 5, 5, 4, 0, 2, 7,
                    3, 7, 3, 6, 1, 3, 5, 3, 7, 5,
                ],
            )
            .unwrap(),
        );
        game.program =
            Some(Program::from_source(&game.disp, vertex_src, fragment_src, None).unwrap());

        game
    }

    pub fn update(&mut self, elapsed: Duration) {
        self.t += 0.002 * elapsed.as_millis() as f32;
        self.w += 0.004 * elapsed.as_millis() as f32;
    }

    pub fn input(&mut self, input: KeyboardInput, elapsed: Duration) {
        match input.scancode {
            17 => {
                // w
                self.player.position +=
                    self.player.orientation * elapsed.as_millis() as f32 * 0.002;
            }
            30 => {
                // a
                let dir = self.player.orientation.cross(self.player.up);
                self.player.position -= dir * elapsed.as_millis() as f32 * 0.002;
            }
            31 => {
                // s
                self.player.position -=
                    self.player.orientation * elapsed.as_millis() as f32 * 0.002;
            }
            32 => {
                // d
                let dir = self.player.orientation.cross(self.player.up);
                self.player.position += dir * elapsed.as_millis() as f32 * 0.002;
            }

            _ => (),
        };
    }

    pub fn render(&self) {
        let mut frame = self.disp.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame
            .draw(
                self.vbo.as_ref().unwrap(),
                self.indicies.as_ref().unwrap(),
                self.program.as_ref().unwrap(),
                &uniform! { projection_worldview: Into::<[[f32; 4]; 4]>::into(self.projection * Matrix4::look_to_rh(self.player.position, self.player.orientation, self.player.up) * self.modelview* Matrix4::from_angle_z(Rad(self.t)) * Matrix4::from_angle_x(Rad(self.w))) },
                &Default::default(),
            )
            .unwrap();
        frame.finish().unwrap();
    }
}
