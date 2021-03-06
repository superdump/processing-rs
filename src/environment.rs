use glium;
use glium::glutin;

use Screen;

impl<'a> Screen<'a> {
    #[inline]
    pub fn reset_cursor(&mut self) {
        self.currCursor = glium::glutin::MouseCursor::Default;
        self.display.gl_window().set_cursor(
            glium::glutin::MouseCursor::Default,
        );
    }

    #[inline]
    pub fn cursor(&mut self, cursorType: &str) {
        // GLFW.SetInputMode(window, GLFW.CURSOR, GLFW.CURSOR_NORMAL);
        if cursorType == "HAND" {
            self.currCursor = glium::glutin::MouseCursor::Hand;
        } else if cursorType == "ARROW" {
            self.currCursor = glium::glutin::MouseCursor::Arrow;
        } else if cursorType == "CROSS" {
            self.currCursor = glium::glutin::MouseCursor::Crosshair;
        } else if cursorType == "MOVE" {
            self.currCursor = glium::glutin::MouseCursor::Move;
        } else if cursorType == "TEXT" {
            self.currCursor = glium::glutin::MouseCursor::Text;
        } else if cursorType == "WAIT" {
            self.currCursor = glium::glutin::MouseCursor::Wait;
        }
        self.display.gl_window().set_cursor(self.currCursor);
    }

    #[inline]
    pub fn focused(&mut self) -> bool {
        let mut focused = false;
        self.events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Focused(b) => {
                        focused = true;
                    }
                    _ => (),
                }
            }
            _ => (),
        });

        focused
    }

    #[inline]
    pub fn frameCount(&self) -> isize {
        self.frameCount
    }

    #[inline]
    pub fn get_frameRate(&self) -> isize {
        self.frameRate
    }

    #[inline]
    pub fn set_frameRate(&mut self, fRate: isize) {
        self.frameRate = fRate;
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn noCursor(&mut self) {
        unimplemented!{};
        // GLFW.SetInputMode(self, GLFW.CURSOR, GLFW.CURSOR_HIDDEN);
    }

    #[inline]
    pub fn noSmooth(&mut self) {
        self.draw_params = glium::draw_parameters::DrawParameters {
            smooth: None,
            ..self.draw_params.clone()
        };
    }

    #[inline]
    pub fn smooth(&mut self) {
        self.draw_params = glium::draw_parameters::DrawParameters {
            smooth: Some(glium::draw_parameters::Smooth::Nicest),
            ..self.draw_params.clone()
        };
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }
}
