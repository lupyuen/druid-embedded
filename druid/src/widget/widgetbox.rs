//! WidgetBox contains a Widget. Allows for dynamic dispatch with static Widgets.

use crate::kurbo::{Point, Rect, Size}; ////

use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
    WidgetPod,
    widget::{Button, Label},
};

pub struct WidgetBox<DataType: Data> {
    pub widget: WidgetType<DataType>,
}

pub enum WidgetType<DataType: Data> {
    Button(Button<DataType>),
    Label(Label<DataType>),
}

/*
impl<DataType: Data> WidgetBox<DataType> {
    fn get_widget(&mut self) -> &mut impl Widget<DataType> {
        match &mut self.widget {
            WidgetType::Button(b) => b,
            WidgetType::Label(l)  => l,
        }
    }
}
*/

impl<DataType: Data> Widget<DataType> for WidgetBox<DataType> {
    fn paint(
        &mut self, 
        paint_ctx: &mut PaintCtx, 
        base_state: &BaseState, 
        data: &DataType, 
        env: &Env
    ) {
        match &mut self.widget {
            WidgetType::Button(w) => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Label(w)  => w.paint(paint_ctx, base_state, data, env),
        };
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DataType,
        env: &Env,
    ) -> Size {
        match &mut self.widget {
            WidgetType::Button(w) => w.layout(layout_ctx, bc, data, env),
            WidgetType::Label(w)  => w.layout(layout_ctx, bc, data, env),
        }
    }

    fn event(
        &mut self, 
        ctx: &mut EventCtx, 
        event: &Event, 
        data: &mut DataType, 
        env: &Env
    ) {
        match &mut self.widget {
            WidgetType::Button(w) => w.event(ctx, event, data, env),
            WidgetType::Label(w)  => w.event(ctx, event, data, env),
        };
    }

    fn update(
        &mut self, 
        ctx: &mut UpdateCtx, 
        old_data: Option<&DataType>, 
        data: &DataType, 
        env: &Env
    ) {
        match &mut self.widget {
            WidgetType::Button(w) => w.update(ctx, old_data, data, env),
            WidgetType::Label(w)  => w.update(ctx, old_data, data, env),
        };
    }
}