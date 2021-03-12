use crate::gfx;
use cgmath::Vector2;
use glium::{Display, Program, Surface, VertexBuffer};
use std::time::Duration;

pub struct Game {
    t: f32,
    disp: Display,
    vbo: Option<VertexBuffer<gfx::Vertex>>,
    program: Option<Program>,
    indicies: Option<glium::index::NoIndices>,
}

impl Game {
    pub fn new(disp: Display) -> Self {
        let v1 = Vector2::new(-0.5, -0.5);
        let v2 = Vector2::new(0.0, 0.5);
        let v3 = Vector2::new(0.5, 0.25);
        let shape: Vec<_> = vec![v1, v2, v3]
            .iter()
            .map(|v| gfx::Vertex {
                position: [v.x, v.y],
            })
            .collect();

        let vertex_src = r#"
            #version 140

            in vec2 position;

            uniform float t;

            void main() {
                float c = cos(t);
                float s = sin(t);
                float x = position.x * c - position.y * s;
                float y = position.y * c + position.x * s;
                gl_Position = vec4(x, y, 0.0, 1.0);
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
            disp,
            vbo: None,
            program: None,
            indicies: None,
        };

        game.vbo = Some(VertexBuffer::new(&game.disp, &shape).unwrap());
        game.indicies = Some(glium::index::NoIndices(
            glium::index::PrimitiveType::TrianglesList,
        ));
        game.program =
            Some(Program::from_source(&game.disp, vertex_src, fragment_src, None).unwrap());

        game
    }

    pub fn update(&mut self, elapsed: Duration) {
        self.t += 0.002 * elapsed.as_millis() as f32;
        if self.t > std::f32::consts::PI * 2.0 {
            self.t = 0.0;
        }
    }

    pub fn render(&self) {
        let mut frame = self.disp.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame
            .draw(
                self.vbo.as_ref().unwrap(),
                self.indicies.as_ref().unwrap(),
                self.program.as_ref().unwrap(),
                &uniform! { t: self.t},
                &Default::default(),
            )
            .unwrap();
        frame.finish().unwrap();
    }
}
