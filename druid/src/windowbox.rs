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
    pub WindowType<D>,
    //pub PhantomData<D>,  //  Needed to do compile-time checking for `Data`
);

/// Enum to store each `Window`
#[derive(Clone)]
pub enum WindowType<D: Data + 'static> {
    None,
    Flex(Window<D, Flex<D>>),
}

impl<D: Data + 'static + Default> Default for WindowType<D> {
    fn default() -> Self { WindowType::None }
}

/// Generic implementation of `WindowBox`
impl<D: Data + 'static> WindowBox<D> {
    /// Create a new empty box without a `Window`
    pub fn new() -> Self {
        WindowBox(
            WindowType::None,
            //PhantomData,
        )
    }
}

/// Implementation of `WindowBox`. We just forward to the inner `Window`.
impl<D: Data + 'static> WindowBox<D> {
    pub fn event(
        &mut self, 
        ctx: &mut EventCtx<D>, 
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
        ctx: &mut UpdateCtx<D>, 
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
        &mut self,
    ) -> bool {
        match &mut self.0 {
            WindowType::Flex(w)   => w.root.state.has_active,
            WindowType::None => false
        }        
    }
}