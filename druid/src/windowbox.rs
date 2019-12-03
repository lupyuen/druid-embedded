//! `WindowBox` contains a `Window`. Allows for dynamic dispatch with static `Windows` in `[no_std]`.

use core::marker::PhantomData;
use crate::kurbo::{Rect, Size};
use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Window, Widget,
    widget::{Button, Flex, Label},
};

/// Boxed version of a `Window`
#[derive(Clone, Default)]
pub struct WindowBox<D: Data + 'static>(
    WindowType<D>,
    PhantomData<D>,  //  Needed to do compile-time checking for `Data`
);

/// Enum to store each `Window`
#[derive(Clone)]
pub enum WindowType<D: Data + 'static> {
    None,
    Flex(Window<D, Flex<D>>),
}

impl<D: Data + 'static> Default for WindowType<D> {
    fn default() -> Self { WindowType::None }
}

/*
pub fn widget_to_window_type<D: Data + 'static, W: Widget<D>>(window: &mut Window<D, W>) -> WindowType<D> {
    WindowType::None
}        

pub fn widget_to_window_type<D: Data + 'static, W>(window: &mut Window<D, W>) -> WindowType<D>
    where W: Flex<D> {
    WindowType::Flex(*window)
}        
*/

pub trait WindowTrait<D: Data + 'static> {
    /// Wrap this `Window` in a `WindowType` enum for boxing by `WindowBox`
    fn to_type() -> WindowType<T>;
}

impl<D: Data + 'static, W: Widget<D> + 'static> WindowTrait<D> for Window<D, W> {
    fn to_type() -> WindowType<T> {
        WindowType::Flex(*window)
    }
}

/// Generic implementation of `WindowBox`
impl<D: Data + 'static> WindowBox<D> {
    /// Create a new box for the `Window`
    pub fn new(window: &mut Window<D, Flex<D>>) -> Self {
    ////TODO pub fn new<W: Widget<D>>(window: &mut Window<D, W>) -> Self {
        WindowBox(
            WindowType::Flex(*window), ////TODO: window.to_type(),
            ////widget.to_type(),
            ////widget_to_window_type::<D, W>(window),
            PhantomData,
        )
    }
}

/// Implementation of `WindowBox`. We just forward to the inner `Window`.
impl<D: Data + 'static> WindowBox<D> {
    pub fn event(
        &mut self, 
        ctx: &mut EventCtx, 
        event: &Event, 
        data: &mut D, 
        env: &Env
    ) {
        match &mut self.0 {
            WindowType::Flex(w)   => w.event(ctx, event, data, env),
            WindowType::None => {}
        };
    }

    pub fn update(
        &mut self, 
        ctx: &mut UpdateCtx, 
        data: &D, 
        env: &Env
    ) {
        match &mut self.0 {
            WindowType::Flex(w)   => w.update(ctx, data, env),
            WindowType::None => {}
        };
    }

    pub fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        data: &D,
        env: &Env,
    ) {
        match &mut self.0 {
            WindowType::Flex(w)   => w.layout(layout_ctx, data, env),
            WindowType::None => {}
        };
    }

    pub fn paint(
        &mut self, 
        paint_ctx: &mut PaintCtx, 
        data: &D, 
        env: &Env
    ) {
        match &mut self.0 {
            WindowType::Flex(w)   => w.paint(paint_ctx, data, env),
            WindowType::None => {}
        };
    }

    pub fn has_active(
        self,
    ) -> bool {
        match self.0 {
            WindowType::Flex(w)   => w.root.state.has_active,
            WindowType::None => false
        }        
    }
}