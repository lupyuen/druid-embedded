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

//! The implementation of the WinHandler trait (druid-shell integration).

/* ////
    use std::any::Any;
    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::rc::Rc;
    use std::time::Instant;

    use log::{error, info, warn};
*/ ////

use core::marker::PhantomData; ////
use core::clone::Clone; ////
use crate::kurbo::{Rect, Size}; ////
use crate::piet::{Piet, RenderContext};
use crate::shell::{
    /* Application, */ Cursor, /* FileDialogOptions, */ MouseEvent, WinCtx, WinHandler, WindowHandle,
};

/* ////
    use crate::app_delegate::{AppDelegate, DelegateCtx};
    use crate::menu::ContextMenu;
    use crate::theme;
*/ ////
////use crate::window::Window;
use crate::{
    BaseState, /* Command, */ Data, Env, Event, EventCtx, /* KeyEvent, KeyModifiers, */ LayoutCtx, /* MenuDesc, */ ////
    PaintCtx, /* TimerToken, */ UpdateCtx, /* WheelEvent, WindowDesc, */ WindowId,
    WindowIdType, WindowBox, ////
};

////use crate::command::sys as sys_cmd;

//  Static storage for Windows, Window Handlers and Application Data, used for embedded platforms only
 
/// Max number of Windows supported. i=0 is not used, so MAX_WINDOWS should be 1 more than max number of Windows.
pub const MAX_WINDOWS: usize = 3; ////

/// Specialised Trait for handling static Windows, Window Handlers and Application Data on embedded platforms
pub trait GlobalWindows<D: Data + 'static + Default> { ////
    /// Add a WindowBox for the Data type
    fn add_window(&self, window_id: WindowId, window: WindowBox<D>);
    /// Add a Window Handler for the Data type
    fn add_handler(&self, window_id: WindowId, handler: DruidHandler<D>);
    /// Return a Window Handle that wraps the Window Handler for the Data type
    fn get_handle(&self, window_id: WindowId) -> WindowHandle<DruidHandler<D>>;
    /// Set the application data
    fn set_data(&self, data: D);
    /// Handle the Window event
    fn window_event(
        &mut self, 
        window_id: WindowId,
        ctx: &mut EventCtx<D>, 
        event: &Event, 
    );
    /// Handle the Window update
    fn window_update(
        &mut self, 
        window_id: WindowId,
        ctx: &mut UpdateCtx<D>, 
    );
    /// Handle the Window layout
    fn window_layout(
        &mut self,
        window_id: WindowId,
        layout_ctx: &mut LayoutCtx,
    );
    /// Handle the Window painting
    fn window_paint(
        &mut self, 
        window_id: WindowId,
        paint_ctx: &mut PaintCtx, 
    );
    /// Return true if Window is active
    fn window_has_active(
        &mut self,
        window_id: WindowId,
    ) -> bool;
}

/// Default Trait will not have static Windows and Window Handlers
impl<D: Data + 'static + Default> GlobalWindows<D> for AppState<D> { ////
    default fn add_window(&self, _window_id: WindowId, _window: WindowBox<D>)
        { panic!("no global windows") }
    default fn add_handler(&self, _window_id: WindowId, _handler: DruidHandler<D>)
        { panic!("no global windows") }
    default fn get_handle(&self, _window_id: WindowId) -> WindowHandle<DruidHandler<D>>
        { panic!("no global windows") }
    default fn set_data(&self, _data: D)
        { panic!("no global windows") }
    default fn window_event(
        &mut self, 
        _window_id: WindowId,
        _ctx: &mut EventCtx<D>, 
        _event: &Event, 
    ) { panic!("no global windows") }
    default fn window_update(
        &mut self, 
        _window_id: WindowId,
        _ctx: &mut UpdateCtx<D>, 
    ) { panic!("no global windows") }
    default fn window_layout(
        &mut self,
        _window_id: WindowId,
        _layout_ctx: &mut LayoutCtx,
    ) { panic!("no global windows") }
    default fn window_paint(
        &mut self, 
        _window_id: WindowId,
        _paint_ctx: &mut PaintCtx, 
    ) { panic!("no global windows") }
    default fn window_has_active(
        &mut self,
        _window_id: WindowId,
    ) -> bool { panic!("no global windows") }
}

/// The struct implements the druid-shell `WinHandler` trait.
///
/// One `DruidHandler` exists per window.
///
/// This is something of an internal detail and possibly we don't want to surface
/// it publicly.
#[derive(Clone, Default)] ////
pub struct DruidHandler<T: Data + 'static + Default> { ////
////pub struct DruidHandler<T: Data> {
    /// The shared app state.
    ////app_state: Rc<RefCell<AppState<T>>>,

    /// The id for the current window.
    pub window_id: WindowId, ////
    pub phantom: PhantomData<T>,  ////  Needed to do compile-time checking for `Data`
}

/// State shared by all windows in the UI.
#[derive(Clone)] ////
pub struct AppState<T: Data + 'static + Default> { ////
////pub(crate) struct AppState<T: Data> {
    phantom: PhantomData<T>,  ////  Needed to do compile-time checking for `Data`
    ////delegate: Option<Box<dyn AppDelegate<T>>>,
    ////command_queue: VecDeque<(WindowId, Command)>,
    ////windows: Windows<T>,  //// Replaced by ALL_WINDOWS
    ////pub(crate) env: Env,  //// Replaced by Env{}
    ////pub(crate) data: T,   //// Replaced by ALL_DATA
}

/// All active windows.
#[derive(Clone)] ////
struct Windows<T: Data + 'static + Default> { ////
////struct Windows<T: Data> {
    phantom: PhantomData<T>,  ////  Needed to do compile-time checking for `Data`
    ////windows: HashMap<WindowId, Window<T>>,  //// Replaced by ALL_WINDOWS
    ////state: HashMap<WindowId, WindowState>,  //// Replaced by ALL_DATA and ALL_HANDLERS
}

/// Per-window state not owned by user code.
#[derive(Clone, Default)] ////
pub(crate) struct WindowState<D: Data + 'static + Default> { ////  D is Data + 'static
////pub(crate) struct WindowState {
    window_id: WindowId,  ////
    phantom: PhantomData<D>,  ////  Needed to do compile-time checking for `Data`
    pub(crate) handle: WindowHandle<DruidHandler<D>>,
    ////prev_paint_time: Option<Instant>,
}

/// Everything required for a window to handle an event.
struct SingleWindowState<T: Data + 'static + Default> { ////
////struct SingleWindowState<'a, T: Data> {
    window_id: WindowId,
    phantom: PhantomData<T>,  ////  Needed to do compile-time checking for `Data`
    ////window: &'a mut Window<T>,
    ////state: &'a mut WindowState<T>,
    ////command_queue: &'a mut VecDeque<(WindowId, Command)>,
    ////data: &'a mut T, //// Replaced by ALL_DATA
    ////env: &'a Env, //// Replaced by Env{}
}

impl<T: Data + 'static + Default> Windows<T> { ////
////impl<T: Data> Windows<T> {
    #[allow(dead_code)] ////
    fn connect(&mut self, _id: WindowId, _handle: WindowHandle<DruidHandler<T>>) { ////
    ////fn connect(&mut self, id: WindowId, handle: WindowHandle) {
        ////AppState::<T>::new().add_handler(id, handle.0); //// TODO1
        /* ////
        let state = WindowState {
            handle,
            ////prev_paint_time: None,
        };
        self.state.insert(id, state);
        */ ////
    }

    #[allow(dead_code)] ////
    fn add(&mut self, id: WindowId, window: WindowBox<T>) { ////
    ////fn add(&mut self, id: WindowId, window: Window<T>) {
        AppState::<T>::new().add_window(id, window); ////
        ////self.windows.insert(id, window);
    }

    #[allow(dead_code)] ////
    fn remove(&mut self, _id: WindowId) -> Option<WindowHandle<DruidHandler<T>>> { ////
    ////fn remove(&mut self, id: WindowId) -> Option<WindowHandle> {
        ////self.windows.remove(&id);
        ////self.state.remove(&id).map(|state| state.handle)
        ////Some(self.state[0].handle) ////
        None //// TODO
    }

    //TODO: rename me?
    #[allow(dead_code)] ////
    fn get<'a>( ////
        &'a mut self,
        window_id: WindowId,
        ////command_queue: &'a mut VecDeque<(WindowId, Command)>,
        _data: &'a mut T,
        _env: &'a Env,
    ) -> Option<SingleWindowState<T>> { ////
    ////) -> Option<SingleWindowState<'a, T>> {        
        ////let state = self.state.get_mut(&window_id);
        ////let window = self.windows.get_mut(&window_id);
        Some( SingleWindowState { ////
            window_id,
            phantom: PhantomData,
        })
        /* ////
            match (self.state, self.windows) { ////
            ////match (state, window) {
                (Some(mut state), Some(mut window)) => { ////
                ////(Some(state), Some(window)) => {
                    return Some(SingleWindowState {
                        window_id,
                        window: &mut window, ////
                        ////window,
                        state: &mut state, ////
                        ////state,
                        ////command_queue,
                        data,
                        env,
                    })
                }
                (None, Some(_)) => {} ////warn!("missing window for id {:?}", window_id),
                (Some(_), None) => {} ////warn!("missing state for window id {:?}", window_id),
                (None, None) => {} ////warn!("unknown window {:?}", window_id),
            }
            None
        */ ////
    }
}

impl<T: Data + 'static + Default> SingleWindowState<T> { ////
////impl<T: Data + 'static> SingleWindowState<T> {
    fn paint(&mut self, piet: &mut Piet, _ctx: &mut dyn WinCtx) -> bool {
        ////let request_anim = self.do_anim_frame(ctx);
        self.do_layout(piet);
        piet.clear(crate::env::WINDOW_BACKGROUND_COLOR); ////
        ////piet.clear(self.env.get(theme::WINDOW_BACKGROUND_COLOR));
        self.do_paint(piet);
        false ////
        ////request_anim
    }

    /* ////
        fn do_anim_frame(&mut self, ctx: &mut dyn WinCtx) -> bool {
            // TODO: this calculation uses wall-clock time of the paint call, which
            // potentially has jitter.
            //
            // See https://github.com/xi-editor/druid/issues/85 for discussion.
            let this_paint_time = Instant::now();
            let prev_paint_time = self.state.prev_paint_time;
            let interval = if let Some(last) = prev_paint_time {
                let duration = this_paint_time.duration_since(last);
                1_000_000_000 * duration.as_secs() + u64::from(duration.subsec_nanos())
            } else {
                0
            };
            let anim_frame_event = Event::AnimFrame(interval);
            let (_, _, request_anim) = self.do_event_inner(anim_frame_event, ctx);
            let prev = if request_anim {
                Some(this_paint_time)
            } else {
                None
            };
            self.state.prev_paint_time = prev;
            request_anim
        }
    */ ////

    fn do_layout(&mut self, piet: &mut Piet) {
        let mut layout_ctx = LayoutCtx {
            text_factory: piet.text(),
            window_id: self.window_id,
        };
        AppState::<T>::new().window_layout(self.window_id, &mut layout_ctx); ////
        ////self.window.layout(&mut layout_ctx, self.data, self.env);
    }

    fn do_paint(&mut self, piet: &mut Piet) {
        let mut paint_ctx = PaintCtx {
            render_ctx: piet,
            window_id: self.window_id,
            region: Rect::ZERO.into(),
        };
        AppState::<T>::new().window_paint(self.window_id, &mut paint_ctx); ////
        ////self.window.paint(&mut paint_ctx, self.data, self.env);
    }

    /// Send an event to the widget hierarchy.
    ///
    /// Returns three flags. The first is true if the event was handled. The
    /// second is true if invalidation is requested. The third is true if an
    /// animation frame is requested.
    fn do_event_inner(&mut self, event: Event, win_ctx: &mut dyn WinCtx) -> (bool, bool, bool) {
        // should there be a root base state persisting in the ui state instead?
        //cortex_m::asm::bkpt(); ////
        let _cursor = match event {
            Event::MouseMoved(..) => Some(Cursor::Arrow),
            _ => None,
        };

        /* ////
        let event = match event {
            Event::Size(size) => {
                let dpi = f64::from(self.state.handle.get_dpi());
                let scale = 96.0 / dpi;
                Event::Size(Size::new(size.width * scale, size.height * scale))
            }
            other => other,
        };
        */ ////

        let mut base_state = BaseState::default();
        let mut ctx = EventCtx {
            win_ctx,
            ////cursor: &mut cursor,
            ////command_queue: self.command_queue,
            base_state: &mut base_state,
            is_handled: false,
            is_root: true,
            had_active: false, ////TODO self.window.has_active(),
            ////had_active: self.window.root.state.has_active,
            window: &AppState::<T>::new().get_handle(self.window_id),
            ////window: &self.state.handle,
            window_id: self.window_id,
        };
        AppState::<T>::new().window_event(self.window_id, &mut ctx, &event); ////
        ////self.window.event(&mut ctx, &event, self.data, self.env);

        let is_handled = ctx.is_handled;
        if ctx.base_state.request_focus {
            let focus_event = Event::FocusChanged(true);
            AppState::<T>::new().window_event(self.window_id, &mut ctx, &focus_event); ////
            ////self.window
                ////.event(&mut ctx, &focus_event, self.data, self.env);
        }
        let needs_inval = ctx.base_state.needs_inval;
        let request_anim = ctx.base_state.request_anim;
        /* ////
            if let Some(cursor) = cursor {
                win_ctx.set_cursor(&cursor);
            }
        */ ////

        (is_handled, needs_inval, request_anim)
    }

    /* ////
        fn set_menu(&mut self, cmd: &Command) {
            let mut menu = match cmd.get_object::<MenuDesc<T>>() {
                Some(menu) => menu.to_owned(),
                None => {
                    warn!("set-menu command is missing menu object");
                    return;
                }
            };

            let platform_menu = menu.build_window_menu(&self.data, &self.env);
            self.state.handle.set_menu(platform_menu);
            self.window.menu = Some(menu.to_owned());
        }

        fn show_context_menu(&mut self, cmd: &Command) {
            let (mut menu, point) = match cmd.get_object::<ContextMenu<T>>() {
                Some(ContextMenu { menu, location }) => (menu.to_owned(), *location),
                None => {
                    warn!("show-context-menu command is missing menu object.");
                    return;
                }
            };
            let platform_menu = menu.build_popup_menu(&self.data, &self.env);
            self.state.handle.show_context_menu(platform_menu, point);
            self.window.context_menu = Some(menu);
        }
    */ ////

    fn window_got_focus(&mut self) {
        #[cfg(target_os = "macos")]
        self.macos_update_app_menu()
    }

    /// On macos we need to update the global application menu to be the menu
    /// for the current window.
    #[cfg(target_os = "macos")]
    fn macos_update_app_menu(&mut self) {
        let SingleWindowState {
            window,
            state,
            data,
            env,
            ..
        } = self;
        let platform_menu = window
            .menu
            .as_mut()
            .map(|m| m.build_window_menu(&data, &env));
        if let Some(platform_menu) = platform_menu {
            state.handle.set_menu(platform_menu);
        }
    }
}

impl<T: Data + 'static + Default> AppState<T> { ////
////impl<T: Data + 'static> AppState<T> {
    pub(crate) fn new(
        ////data: T,
        ////env: Env,
        ////delegate: Option<Box<dyn AppDelegate<T>>>,
    ) -> Self { ////
    ////) -> Rc<RefCell<Self>> {
        AppState { ////
            phantom: PhantomData, ////
        ////Rc::new(RefCell::new(AppState {
            ////delegate,
            ////command_queue: VecDeque::new(),
            ////data,
            ////env,
            ////windows: Windows::default(),
        } ////))
    }

    /* ////
        fn get_menu_cmd(&self, window_id: WindowId, cmd_id: u32) -> Option<Command> {
            self.windows
                .windows
                .get(&window_id)
                .and_then(|w| w.get_menu_cmd(cmd_id))
        }

        /// A helper fn for setting up the `DelegateCtx`. Takes a closure with
        /// an arbitrary return type `R`, and returns `Some(R)` if an `AppDelegate`
        /// is configured.
        fn with_delegate<R, F>(&mut self, id: WindowId, f: F) -> Option<R>
        where
            F: FnOnce(&mut Box<dyn AppDelegate<T>>, &mut T, &Env, &mut DelegateCtx) -> R,
        {
            let AppState {
                ref mut delegate,
                ref mut command_queue,
                ref mut data,
                ref env,
                ..
            } = self;
            let mut ctx = DelegateCtx {
                source_id: id,
                command_queue,
            };
            if let Some(delegate) = delegate {
                Some(f(delegate, data, env, &mut ctx))
            } else {
                None
            }
        }

        fn delegate_event(&mut self, id: WindowId, event: Event) -> Option<Event> {
            if self.delegate.is_some() {
                self.with_delegate(id, |del, data, env, ctx| del.event(event, data, env, ctx))
                    .unwrap()
            } else {
                Some(event)
            }
        }

        fn connect(&mut self, id: WindowId, handle: WindowHandle) {
            self.windows.connect(id, handle);
            self.with_delegate(id, |del, data, env, ctx| {
                del.window_added(id, data, env, ctx)
            });
        }
    */ ////

    /* Already implemented in GlobalWindows
    pub(crate) fn add_window(&mut self, id: WindowId, window: WindowBox<T>) { ////
    ////pub(crate) fn add_window(&mut self, id: WindowId, window: Window<T>) {
        self.add_window(id, window); ////
        ////self.windows.add(id, window);
    }
    */
    
    #[allow(dead_code)] ////
    fn remove_window(&mut self, _id: WindowId) -> Option<WindowHandle<DruidHandler<T>>> { ////
    ////fn remove_window(&mut self, id: WindowId) -> Option<WindowHandle> {
        None //// TODO
        /* ////
            let res = self.windows.remove(id);
            self.with_delegate(id, |del, data, env, ctx| {
                del.window_removed(id, data, env, ctx)
            });
            res
        */ ////
    }

    #[allow(dead_code)] ////
    fn show_window(&mut self, _id: WindowId) {
        ////if let Some(state) = self.windows.state.get(&id) {
            ////TODO1 state.handle.bring_to_front_and_focus();
        ////}
    }

    fn assemble_window_state(&mut self, window_id: WindowId) -> Option<SingleWindowState<T>> { ////
    ////fn assemble_window_state(&mut self, window_id: WindowId) -> Option<SingleWindowState<'_, T>> {
        Some( SingleWindowState { 
            window_id, 
            phantom: PhantomData,
        } )
        /* ////
            let AppState {
                ref mut command_queue,
                ref mut windows,
                ref mut data,
                ref env,
                ..
            } = self;
            windows.get(window_id, command_queue, data, env)
        */ ////
    }

    fn paint(&mut self, window_id: WindowId, piet: &mut Piet, ctx: &mut dyn WinCtx) -> bool {
        self.assemble_window_state(window_id)
            .map(|mut win| win.paint(piet, ctx))
            .unwrap_or(false)
    }

    fn do_event(&mut self, source_id: WindowId, event: Event, win_ctx: &mut dyn WinCtx) -> bool {
        ////let event = self.delegate_event(source_id, event);

        let (_is_handled, _dirty, _anim) = { ////
        ////let (is_handled, dirty, anim) = if let Some(event) = event {
            /* ////
            // handle system window-level commands
            if let Event::Command(ref cmd) = event {
                match cmd.selector {
                    sys_cmd::SET_MENU => {
                        if let Some(mut win) = self.assemble_window_state(source_id) {
                            win.set_menu(cmd);
                        }
                        return true;
                    }
                    sys_cmd::SHOW_CONTEXT_MENU => {
                        if let Some(mut win) = self.assemble_window_state(source_id) {
                            win.show_context_menu(cmd);
                        }
                        return true;
                    }
                    _ => (),
                }
            }
            */ ////

            self.assemble_window_state(source_id)
                .map(|mut win| win.do_event_inner(event, win_ctx))
                .unwrap_or((false, false, false))
        };
        /* ////
        else {
            // if the event was swallowed by the delegate we consider it handled?
            (true, false, false)
        };
        */ ////

        let window_id = WindowId(1);  ////  Assume Window ID 1 is the first and only window
        let mut update_ctx = UpdateCtx { ////
            text_factory: win_ctx.text_factory(),
            window: &self.get_handle(window_id),
            needs_inval: false,
            window_id,
        };
        self.window_update(window_id, &mut update_ctx); ////
        //cortex_m::asm::bkpt(); ////
        if update_ctx.needs_inval { ////
            update_ctx.window.invalidate();
        }
        true

        /* ////
        let AppState {
            ref mut windows,
            ref data,
            ref env,
            ..
        } = self;
        let Windows { state, windows } = windows;
        // we send `update` to all windows, not just the active one:
        for (id, window) in windows {
            if let Some(state) = state.get(id) {
                let mut update_ctx = UpdateCtx {
                    text_factory: win_ctx.text_factory(),
                    window: &state.handle,
                    needs_inval: false,
                    window_id: *id,
                };
                window.update(&mut update_ctx, data, env);
                if update_ctx.needs_inval || (*id == source_id && (anim || dirty)) {
                    update_ctx.window.invalidate();
                }
            }
        }
        is_handled
        */ ////
    }

    fn window_got_focus(&mut self, window_id: WindowId, _ctx: &mut dyn WinCtx) {
        self.assemble_window_state(window_id)
            .as_mut()
            .map(SingleWindowState::window_got_focus);
    }

}

impl<T: Data + 'static + Default> DruidHandler<T> { ////
////impl<T: Data + 'static> DruidHandler<T> {
    /// Note: the root widget doesn't go in here, because it gets added to the
    /// app state.
    pub(crate) fn new_shared(
        ////app_state_U32: Rc<RefCell<AppState<T>>>,
        window_id: WindowId,
    ) -> DruidHandler<T> { ////
    ////) -> DruidHandler<T> {
        DruidHandler {
            window_id,
            phantom: PhantomData, ////
        }
    }

    /// Send an event to the widget hierarchy.
    ///
    /// Returns `true` if the event produced an action.
    ///
    /// This is principally because in certain cases (such as keydown on Windows)
    /// the OS needs to know if an event was handled.
    fn do_event(&mut self, event: Event, win_ctx: &mut dyn WinCtx) -> bool {
        AppState::<T>::new().do_event(self.window_id, event, win_ctx) ////
        /* ////
            let result = self
                .app_state
                .borrow_mut()
                .do_event(self.window_id, event, win_ctx);
            self.process_commands(win_ctx);
            result
        */ ////
    }

    /* ////
        fn process_commands(&mut self, win_ctx: &mut dyn WinCtx) {
            loop {
                let next_cmd = self.app_state_U32.borrow_mut().command_queue.pop_front();
                match next_cmd {
                    Some((id, cmd)) => self.handle_cmd(id, cmd, win_ctx),
                    None => break,
                }
            }
        }

        fn handle_system_cmd(&mut self, cmd_id: u32, win_ctx: &mut dyn WinCtx) {
            let cmd = self.app_state_U32.borrow().get_menu_cmd(self.window_id, cmd_id);
            match cmd {
                Some(cmd) => self
                    .app_state_U32
                    .borrow_mut()
                    .command_queue
                    .push_back((self.window_id, cmd)),
                None => warn!("No command for menu id {}", cmd_id),
            }
            self.process_commands(win_ctx)
        }

        /// Handle a command. Top level commands (e.g. for creating and destroying windows)
        /// have their logic here; other commands are passed to the window.
        fn handle_cmd(&mut self, window_id: WindowId, cmd: Command, win_ctx: &mut dyn WinCtx) {
            //FIXME: we need some way of getting the correct `WinCtx` for this window.
            match &cmd.selector {
                &sys_cmd::OPEN_FILE => self.open_file(cmd, window_id, win_ctx),
                &sys_cmd::NEW_WINDOW => self.new_window(cmd),
                &sys_cmd::CLOSE_WINDOW => self.close_window(cmd, window_id),
                &sys_cmd::SHOW_WINDOW => self.show_window(cmd),
                &sys_cmd::QUIT_APP => self.quit(),
                &sys_cmd::HIDE_APPLICATION => self.hide_app(),
                &sys_cmd::HIDE_OTHERS => self.hide_others(),
                &sys_cmd::PASTE => self.do_paste(window_id, win_ctx),
                sel => {
                    info!("handle_cmd {}", sel);
                    let event = Event::Command(cmd);
                    self.app_state_U32
                        .borrow_mut()
                        .do_event(window_id, event, win_ctx);
                }
            }
        }

        fn open_file(&mut self, cmd: Command, window_id: WindowId, win_ctx: &mut dyn WinCtx) {
            let options = cmd
                .get_object::<FileDialogOptions>()
                .map(|opts| opts.to_owned())
                .unwrap_or_default();
            let result = win_ctx.open_file_sync(options);
            if let Some(info) = result {
                let event = Event::OpenFile(info);
                self.app_state_U32
                    .borrow_mut()
                    .do_event(window_id, event, win_ctx);
            }
        }

        fn new_window(&mut self /*, cmd: Command*/) { ////
            let desc = match cmd.get_object::<WindowDesc<T>>() {
                Some(wd) => wd,
                None => {
                    ////warn!("new_window command is missing window description");
                    return;
                }
            };

            let window = match desc.build_native(&self.app_state_U32) {
                Ok(win) => win,
                Err(e) => {
                    ////error!("failed to create window: '{:?}'", e);
                    return;
                }
            };
            window.show();
        }

        fn close_window(&mut self, /* cmd: Command, */ window_id: WindowId) { ////
            let id = cmd.get_object().unwrap_or(&window_id);
            let handle = self.app_state_U32.remove_window(*id); ////
            ////let handle = self.app_state_U32.borrow_mut().remove_window(*id);
            if let Some(handle) = handle {
                handle.close();
            }
        }

        fn show_window(&mut self, cmd: Command) {
            let id: WindowId = *cmd
                .get_object()
                .expect("show window selector missing window id");
            self.app_state_U32.borrow_mut().show_window(id);
        }

        fn do_paste(&mut self, window_id: WindowId, ctx: &mut dyn WinCtx) {
            let event = Event::Paste(Application::clipboard());
            self.app_state_U32.borrow_mut().do_event(window_id, event, ctx);
        }

        fn quit(&self) {
            Application::quit()
        }

        fn hide_app(&self) {
            #[cfg(all(target_os = "macos", not(feature = "use_gtk")))]
            Application::hide()
        }

        fn hide_others(&mut self) {
            #[cfg(all(target_os = "macos", not(feature = "use_gtk")))]
            Application::hide_others()
        }
    */ ////
}

impl<T: Data + 'static + Default> WinHandler<DruidHandler<T>> for DruidHandler<T> { ////
////impl<T: Data + 'static> WinHandler for DruidHandler<T> {
    /* //// TODO1
    fn connect(&mut self, handle: &WindowHandle<DruidHandler<T>>) { ////
    ////fn connect(&mut self, handle: &WindowHandle) {
        AppState::<T>::new().connect(self.window_id, handle.clone()); ////
        ////self.app_state_U32
            ////.borrow_mut()
            ////.connect(self.window_id, handle.clone());
    }
    */ ////

    fn paint(&mut self, piet: &mut Piet, ctx: &mut dyn WinCtx) -> bool {
        AppState::<T>::new().paint(self.window_id, piet, ctx) ////
        ////self.app_state_U32.borrow_mut().paint(self.window_id, piet, ctx)
    }

    fn size(&mut self, width: u32, height: u32, ctx: &mut dyn WinCtx) {
        let event = Event::Size(Size::new(f64::from(width), f64::from(height)));
        self.do_event(event, ctx);
    }

    /* ////
        fn command(&mut self, id: u32, ctx: &mut dyn WinCtx) {
            self.handle_system_cmd(id, ctx);
        }
    */ ////

    fn mouse_down(&mut self, event: &MouseEvent, ctx: &mut dyn WinCtx) {
        // TODO: double-click detection (or is this done in druid-shell?)
        let event = Event::MouseDown(event.clone().into());
        self.do_event(event, ctx);
    }

    fn mouse_up(&mut self, event: &MouseEvent, ctx: &mut dyn WinCtx) {
        ////TODO
        let event = Event::MouseUp(event.clone().into());
        self.do_event(event, ctx);
    }

    fn mouse_move(&mut self, _event: &MouseEvent, _ctx: &mut dyn WinCtx) {
        ////TODO
        ////let event = Event::MouseMoved(event.clone().into());
        ////self.do_event(event, ctx);
    }

    /* ////
        fn key_down(&mut self, event: KeyEvent, ctx: &mut dyn WinCtx) -> bool {
            self.do_event(Event::KeyDown(event), ctx)
        }

        fn key_up(&mut self, event: KeyEvent, ctx: &mut dyn WinCtx) {
            self.do_event(Event::KeyUp(event), ctx);
        }

        fn wheel(&mut self, delta: Vec2, mods: KeyModifiers, ctx: &mut dyn WinCtx) {
            let event = Event::Wheel(WheelEvent { delta, mods });
            self.do_event(event, ctx);
        }
    */ ////

    fn got_focus(&mut self, ctx: &mut dyn WinCtx) {
        AppState::<T>::new().window_got_focus(self.window_id, ctx); ////
        /* ////
            self.app_state
                .borrow_mut()
                .window_got_focus(self.window_id, ctx);
        */ ////
    }

    fn get_window_id(&self) -> WindowIdType { ////
        self.window_id.0
    }

    fn add_handler(&self, window_id: WindowIdType, handler: DruidHandler<T>) { ////
        AppState::<T>::new().add_handler(WindowId(window_id), handler);
    }

    /* ////
        fn timer(&mut self, token: TimerToken, ctx: &mut dyn WinCtx) {
            self.do_event(Event::Timer(token), ctx);
        }

        fn as_any(&mut self) -> &mut dyn Any {
            self
        }
    */ ////
}

impl<T: Data + 'static + Default> Default for Windows<T> { ////
////impl<T: Data> Default for Windows<T> {
    fn default() -> Self {
        Windows {
            phantom: PhantomData, ////
            ////windows: HashMap::new(),
            ////state: HashMap::new(),
        }
    }
}
