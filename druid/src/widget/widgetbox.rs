//! `WidgetBox` contains a `Widget`. Allows for dynamic dispatch with static `Widgets` for `[no_std]`.

use crate::kurbo::{Point, Rect, Size}; ////

use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
    WidgetPod,
    widget::{Button, Flex, Label},
};

/// Boxed version of a `Widget`
pub struct WidgetBox<D: Data> {
    pub widget: WidgetType<D>,
}

/// Enum for a `Widget`
pub enum WidgetType<D: Data> {
    Button(Button<D>),
    Flex(Flex<D>),
    Label(Label<D>),
}

/// Generic implementation of `WidgetBox`
impl<D: Data> WidgetBox<D> {
    /// Create a new box for the `Widget`
    pub fn new(widget: W) -> Self {
        WidgetBox(
            widget,
            PhantomData,
        )
    }
}

/// Implementation of `Widget` trait for `WidgetBox`. We just forward to the inner `Widget`.
impl<D: Data> Widget<D> for WidgetBox<D> {
    fn paint(
        &mut self, 
        paint_ctx: &mut PaintCtx, 
        base_state: &BaseState, 
        data: &D, 
        env: &Env
    ) {
        match &mut self.widget {
            WidgetType::Button(w) => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Flex(w)   => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Label(w)  => w.paint(paint_ctx, base_state, data, env),
        };
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &D,
        env: &Env,
    ) -> Size {
        match &mut self.widget {
            WidgetType::Button(w) => w.layout(layout_ctx, bc, data, env),
            WidgetType::Flex(w)   => w.layout(layout_ctx, bc, data, env),
            WidgetType::Label(w)  => w.layout(layout_ctx, bc, data, env),
        }
    }

    fn event(
        &mut self, 
        ctx: &mut EventCtx, 
        event: &Event, 
        data: &mut D, 
        env: &Env
    ) {
        match &mut self.widget {
            WidgetType::Button(w) => w.event(ctx, event, data, env),
            WidgetType::Flex(w)   => w.event(ctx, event, data, env),
            WidgetType::Label(w)  => w.event(ctx, event, data, env),
        };
    }

    fn update(
        &mut self, 
        ctx: &mut UpdateCtx, 
        old_data: Option<&D>, 
        data: &D, 
        env: &Env
    ) {
        match &mut self.widget {
            WidgetType::Button(w) => w.update(ctx, old_data, data, env),
            WidgetType::Flex(w)   => w.update(ctx, old_data, data, env),
            WidgetType::Label(w)  => w.update(ctx, old_data, data, env),
        };
    }
}