//! `WidgetBox` contains a `Widget`. Allows for dynamic dispatch with static `Widgets` in `[no_std]`.

use core::marker::PhantomData;
use crate::kurbo::{Rect, Size};
use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget, WindowBox,
    widget::{Button, Flex, Label},
};

/*
type MAX_WIDGETS = heapless::consts::U5;  //  Max number of `Widgets`
static mut ALL_WIDGETS: Option<WidgetVec<u32>> = None;

struct WidgetVec<D: Data + 'static + Default> {
    /// widgets[i] contains the Widget with ID i
    widgets: heapless::Vec::<WidgetType<D>, MAX_WIDGETS>,
}

impl<D: Data + 'static + Default> WidgetVec<D> {
    fn new() -> Self {
        //  Fill the Widget list with None.
        let widgets = heapless::Vec::new();
        loop {
            if let Err(_) = widgets.push(WidgetType::None) {
                break;
            }
        }
        WidgetVec{
            widgets,
        }
    }

    fn set(&mut self, id: u32, widget: WidgetType<D>) {
        self.widgets[id as usize] = widget;
    }

    fn get(&mut self, id: u32) -> WidgetType<D> {
        self.widgets[id as usize]
    }
}
*/

/// Boxed version of a `Widget`
#[derive(Clone, Default)]
pub struct WidgetBox<D: Data + 'static + Default>(
    u32,  //  Widget ID
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
        /*
        if let None = ALL_WIDGETS {
            ALL_WIDGETS = Some(WidgetVec::new());
        }
        let mut widgets = ALL_WIDGETS.unwrap();
        */
        let id = widget.get_id();
        crate::AppState::<D>::add_widget(id, widget.to_type());
        //widgets.set(id, widget.to_type());
        WidgetBox(
            id,
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
        match &mut crate::APP_STATE.get_widget(self.0) {
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
        match &mut crate::APP_STATE.get_widget(self.0) {
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
        match &mut crate::APP_STATE.get_widget(self.0) {
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
        match &mut crate::APP_STATE.get_widget(self.0) {
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

    fn get_id(self) -> u32 {
        match crate::APP_STATE.get_widget(self.0) {
            WidgetType::Button(w) => w.get_id(),
            ////WidgetType::Flex(w)   => w.get_id(),
            WidgetType::Label(w)  => w.get_id(),
            WidgetType::None => panic!("no id")
        }
    }
}