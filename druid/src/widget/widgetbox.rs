//! `WidgetBox` contains a `Widget`. Allows for dynamic dispatch with static `Widgets` in `[no_std]`.

use core::marker::PhantomData;
use crate::kurbo::{Rect, Size};
use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
    widget::{Button, Flex, Label},
};

/// Boxed version of a `Widget`
#[derive(Clone, Copy)]
pub struct WidgetBox<D: Data + 'static>(
    WidgetType<D>,
    PhantomData<D>,  //  Needed to do compile-time checking for `Data`
);

/// Enum to store each `Widget`
#[derive(Clone, Copy)]
pub enum WidgetType<D: Data + 'static> {
    None,
    Button(Button<D>),
    Flex(Flex<D>),
    Label(Label<D>),
}

/// Generic implementation of `WidgetBox`
impl<D: Data + 'static> WidgetBox<D> {
    /// Create a new box for the `Widget`
    pub fn new<W: Widget<D>>(widget: &mut W) -> Self {
        WidgetBox(
            widget.to_type(),
            PhantomData,
        )
    }
}

/// Implementation of `Widget` trait for `WidgetBox`. We just forward to the inner `Widget`.
impl<D: Data + 'static> Widget<D> for WidgetBox<D> {
    fn paint(
        &mut self, 
        paint_ctx: &mut PaintCtx, 
        base_state: &BaseState, 
        data: &D, 
        env: &Env
    ) {
        match &mut self.0 {
            WidgetType::Button(w) => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Flex(w)   => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Label(w)  => w.paint(paint_ctx, base_state, data, env),
            WidgetType::None => {}
        };
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &D,
        env: &Env,
    ) -> Size {
        match &mut self.0 {
            WidgetType::Button(w) => w.layout(layout_ctx, bc, data, env),
            WidgetType::Flex(w)   => w.layout(layout_ctx, bc, data, env),
            WidgetType::Label(w)  => w.layout(layout_ctx, bc, data, env),
            WidgetType::None => Size::ZERO,
        }
    }

    fn event(
        &mut self, 
        ctx: &mut EventCtx, 
        event: &Event, 
        data: &mut D, 
        env: &Env
    ) {
        match &mut self.0 {
            WidgetType::Button(w) => w.event(ctx, event, data, env),
            WidgetType::Flex(w)   => w.event(ctx, event, data, env),
            WidgetType::Label(w)  => w.event(ctx, event, data, env),
            WidgetType::None => {}
        };
    }

    fn update(
        &mut self, 
        ctx: &mut UpdateCtx, 
        old_data: Option<&D>, 
        data: &D, 
        env: &Env
    ) {
        match &mut self.0 {
            WidgetType::Button(w) => w.update(ctx, old_data, data, env),
            WidgetType::Flex(w)   => w.update(ctx, old_data, data, env),
            WidgetType::Label(w)  => w.update(ctx, old_data, data, env),
            WidgetType::None => {}
        };
    }

    fn to_type(&mut self) -> WidgetType<D> {
        WidgetType::None
    }
}