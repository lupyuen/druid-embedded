//! WidgetBox contains a Widget. Allows for dynamic dispatch with static Widgets.

use core::any::Any;
use crate::kurbo::{Point, Rect, Size};
use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
    WidgetPod,
    widget::{Button, Flex, Label},
};

/// Boxed version of a Widget
pub struct WidgetBox<DataType: Data + 'static> {
    pub widget: WidgetType<DataType>,
}

/// Enum of all possible Widget types
pub enum WidgetType<DataType: Data + 'static> {
    Button(Button<DataType>),
    Flex(Flex<DataType>),
    Label(Label<DataType>),
}

impl<DataType: Data + 'static> WidgetBox<DataType> {
    /// Create a new box for the `Widget`
    pub fn new<T: Any>(widget: &T) -> Self {
        let widget_any = widget as &dyn Any;
        WidgetBox {
            widget: {
                //  Try to convert our widget into a concrete `Widget`
                if let Some(w) = widget_any.downcast_ref::<Button::<DataType>>() {
                    WidgetType::Button(*w)        
                }
                else if let Some(w) = widget_any.downcast_ref::<Flex::<DataType>>() {
                    WidgetType::Flex(*w)        
                }
                else if let Some(w) = widget_any.downcast_ref::<Label::<DataType>>() {
                    WidgetType::Label(*w)        
                }
                else { panic!("not widget") }
            }
        }
    }

    /// Return the `Widget` inside the box
    fn get_widget(&mut self) -> &mut dyn Widget<DataType> {
        match &mut self.widget {
            WidgetType::Button(w) => w as &mut dyn Widget<DataType>,
            WidgetType::Flex(w)   => w as &mut dyn Widget<DataType>,
            WidgetType::Label(w)  => w as &mut dyn Widget<DataType>,
        }
    }
}

impl<DataType: Data + 'static> Widget<DataType> for WidgetBox<DataType> {
    fn paint(
        &mut self, 
        paint_ctx: &mut PaintCtx, 
        base_state: &BaseState, 
        data: &DataType, 
        env: &Env
    ) {
        self.get_widget().paint(paint_ctx, base_state, data, env);
        /*
        match &mut self.widget {
            WidgetType::Button(w) => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Label(w)  => w.paint(paint_ctx, base_state, data, env),
        };
        */
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DataType,
        env: &Env,
    ) -> Size {
        self.get_widget().layout(layout_ctx, bc, data, env)
        /*
        match &mut self.widget {
            WidgetType::Button(w) => w.layout(layout_ctx, bc, data, env),
            WidgetType::Label(w)  => w.layout(layout_ctx, bc, data, env),
        }
        */
    }

    fn event(
        &mut self, 
        ctx: &mut EventCtx, 
        event: &Event, 
        data: &mut DataType, 
        env: &Env
    ) {
        self.get_widget().event(ctx, event, data, env);
        /*
        match &mut self.widget {
            WidgetType::Button(w) => w.event(ctx, event, data, env),
            WidgetType::Label(w)  => w.event(ctx, event, data, env),
        };
        */
    }

    fn update(
        &mut self, 
        ctx: &mut UpdateCtx, 
        old_data: Option<&DataType>, 
        data: &DataType, 
        env: &Env
    ) {
        self.get_widget().update(ctx, old_data, data, env);
        /*
        match &mut self.widget {
            WidgetType::Button(w) => w.update(ctx, old_data, data, env),
            WidgetType::Label(w)  => w.update(ctx, old_data, data, env),
        };
        */
    }
}