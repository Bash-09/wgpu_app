use egui_winit::winit::event::{Event, WindowEvent, DeviceEvent, MouseButton, ElementState, MouseScrollDelta};

pub struct Mouse {
    this_frame: [bool; 10],
    pressed: [bool; 10],
    pos: (i32, i32),
    delta: (f64, f64),
    wheel: (f32, f32),

    focused: bool,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            this_frame: [false; 10],
            pressed: [false; 10],
            pos: (0, 0),
            delta: (0.0, 0.0),
            wheel: (0.0, 0.0),

            focused: true,
        }
    }

    fn press_button(&mut self, button: usize) {
        self.this_frame[button] = true;
        self.pressed[button] = true;
    }

    fn release_button(&mut self, button: usize) {
        self.this_frame[button] = true;
        self.pressed[button] = false;
    }

    pub fn translate(&mut self, delta: (f64, f64)) {
        self.delta.0 += delta.0;
        self.delta.1 += delta.1;
    }

    fn scroll(&mut self, wheel: (f32, f32)) {
        if !self.focused {
            return;
        }
        self.wheel.0 += wheel.0;
        self.wheel.1 += wheel.1;
    }

    /// Set the new position for the mouse, updating the delta relative to where it last was
    fn update_pos(&mut self, pos: (i32, i32)) {
        // self.delta.0 = (pos.0 - self.pos.0) as f64;
        // self.delta.1 = (pos.1 - self.pos.1) as f64;
        self.pos = pos;
    }

    /// This function is called automatically in the application loop, you shouldn't be calling this yourself.
    pub fn handle_event(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent {window_id: _, event} => {
                match event {
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        ..
                    } => {
                        self.update_pos((position.x as i32, position.y as i32));
                    }
                    WindowEvent::MouseInput {
                        device_id: _,
                        state,
                        button,
                        ..
                    } => {
                        let mbutton: u16;
                        match button {
                            MouseButton::Left => {
                                mbutton = 0;
                            }
                            MouseButton::Middle => {
                                mbutton = 1;
                            }
                            MouseButton::Right => {
                                mbutton = 2;
                            }
                            MouseButton::Other(bnum) => {
                                if bnum > &(9 as u16) {
                                    return;
                                }
                                mbutton = *bnum;
                            }
                        }
                        let mut pressed = false;
                        if state == &ElementState::Pressed {
                            pressed = true;
                        }
                        if pressed {
                            self.press_button(mbutton as usize);
                        } else {
                            self.release_button(mbutton as usize);
                        }
                    }
                    WindowEvent::MouseWheel {
                        device_id: _, delta, ..
                    } => match delta {
                        MouseScrollDelta::LineDelta(y, x) => {
                            self.scroll((*x, *y));
                        }
                        _ => {}
                    },
                    WindowEvent::Focused(focused) => {
                        self.focused = *focused;
                    },
                    _ => {}
                }
            },
            Event::DeviceEvent {device_id: _, event} => {
                match event {
                    DeviceEvent::MouseMotion{ delta} => {
                        if self.focused {
                            self.translate(*delta);
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    /// Resets the Mouse for the next frame, this function is called automatically so you shouldn't need to call this function yourself.
    pub fn next_frame(&mut self) {
        self.delta = (0.0, 0.0);
        self.wheel = (0.0, 0.0);
        self.this_frame = [false; 10];
    }


    /// Get a tuple containing the x and y position of the mouse inside the window
    pub fn get_pos(&self) -> (i32, i32) {
        self.pos
    }

    /// Get the distance in pixels that the mouse has moved since the last frame
    pub fn get_delta(&self) -> (f64, f64) {
        self.delta
    }

    /// Get the vertical and horizontal scroll distance since last frame
    pub fn get_scroll(&self) -> (f32, f32) {
        self.wheel
    }

    /// Returns if the provided mouse button is currently held down
    pub fn is_pressed(&self, button: usize) -> bool {
        self.pressed[button]
    }

    /// Returns if the provided mouse button was pressed down this frame
    pub fn pressed_this_frame(&self, button: usize) -> bool {
        self.pressed[button] && self.this_frame[button]
    }

    /// Returns if the provided mouse button was released this frame
    pub fn released_this_frame(&self, button: usize) -> bool {
        !self.pressed[button] && self.this_frame[button]
    }
}
