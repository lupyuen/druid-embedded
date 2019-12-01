//! `WidgetBox` contains a `Widget`. Allows for dynamic dispatch with static `Widgets` in `[no_std]`.

use core::marker::PhantomData;
use crate::kurbo::{Rect, Size};
use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
    widget::{Button, Flex, Label},
};

/// Boxed version of a `Widget`
pub struct WidgetBox<D: Data + 'static, W: Widget<D>>(
    W, 
    PhantomData<D>,  //  Needed to do compile-time checking for `Data`
);

/// Implementation of `WidgetBox` for specific `Widgets`
impl<D: Data + 'static> WidgetBox<D, Button<D>> {}
impl<D: Data + 'static> WidgetBox<D, Flex<D>> {}
impl<D: Data + 'static> WidgetBox<D, Label<D>> {}

/// Generic implementation of `WidgetBox`
impl<D: Data + 'static, W: Widget<D>> WidgetBox<D, W> {
    /// Create a new box for the `Widget`
    pub fn new(widget: W) -> Self {
        WidgetBox(
            widget,
            PhantomData,
        )
    }
}

/// Implementation of `Widget` trait for `WidgetBox`. We just forward to the inner `Widget`.
impl<D: Data + 'static, W: Widget<D>> Widget<D> for WidgetBox<D, W> {
    fn paint(
        &mut self, 
        paint_ctx: &mut PaintCtx, 
        base_state: &BaseState, 
        data: &D, 
        env: &Env
    ) {
        self.0.paint(paint_ctx, base_state, data, env);
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &D,
        env: &Env,
    ) -> Size {
        self.0.layout(layout_ctx, bc, data, env)
    }

    fn event(
        &mut self, 
        ctx: &mut EventCtx, 
        event: &Event, 
        data: &mut D, 
        env: &Env
    ) {
        self.0.event(ctx, event, data, env);
    }

    fn update(
        &mut self, 
        ctx: &mut UpdateCtx, 
        old_data: Option<&D>, 
        data: &D, 
        env: &Env
    ) {
        self.0.update(ctx, old_data, data, env);
    }
}