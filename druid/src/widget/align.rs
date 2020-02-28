// Copyright 2018 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A widget that aligns its child (for example, centering it).

use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, Rect, Size,
    UpdateCtx, Widget, WidgetPod,
    WidgetId, WidgetType, WidgetBox, Window, WindowType, WindowBox, ////
};

use crate::piet::UnitPoint;

/// A widget that aligns its child.
#[derive(Clone)] ////
pub struct Align<T: Data + 'static + Default> { ////
////pub struct Align<T: Data> {
    id: WidgetId, //// Unique Widget ID
    align: UnitPoint,
    child: WidgetPod<T, WidgetBox<T>>, ////
    ////child: WidgetPod<T, Box<dyn Widget<T>>>,
    width_factor: Option<f64>,
    height_factor: Option<f64>,
}

impl<T: Data + 'static + Default> Align<T> { ////
////impl<T: Data> Align<T> {
    /// Create widget with alignment.
    ///
    /// Note that the `align` parameter is specified as a `UnitPoint` in
    /// terms of left and right. This is inadequate for bidi-aware layout
    /// and thus the API will change when druid gains bidi capability.
    pub fn new<W: Widget<T> + Clone>(align: UnitPoint, child: W) -> Align<T> { ////
    ////pub fn new(align: UnitPoint, child: impl Widget<T> + 'static) -> Align<T> {
        Align {
            id: super::get_widget_id(), ////
            align,
            child: WidgetPod::new( ////
                WidgetBox::<T>::new(child)
            ),
            ////child: WidgetPod::new(child).boxed(),
            width_factor: None,
            height_factor: None,
        }
    }

    /// Create centered widget.
    pub fn centered<W: Widget<T> + Clone>(child: W) -> Align<T> { ////
    ////pub fn centered(child: impl Widget<T> + 'static) -> Align<T> {
        Align::new(UnitPoint::CENTER, child)
    }

    /// Create right-aligned widget.
    pub fn right<W: Widget<T> + Clone>(child: W) -> Align<T> { ////
    ////pub fn right(child: impl Widget<T> + 'static) -> Align<T> {
        Align::new(UnitPoint::RIGHT, child)
    }

    /// Create left-aligned widget.
    pub fn left<W: Widget<T> + Clone>(child: W) -> Align<T> { ////
    ////pub fn left(child: impl Widget<T> + 'static) -> Align<T> {
        Align::new(UnitPoint::LEFT, child)
    }

    /// Align only in the horizontal axis, keeping the child's size in the vertical.
    pub fn horizontal<W: Widget<T> + Clone>(align: UnitPoint, child: W) -> Align<T> { ////
    ////pub fn horizontal(align: UnitPoint, child: impl Widget<T> + 'static) -> Align<T> {
        Align {
            id: super::get_widget_id(), ////
            align,
            child: WidgetPod::new( ////
                WidgetBox::<T>::new(child)
            ),
            ////child: WidgetPod::new(child).boxed(),
            width_factor: None,
            height_factor: Some(1.0),
        }
    }

    /// Align only in the vertical axis, keeping the child's size in the horizontal.
    pub fn vertical<W: Widget<T> + Clone>(align: UnitPoint, child: W) -> Align<T> { ////
    ////pub fn vertical(align: UnitPoint, child: impl Widget<T> + 'static) -> Align<T> {
        Align {
            id: super::get_widget_id(), ////
            align,
            child: WidgetPod::new( ////
                WidgetBox::<T>::new(child)
            ),
            ////child: WidgetPod::new(child).boxed(),
            width_factor: Some(1.0),
            height_factor: None,
        }
    }
}

impl<T: Data + 'static + Default> Widget<T> for Align<T> {
    fn paint(&mut self, paint_ctx: &mut PaintCtx, _base_state: &BaseState, data: &T, env: &Env) {
        self.child.paint_with_offset(paint_ctx, data, env);
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        bc.debug_check("Align");

        let size = self.child.layout(layout_ctx, &bc.loosen(), data, env);
        let mut my_size = size;
        if bc.is_width_bounded() {
            my_size.width = bc.max().width;
        }
        if bc.is_height_bounded() {
            my_size.height = bc.max().height;
        }

        if let Some(width) = self.width_factor {
            my_size.width = size.width * width;
        }
        if let Some(height) = self.height_factor {
            my_size.height = size.height * height;
        }

        my_size = bc.constrain(my_size);
        let extra_width = (my_size.width - size.width).max(0.);
        let extra_height = (my_size.height - size.height).max(0.);
        let origin = self
            .align
            .resolve(Rect::new(0., 0., extra_width, extra_height));
        self.child
            .set_layout_rect(Rect::from_origin_size(origin, size));
        my_size
    }

    fn event(&mut self, ctx: &mut EventCtx<T>, event: &Event, data: &mut T, env: &Env) { ////
    ////fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.child.event(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx<T>, _old_data: Option<&T>, data: &T, env: &Env) { ////
    ////fn update(&mut self, ctx: &mut UpdateCtx, _old_data: Option<&T>, data: &T, env: &Env) {
        self.child.update(ctx, data, env);
    }

    fn to_type(self) -> WidgetType<T> { ////
        WidgetType::Align(self)
    }

    fn new_window(self) -> WindowBox<T> { ////
        let window = Window::new(self);
        let window_box = WindowBox(
            WindowType::Align(window),
        );
        window_box
    }

    fn get_id(self) -> WidgetId { ////
        self.id
    }
}
