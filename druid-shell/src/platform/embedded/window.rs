// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! embedded-graphics window creation and management.

/*
    use std::any::Any;
    use std::cell::{Cell, RefCell};
    use std::ffi::c_void;
    use std::ffi::OsString;
    use std::os::raw::{c_int, c_uint};
    use std::ptr;
    use std::slice;
    use std::sync::{Arc, Mutex, Weak};

    use gdk::{EventKey, EventMask, ModifierType, ScrollDirection, WindowExt};
    use gio::ApplicationExt;
    use gtk::prelude::*;
    use gtk::{AccelGroup, ApplicationWindow};
*/
use crate::kurbo::{Point, Size, Vec2};
use crate::piet::{Piet, RenderContext};

////use super::dialog::{self, FileDialogType};
////use super::menu::Menu;
////use super::runloop::with_application;
////use super::util::assert_main_thread;

////use crate::common_util::IdleCallback;
////use crate::dialog::{FileDialogOptions, FileInfo};
////use crate::keyboard;
use crate::mouse::{Cursor, MouseButton, MouseEvent};
use crate::window::{Text, TimerToken, WinCtx, WinHandler};
use crate::Error;

/*
    /// Taken from https://gtk-rs.org/docs-src/tutorial/closures
    /// It is used to reduce the boilerplate of setting up gtk callbacks
    /// Example:
    /// ```
    /// button.connect_clicked(clone!(handle => move |_| { ... }))
    /// ```
    /// is equivalent to:
    /// ```
    /// {
    ///     let handle = handle.clone();
    ///     button.connect_clicked(move |_| { ... })
    /// }
    /// ```
    macro_rules! clone {
        (@param _) => ( _ );
        (@param $x:ident) => ( $x );
        ($($n:ident),+ => move || $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move || $body
            }
        );
        ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
            }
        );
    }
*/ ////

#[derive(Clone, Copy, Default)]
pub struct WindowHandle<THandler> { ////
////pub struct WindowHandle {
    pub(crate) state: WindowState<THandler>, ////
    ////pub(crate) state: Weak<WindowState>,
}

/// Builder abstraction for creating new windows
pub struct WindowBuilder<THandler> { ////
////pub struct WindowBuilder {
    handler: Option<THandler>, ////
    ////handler: Option<&'static mut dyn WinHandler>, ////
    ////handler: Option<Box<dyn WinHandler>>,
    ////title: String,
    ////menu: Option<Menu>,
    size: Size,
}

/* ////
    #[derive(Clone)]
    pub struct IdleHandle {
        idle_queue: Arc<Mutex<Vec<Box<dyn IdleCallback>>>>,
        state: Weak<WindowState>,
    }
*/ ////

#[derive(Clone, Copy, Default)]
pub(crate) struct WindowState<THandler> {
    handler: Option<THandler>, ////
    ////pub(crate) handler: RefCell<Box<dyn WinHandler>>,
    ////idle_queue: Arc<Mutex<Vec<Box<dyn IdleCallback>>>>,
    ////current_keyval: RefCell<Option<u32>>,
}

/* ////
    pub(crate) struct WinCtxImpl<'a> {
        handle: &'a WindowHandle,
        text: Text<'static>,
    }
*/ ////

impl<THandler> WindowBuilder<THandler> { ////
////impl WindowBuilder {
    pub fn new() -> Self { ////
    ////pub fn new() -> WindowBuilder {
        WindowBuilder  {
            handler: None,
            ////title: String::new(),
            ////menu: None,
            size: Size::new( ////
                240., //// crate::env::WINDOW_WIDTH as f64, 
                240., //// crate::env::WINDOW_HEIGHT as f64
            ), ////
            ////size: Size::new(500.0, 400.0),
        }
    }

    pub fn set_handler(&mut self, handler: THandler) { ////
    ////pub fn set_handler(&mut self, handler: Box<dyn WinHandler>) {
        self.handler = Some(handler);
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn build(self) -> Result<WindowHandle<THandler>, Error> { ////
    ////pub fn build(self) -> Result<WindowHandle, Error> {
        let handler = self
            .handler
            .expect("Tried to build a window without setting the handler");

        /*
            let window = with_application(|app| ApplicationWindow::new(&app));

            let dpi_scale = window
                .get_display()
                .map(|c| c.get_default_screen().get_resolution() as f64)
                .unwrap_or(96.0)
                / 96.0;

            window.set_default_size(
                (self.size.width * dpi_scale) as i32,
                (self.size.height * dpi_scale) as i32,
            );

            let accel_group = AccelGroup::new();
            window.add_accel_group(&accel_group);

            let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
            window.add(&vbox);
        */

        let win_state = WindowState {
            handler: Some(handler)
        };

        /*
            with_application(|app| {
                app.connect_shutdown(clone!(win_state => move |_| {
                    // this ties a clone of Arc<WindowState> to the ApplicationWindow to keep it alive
                    // when the ApplicationWindow is destroyed, the last Arc is dropped
                    // and any Weak<WindowState> will be None on upgrade()
                    let _ = &win_state;
                }))
            });
        */

        let handle = WindowHandle {
            state: win_state,
        };

        /*
            if let Some(menu) = self.menu {
                let menu = menu.into_gtk_menubar(&handle, &accel_group);
                vbox.pack_start(&menu, false, false, 0);
            }

            let drawing_area = gtk::DrawingArea::new();

            drawing_area.set_events(
                EventMask::EXPOSURE_MASK
                    | EventMask::POINTER_MOTION_MASK
                    | EventMask::BUTTON_PRESS_MASK
                    | EventMask::BUTTON_RELEASE_MASK
                    | EventMask::KEY_PRESS_MASK
                    | EventMask::ENTER_NOTIFY_MASK
                    | EventMask::KEY_RELEASE_MASK
                    | EventMask::SCROLL_MASK,
            );

            drawing_area.set_can_focus(true);
            drawing_area.grab_focus();

            drawing_area.connect_enter_notify_event(|widget, _| {
                widget.grab_focus();

                Inhibit(true)
            });

            let last_size = Cell::new((0, 0));

            drawing_area.connect_draw(clone!(handle => move |widget, context| {
                if let Some(state) = handle.state.upgrade() {
                    let mut ctx = WinCtxImpl::from(&handle);

                    let extents = context.clip_extents();
                    let size = (
                        (extents.2 - extents.0) as u32,
                        (extents.3 - extents.1) as u32,
                    );

                    if last_size.get() != size {
                        last_size.set(size);
                        state.handler.borrow_mut().size(size.0, size.1, &mut ctx);
                    }

                    // For some reason piet needs a mutable context, so give it one I guess.
                    let mut context = context.clone();
                    let mut piet_context = Piet::new(&mut context);

                    if let Ok(mut handler_borrow) = state.handler.try_borrow_mut() {
                        let anim = handler_borrow
                            .paint(&mut piet_context, &mut ctx);
                        if let Err(e) = piet_context.finish() {
                            eprintln!("piet error on render: {:?}", e);
                        }

                        if anim {
                            widget.queue_draw();
                        }
                    }

                }

                Inhibit(false)
            }));

            drawing_area.connect_button_press_event(clone!(handle => move |_widget, button| {
                if let Some(state) = handle.state.upgrade() {
                    let mut ctx = WinCtxImpl::from(&handle);

                    state.handler.borrow_mut().mouse_down(
                        &MouseEvent {
                            pos: Point::from(button.get_position()),
                            count: get_mouse_click_count(button.get_event_type()),
                            mods: get_modifiers(button.get_state()),
                            button: get_mouse_button(button.get_button()),
                        },
                        &mut ctx,
                    );
                }

                Inhibit(true)
            }));

            drawing_area.connect_button_release_event(clone!(handle => move |_widget, button| {
                if let Some(state) = handle.state.upgrade() {
                    let mut ctx = WinCtxImpl::from(&handle);

                    state.handler.borrow_mut().mouse_up(
                        &MouseEvent {
                            pos: Point::from(button.get_position()),
                            mods: get_modifiers(button.get_state()),
                            count: 0,
                            button: get_mouse_button(button.get_button()),
                        },
                        &mut ctx,
                    );
                }

                Inhibit(true)
            }));

            drawing_area.connect_motion_notify_event(clone!(handle=>move |_widget, motion| {
                if let Some(state) = handle.state.upgrade() {
                    let mut ctx = WinCtxImpl::from(&handle);

                    let pos = Point::from(motion.get_position());
                    let mouse_event = MouseEvent {
                        pos,
                        mods: get_modifiers(motion.get_state()),
                        count: 0,
                        button: get_mouse_button_from_modifiers(motion.get_state()),
                    };

                    state
                        .handler
                        .borrow_mut()
                        .mouse_move(&mouse_event, &mut ctx);
                }

                Inhibit(true)
            }));

            drawing_area.connect_destroy(clone!(handle => move |_widget| {
                if let Some(state) = handle.state.upgrade() {
                    let mut ctx = WinCtxImpl::from(&handle);
                    state.handler.borrow_mut().destroy(&mut ctx);
                }
            }));

            vbox.pack_end(&drawing_area, true, true, 0);
        */

        ////TODO handler.connect(&mut handle.into()); ////

        /* ////
            win_state
                .handler
                ////.borrow_mut()
                .connect(&handle.into());
                ////.connect(&handle.clone().into());
        */ ////

        Ok(handle)
    }
}

impl<THandler> WindowHandle<THandler> { ////
////impl WindowHandle {
    pub fn show(&self) {
        /* ////
        if let Some(state) = self.state.upgrade() {
            state.window.show_all();
        }
        */ ////
    }

    /// Close the window.
    pub fn close(&self) {
        /*
        if let Some(state) = self.state.upgrade() {
            with_application(|app| {
                app.remove_window(&state.window);
            });
        }
        */
    }

    /// Bring this window to the front of the window stack and give it focus.
    pub fn bring_to_front_and_focus(&self) {
        //FIXME: implementation goes here
        ////log::warn!("bring_to_front_and_focus not yet implemented for gtk");
    }

    // Request invalidation of the entire window contents.
    pub fn invalidate(&self) {
        /* ////
        if let Some(state) = self.state.upgrade() {
            state.window.queue_draw();
        }
        */ ////
    }
    /* ////
        /// Get a handle that can be used to schedule an idle task.
        pub fn get_idle_handle(&self) -> Option<IdleHandle> {
            self.state.upgrade().map(|s| IdleHandle {
                idle_queue: s.idle_queue.clone(),
                state: Arc::downgrade(&s),
            })
        }

        /// Get the dpi of the window.
        ///
        /// TODO: we want to migrate this from dpi (with 96 as nominal) to a scale
        /// factor (with 1 as nominal).
        pub fn get_dpi(&self) -> f32 {
            self.state
                .upgrade()
                .and_then(|s| s.window.get_window())
                .map(|w| w.get_display().get_default_screen().get_resolution() as f32)
                .unwrap_or(96.0)
        }

        // TODO: the following methods are cut'n'paste code. A good way to DRY
        // would be to have a platform-independent trait with these as methods with
        // default implementations.

        /// Convert a dimension in px units to physical pixels (rounding).
        pub fn px_to_pixels(&self, x: f32) -> i32 {
            (x * self.get_dpi() * (1.0 / 96.0)).round() as i32
        }

        /// Convert a point in px units to physical pixels (rounding).
        pub fn px_to_pixels_xy(&self, x: f32, y: f32) -> (i32, i32) {
            let scale = self.get_dpi() * (1.0 / 96.0);
            ((x * scale).round() as i32, (y * scale).round() as i32)
        }

        /// Convert a dimension in physical pixels to px units.
        pub fn pixels_to_px<T: Into<f64>>(&self, x: T) -> f32 {
            (x.into() as f32) * 96.0 / self.get_dpi()
        }

        /// Convert a point in physical pixels to px units.
        pub fn pixels_to_px_xy<T: Into<f64>>(&self, x: T, y: T) -> (f32, f32) {
            let scale = 96.0 / self.get_dpi();
            ((x.into() as f32) * scale, (y.into() as f32) * scale)
        }
    */ ////
}

/* ////
    unsafe impl Send for IdleHandle {}
    // WindowState needs to be Send + Sync so it can be passed into glib closures
    unsafe impl Send for WindowState {}
    unsafe impl Sync for WindowState {}

    impl IdleHandle {
        /// Add an idle handler, which is called (once) when the message loop
        /// is empty. The idle handler will be run from the main UI thread, and
        /// won't be scheduled if the associated view has been dropped.
        ///
        /// Note: the name "idle" suggests that it will be scheduled with a lower
        /// priority than other UI events, but that's not necessarily the case.
        pub fn add_idle<F>(&self, callback: F)
        where
            F: FnOnce(&dyn Any) + Send + 'static,
        {
            let mut queue = self.idle_queue.lock().unwrap();
            if let Some(state) = self.state.upgrade() {
                if queue.is_empty() {
                    queue.push(Box::new(callback));
                    gdk::threads_add_idle(move || run_idle(&state));
                } else {
                    queue.push(Box::new(callback));
                }
            }
        }
    }

    fn run_idle(state: &Arc<WindowState>) -> bool {
        assert_main_thread();
        let mut handler = state.handler.borrow_mut();
        let handler_as_any = handler.as_any();

        let queue: Vec<_> = std::mem::replace(&mut state.idle_queue.lock().unwrap(), Vec::new());

        for callback in queue {
            callback.call(handler_as_any);
        }
        false
    }

    impl<'a> WinCtx<'a> for WinCtxImpl<'a> {
        fn invalidate(&mut self) {
            self.handle.invalidate();
        }

        fn text_factory(&mut self) -> &mut Text<'a> {
            &mut self.text
        }

        fn request_timer(&mut self, deadline: std::time::Instant) -> TimerToken {
            let interval = time_interval_from_deadline(deadline);
            let token = next_timer_id();

            let handle = self.handle.clone();

            gdk::threads_add_timeout(interval, move || {
                if let Some(state) = handle.state.upgrade() {
                    if let Ok(mut handler_borrow) = state.handler.try_borrow_mut() {
                        let mut ctx = WinCtxImpl::from(&handle);
                        handler_borrow.timer(TimerToken::new(token), &mut ctx);
                        return false;
                    }
                }
                true
            });

            TimerToken::new(token)
        }
    }

    impl<'a> From<&'a WindowHandle> for WinCtxImpl<'a> {
        fn from(handle: &'a WindowHandle) -> Self {
            WinCtxImpl {
                handle,
                text: Text::new(),
            }
        }
    }
*/ ////