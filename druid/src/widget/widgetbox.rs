//! WidgetBox contains a Widget. Allows for dynamic dispatch with static Widgets.

use crate::kurbo::{Point, Rect, Size}; ////

use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
    WidgetPod,
    widget::Label,
};

pub struct WidgetBox<DataType: Data> {
    pub widget: WidgetType<DataType>,
}

pub enum WidgetType<DataType: Data> {
    Label(Label<DataType>),
}

impl<DataType: Data> Widget<DataType> for WidgetBox<DataType> {
    fn paint(
        &mut self, 
        paint_ctx: &mut PaintCtx, 
        _base_state: &BaseState, 
        data: &DataType, 
        env: &Env
    ) {
        assert!(false, "no paint");
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DataType,
        env: &Env,
    ) -> Size {
        assert!(false, "no layout");
        Size::new(0., 0.)
    }

    fn event(
        &mut self, 
        ctx: &mut EventCtx, 
        event: &Event, 
        data: &mut DataType, 
        env: &Env
    ) {
        assert!(false, "no event");
    }

    fn update(
        &mut self, 
        ctx: &mut UpdateCtx, 
        _old_data: Option<&DataType>, 
        data: &DataType, 
        env: &Env
    ) {
        assert!(false, "no update");
    }
}