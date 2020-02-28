//! `WidgetBox` contains a `Widget`. Allows for dynamic dispatch with static `Widgets` in `[no_std]`.
use core::marker::PhantomData;
use crate::kurbo::{Size};
use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget, WindowBox,
    widget::{Align, Button, Flex, Label, Padding},
};

/// Widgets are identified by an 8-bit ID
pub type WidgetId = u8;

/// Max number of `Widgets` on embedded platforms
pub const MAX_WIDGETS: usize = 10;

/// Unique Widget ID
static mut WIDGET_ID: WidgetId = 0;

/// Assign a unique Widget ID
pub fn get_widget_id() -> WidgetId {
    let id = unsafe { WIDGET_ID };
    assert!((id as usize) < MAX_WIDGETS, "too many widgets");
    unsafe { WIDGET_ID += 1; }
    id
}

/// Specialised Trait for handling static `Widgets` on embedded platforms
pub trait GlobalWidgets<D: Data + 'static + Default> {
    /// Fetch the static `Widgets` for the Data type
    fn get_widgets(&self) -> &'static mut [ WidgetType<D> ];
    /// Add a `Widget` for the Data type
    fn add_widget(&self, widget: WidgetType<D>);
}

/// Default Trait will not have static `Widgets`
impl<D: Data + 'static + Default> GlobalWidgets<D> for WidgetBox<D> {
    default fn get_widgets(&self) -> &'static mut [ WidgetType<D> ] { panic!("no global widgets") }
    default fn add_widget(&self, _widget: WidgetType<D>) { panic!("no global widgets") }
}

/// Boxed version of a `Widget`
#[derive(Clone, Default)]
pub struct WidgetBox<D: Data + 'static>(
    pub WidgetId,    //  Widget ID
    PhantomData<D>,  //  Needed to do compile-time checking for `Data`
);

/// Enum to store each `Widget`
#[derive(Clone)]
pub enum WidgetType<D: Data + 'static + Default> {
    None,
    Align(Align<D>),
    Button(Button<D>),
    Flex(Flex<D>),
    Label(Label<D>),
    Padding(Padding<D>),
}

impl<D: Data + 'static + Default> Default for WidgetType<D> {
    fn default() -> Self { WidgetType::None }
}

/// Generic implementation of `WidgetBox`
impl<D: Data + 'static + Default> WidgetBox<D> {
    /// Create a new box for the `Widget`
    pub fn new<W: Widget<D> + Clone>(widget: W) -> Self {
        let id = widget.clone().get_id();
        let widget_type: WidgetType<D> = widget.to_type();
        let widget_box: WidgetBox<D> = WidgetBox(
            id,
            PhantomData,
        );
        widget_box.clone().add_widget(widget_type);
        widget_box
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
        match &mut self.get_widgets()[self.0 as usize] {
            WidgetType::Align(w)   => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Button(w)  => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Flex(w)    => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Label(w)   => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Padding(w) => w.paint(paint_ctx, base_state, data, env),
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
        match &mut self.get_widgets()[self.0 as usize] {
            WidgetType::Align(w)   => w.layout(layout_ctx, bc, data, env),
            WidgetType::Button(w)  => w.layout(layout_ctx, bc, data, env),
            WidgetType::Flex(w)    => w.layout(layout_ctx, bc, data, env),
            WidgetType::Label(w)   => w.layout(layout_ctx, bc, data, env),
            WidgetType::Padding(w) => w.layout(layout_ctx, bc, data, env),
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
        match &mut self.get_widgets()[self.0 as usize] {
            WidgetType::Align(w)   => w.event(ctx, event, data, env),
            WidgetType::Button(w)  => w.event(ctx, event, data, env),
            WidgetType::Flex(w)    => w.event(ctx, event, data, env),
            WidgetType::Label(w)   => w.event(ctx, event, data, env),
            WidgetType::Padding(w) => w.event(ctx, event, data, env),
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
        match &mut self.get_widgets()[self.0 as usize] {
            WidgetType::Align(w)   => w.update(ctx, old_data, data, env),
            WidgetType::Button(w)  => w.update(ctx, old_data, data, env),
            WidgetType::Flex(w)    => w.update(ctx, old_data, data, env),
            WidgetType::Label(w)   => w.update(ctx, old_data, data, env),
            WidgetType::Padding(w) => w.update(ctx, old_data, data, env),
            WidgetType::None => {}
        };
    }

    fn to_type(self) -> WidgetType<D> {
        WidgetType::None
    }

    fn new_window(self) -> WindowBox<D> {
        WindowBox::new()
    }

    fn get_id(self) -> WidgetId {
        match &mut self.get_widgets()[self.0 as usize] {
            WidgetType::Align(w)   => w.clone().get_id(),
            WidgetType::Button(w)  => w.clone().get_id(),
            WidgetType::Flex(w)    => w.clone().get_id(),
            WidgetType::Label(w)   => w.clone().get_id(),
            WidgetType::Padding(w) => w.clone().get_id(),
            WidgetType::None => panic!("no id")
        }
    }
}