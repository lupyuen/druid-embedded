//! `WidgetBox` contains a `Widget`. Allows for dynamic dispatch with static `Widgets` in `[no_std]`.

use core::marker::PhantomData;
use crate::kurbo::{Rect, Size};
use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget, WindowBox,
    widget::{Button, Flex, Label},
};

type MAX_WIDGETS = heapless::consts::U5;  //  Max number of `Widgets`
static mut ALL_WIDGETS: Option<heapless::Vec::<WidgetType<u32>, MAX_WIDGETS>> = None;

/// Boxed version of a `Widget`
#[derive(Clone, Default)]
pub struct WidgetBox<D: Data + 'static + Default>(
    WidgetType<D>,
    PhantomData<D>,  //  Needed to do compile-time checking for `Data`
);

/// Enum to store each `Widget`
#[derive(Clone)]
pub enum WidgetType<D: Data + 'static + Default> {
    None,
    Button(Button<D>),
    ////Flex(Flex<D>),
    Label(Label<D>),
}

impl<D: Data + 'static + Default> Default for WidgetType<D> {
    fn default() -> Self { WidgetType::None }
}

/// Generic implementation of `WidgetBox`
impl<D: Data + 'static + Default> WidgetBox<D> {
    /// Create a new box for the `Widget`
    pub fn new<W: Widget<D>>(widget: &mut W) -> Self {
        unsafe { ALL_WIDGETS = Some(heapless::Vec::new()); } ////
        WidgetBox(
            widget.to_type(),
            PhantomData,
        )
    }
}

/// Implementation of `Widget` trait for `WidgetBox`. We just forward to the inner `Widget`.
impl<D: Data + 'static + Default> Widget<D> for WidgetBox<D> {
    fn paint(
        &mut self, 
        paint_ctx: &mut PaintCtx, 
        base_state: &BaseState, 
        data: &D, 
        env: &Env
    ) {
        match &mut self.0 {
            WidgetType::Button(w) => w.paint(paint_ctx, base_state, data, env),
            ////WidgetType::Flex(w)   => w.paint(paint_ctx, base_state, data, env),
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
            ////WidgetType::Flex(w)   => w.layout(layout_ctx, bc, data, env),
            WidgetType::Label(w)  => w.layout(layout_ctx, bc, data, env),
            WidgetType::None => Size::ZERO,
        }
    }

    fn event(
        &mut self, 
        ctx: &mut EventCtx<D>, 
        event: &Event, 
        data: &mut D, 
        env: &Env
    ) {
        match &mut self.0 {
            WidgetType::Button(w) => w.event(ctx, event, data, env),
            ////WidgetType::Flex(w)   => w.event(ctx, event, data, env),
            WidgetType::Label(w)  => w.event(ctx, event, data, env),
            WidgetType::None => {}
        };
    }

    fn update(
        &mut self, 
        ctx: &mut UpdateCtx<D>, 
        old_data: Option<&D>, 
        data: &D, 
        env: &Env
    ) {
        match &mut self.0 {
            WidgetType::Button(w) => w.update(ctx, old_data, data, env),
            ////WidgetType::Flex(w)   => w.update(ctx, old_data, data, env),
            WidgetType::Label(w)  => w.update(ctx, old_data, data, env),
            WidgetType::None => {}
        };
    }

    fn to_type(&mut self) -> WidgetType<D> {
        WidgetType::None
    }

    fn new_window(self) -> WindowBox<D> {
        WindowBox::new()
    }

    fn get_id(self) -> u32 { ////
        match self.0 {
            WidgetType::Button(w) => w.get_id(),
            ////WidgetType::Flex(w)   => w.get_id(),
            WidgetType::Label(w)  => w.get_id(),
            WidgetType::None => panic!("no id")
        }
    }
}