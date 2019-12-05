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

//! Management of multiple windows.

////use std::sync::atomic::{AtomicU32, Ordering};

use core::marker::PhantomData; ////
use crate::kurbo::{Point, Rect, Size};

use crate::shell::WindowHandle;
use crate::{
    BoxConstraints, /* Command, */ Data, Env, Event, EventCtx, LayoutCtx, LocalizedString, /* MenuDesc, */ ////
    PaintCtx, UpdateCtx, Widget, WidgetPod,
    widget::WidgetBox, WindowBox, WindowType, ////
};

/// A unique identifier for a window.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WindowId(u32);

static mut WINDOW_ID_COUNTER: u32 = 1; ////
////static WINDOW_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

/// Per-window state not owned by user code.
#[derive(Clone, Copy, Default)] ////
pub struct Window<T: Data + 'static + Default, W: Widget<T> + 'static> { ////
////pub struct Window<T: Data> {
    pub(crate) root: WidgetPod<T, W>, ////
    ////pub(crate) root: WidgetPod<T, Box<dyn Widget<T>>>,
    ////pub(crate) title: LocalizedString<T>,
    size: Size,
    ////pub(crate) menu: Option<MenuDesc<T>>,
    ////pub(crate) context_menu: Option<MenuDesc<T>>,
    phantom_data: PhantomData<T>,  ////  Needed to do compile-time checking for `Data`
    // delegate?
}

impl<T: Data + 'static + Default, W: Widget<T> + 'static> Window<T, W> { ////
////impl<T: Data> Window<T> {
    pub fn new(
        root: W,
        ////root: &dyn Widget<T>,
        ////title: LocalizedString<T>,
        ////menu: Option<MenuDesc<T>>,
    ) -> Self {
        Window {
            root: WidgetPod::new(root), ////
            ////root: WidgetPod::new(WidgetBox::new(root)), ////
            ////root: WidgetPod::new(Box::new(root)),
            size: Size::ZERO,
            ////title,
            ////menu,
            ////context_menu: None,
            phantom_data: PhantomData, ////
        }
    }

    pub fn event(&mut self, ctx: &mut EventCtx<T>, event: &Event, data: &mut T, env: &Env) { ////
    ////pub fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::Size(size) = event {
            self.size = *size;
        }
        self.root.event(ctx, event, data, env);

        /* ////
        if let Some(cursor) = ctx.cursor {
            ctx.win_ctx.set_cursor(&cursor);
        }
        */ ////
    }

    pub fn update(&mut self, update_ctx: &mut UpdateCtx<T>, data: &T, env: &Env) {
    ////pub fn update(&mut self, update_ctx: &mut UpdateCtx, data: &T, env: &Env) {
        ////self.update_title(&update_ctx.window, data, env);
        self.root.update(update_ctx, data, env);
    }

    pub fn layout(&mut self, layout_ctx: &mut LayoutCtx, data: &T, env: &Env) {
        let bc = BoxConstraints::tight(self.size);
        let size = self.root.layout(layout_ctx, &bc, data, env);
        self.root
            .set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
    }

    pub fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        let visible = Rect::from_origin_size(Point::ZERO, self.size);
        paint_ctx.with_child_ctx(visible, |ctx| self.root.paint(ctx, data, env));
    }

    /* ////
    pub(crate) fn update_title(&mut self, win_handle: &WindowHandle, data: &T, env: &Env) {
        if self.title.resolve(data, env) {
            win_handle.set_title(self.title.localized_str());
        }
    }

    pub(crate) fn get_menu_cmd(&self, cmd_id: u32) -> Option<Command> {
        self.context_menu
            .as_ref()
            .and_then(|m| m.command_for_id(cmd_id))
            .or_else(|| self.menu.as_ref().and_then(|m| m.command_for_id(cmd_id)))
    }
    */ ////

    /*
    pub fn to_type(&mut self) -> WindowType<T> { ////
        W::to_window_type(self.clone)
    }
    */
}

impl WindowId {
    /// Allocate a new, unique window id.
    ///
    /// Do note that if we create 4 billion windows there may be a collision.
    pub fn next() -> WindowId {
        let id = unsafe { WINDOW_ID_COUNTER }; ////
        unsafe { WINDOW_ID_COUNTER += 1 }; ////    
        ////let id = WINDOW_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        WindowId(id)
    }
}
