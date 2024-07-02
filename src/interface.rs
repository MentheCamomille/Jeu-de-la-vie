extern crate x11;

use std::ptr;
use std::convert::TryInto;
use x11::xlib::*;
use crate::grid::Grid;

pub struct X11Interface {
    display: *mut Display,
    window: Window,
    gc: GC,
    grid: *mut Grid,
    width: i32,
    height: i32,
}

impl X11Interface {
    pub fn new(grid: &mut Grid) -> X11Interface {
        let display = unsafe { XOpenDisplay(ptr::null()) };
        if display.is_null() {
            panic!("Failed to open X display");
        }
        
        let screen = unsafe { XDefaultScreen(display) };
        let white_pixel = unsafe { XWhitePixel(display, screen) };
        let black_pixel = unsafe { XBlackPixel(display, screen) };
        let root_window = unsafe { XRootWindow(display, screen) };
        
        let window = unsafe {
            XCreateSimpleWindow(
                display,
                root_window,
                0,
                0,
                800,
                600,
                0,
                black_pixel,
                white_pixel,
            )
        };

        let gc = unsafe { XCreateGC(display, window, 0, ptr::null_mut()) };

        X11Interface {
            display,
            window,
            gc,
            grid: grid as *mut Grid,
            width: 800,
            height: 600,
        }
    }

    pub fn run(&mut self) {
        unsafe {
            XMapWindow(self.display, self.window);
            XFlush(self.display);

            loop {
                if self.handle_events() {
                    break;
                }
                self.draw_grid();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            XFreeGC(self.display, self.gc);
            XDestroyWindow(self.display, self.window);
            XCloseDisplay(self.display);
        }
    }

    fn handle_events(&mut self) -> bool {
        let mut event = XEvent { pad: [0; 24] };
    
        unsafe {
            while XPending(self.display) > 0 {
                XNextEvent(self.display, &mut event);
    
                match event.get_type() {
                    Expose => self.draw_grid(),
                    ButtonPress => {
                        let button_event: &XButtonEvent = event.as_ref();
                        if button_event.button == Button1 {
                            let grid = &mut *self.grid;
                            let cell_size_x: i32 = (self.width / grid.width().try_into().unwrap()) as i32;
                            let cell_size_y: i32 = (self.height / grid.height().try_into().unwrap()) as i32;
                            let x = (button_event.x / cell_size_x) as usize;
                            let y = (button_event.y / cell_size_y) as usize;
                            grid.toggle_cell(x, y);
                            self.draw_grid();
                        }
                    }
                    ConfigureNotify => {
                        let config_event: &XConfigureEvent = event.as_ref();
                        self.width = config_event.width;
                        self.height = config_event.height;
                    }
                    _ => {}
                }
            }
        }
    
        false
    }
    
    fn draw_grid(&mut self) {
        let grid = unsafe { &*self.grid };
        let cell_size_x: i32 = self.width / grid.width().try_into().unwrap();
        let cell_size_y: i32 = self.height / grid.height().try_into().unwrap();
        
        unsafe {
            XSetForeground(self.display, self.gc, 0);
            XFillRectangle(
                self.display,
                self.window,
                self.gc,
                0,
                0,
                self.width as u32,
                self.height as u32,
            );
    
            XSetForeground(self.display, self.gc, 0xFFFFFFFF);
            for i in 0..grid.width() {
                for j in 0..grid.height() {
                    if grid.get_cell(i, j) {
                        let x: i32 = i as i32 * cell_size_x;
                        let y: i32 = j as i32 * cell_size_y;
                        XFillRectangle(
                            self.display,
                            self.window,
                            self.gc,
                            x,
                            y,
                            cell_size_x.try_into().unwrap() as u32,
                            cell_size_y.try_into().unwrap() as u32,
                        );
                    }
                }
            }
    
            XFlush(self.display);
        }
    }
}
