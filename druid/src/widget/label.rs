// Copyright 2019 The xi-editor Authors.
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

//! A label widget.

////use std::marker::PhantomData;

use core::str::FromStr; ////
use crate::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, Size, UpdateCtx,
    Widget, 
    WidgetId, WidgetType, WindowBox, ////
};

use crate::kurbo::Rect;
use crate::piet::{
    FontBuilder, PietText, PietTextLayout, Text, TextLayout, TextLayoutBuilder, UnitPoint,
};

use crate::localization::LocalizedString;
////use crate::theme;
use crate::{Point, RenderContext};

type MaxLabel = heapless::consts::U20; //// Max length of label strings
type String = heapless::String::<MaxLabel>; ////

/// The text for the label; either a localized or a specific string.
#[derive(Clone)] ////
pub enum LabelText<T> {
    Localized(LocalizedString<T>),
    Specific(String),
}

/// A label that displays some text.
#[derive(Clone)] ////
pub struct Label<T> {
    id: WidgetId, //// Unique Widget ID
    text: LabelText<T>,
    align: UnitPoint,
}

/* ////
/// A label with dynamic text.
///
/// The provided closure is called on update, and its return
/// value is used as the text for the label.
pub struct DynLabel<T: Data> {
    label_closure: Box<dyn FnMut(&T, &Env) -> String>,
    phantom: PhantomData<T>,
}
*/ ////

impl<T: Data + 'static + Default> Label<T> { ////
////impl<T: Data> Label<T> {
    /// Discussion question: should this return Label or a wrapped
    /// widget (with WidgetPod)?
    pub fn new(text: impl Into<LabelText<T>>) -> Self {
        Label {
            id: super::get_widget_id(), ////
            text: text.into(),
            align: UnitPoint::LEFT,
        }
    }

    pub fn aligned(text: impl Into<LabelText<T>>, align: UnitPoint) -> Self {
        Label {
            id: super::get_widget_id(), ////
            text: text.into(),
            align,
        }
    }

    fn get_layout(&self, t: &mut PietText, _env: &Env) -> PietTextLayout {
        let font_name = crate::env::FONT_NAME; ////env.get(theme::FONT_NAME);
        let font_size = crate::env::TEXT_SIZE_NORMAL; ////env.get(theme::TEXT_SIZE_NORMAL);
        let text = self.text.display_text();
        // TODO: caching of both the format and the layout
        let font = t.new_font_by_name(font_name, font_size).build().expect("get layout fail"); //// .unwrap();
        t.new_text_layout(&font, text).build().expect("get layout fail") ///// .unwrap()
    }
}

impl<T: Data + 'static + Default> Widget<T> for Label<T> { ////
////impl<T: Data> Widget<T> for Label<T> {
    fn paint(&mut self, paint_ctx: &mut PaintCtx, base_state: &BaseState, _data: &T, env: &Env) {
        let font_size = crate::env::TEXT_SIZE_NORMAL; ////env.get(theme::TEXT_SIZE_NORMAL);

        let text_layout = self.get_layout(paint_ctx.render_ctx.text(), env); ////
        ////let text_layout = self.get_layout(paint_ctx.text(), env);

        // Find the origin for the text
        let mut origin = self.align.resolve(Rect::from_origin_size(
            Point::ORIGIN,
            Size::new(
                (base_state.size().width - text_layout.width()).max(0.0),
                base_state.size().height + (font_size * 1.2) / 2.,
            ),
        ));

        //Make sure we don't draw the text too low
        origin.y = origin.y.min(base_state.size().height);

        paint_ctx.render_ctx.draw_text(&text_layout, origin, &crate::env::LABEL_COLOR); ////
        ////paint_ctx.draw_text(&text_layout, origin, &env.get(theme::LABEL_COLOR));
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        bc.debug_check("Label");
        self.text.resolve(data, env);  ////  TODO: Should auto-resolve

        let font_size = crate::env::TEXT_SIZE_NORMAL; ////
        ////let font_size = env.get(theme::TEXT_SIZE_NORMAL);
        let text_layout = self.get_layout(layout_ctx.text(), env);
        // This magical 1.2 constant helps center the text vertically in the rect it's given
        bc.constrain((text_layout.width(), font_size * 1.2))
    }

    fn event(&mut self, _ctx: &mut EventCtx<T>, _event: &Event, _data: &mut T, _env: &Env) {} ////
    ////fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx<T>, _old_data: Option<&T>, data: &T, env: &Env) { ////
    ////fn update(&mut self, ctx: &mut UpdateCtx, _old_data: Option<&T>, data: &T, env: &Env) {
        //cortex_m::asm::bkpt(); ////
        if self.text.resolve(data, env) {
            ctx.invalidate();
        }
    }

    fn to_type(self) -> WidgetType<T> { ////
        WidgetType::Label(self)
    }

    fn new_window(self) -> WindowBox<T> { ////
        WindowBox::new()
    }

    fn get_id(self) -> WidgetId { ////
        self.id
    }
}

impl<T: Data + 'static + Default> LabelText<T> { ////
////impl<T: Data> LabelText<T> {
    /// The text that should be displayed. This ensures that localized
    /// strings are up to date.
    pub fn display_text(&self) -> &str {
        match self {
            LabelText::Specific(s) => s.as_str(),
            LabelText::Localized(s) => s.localized_str(),
        }
    }

    /// Update the localization, if necessary.
    ///
    /// Returns `true` if the string has changed.
    pub fn resolve(&mut self, data: &T, env: &Env) -> bool {
        match self {
            LabelText::Specific(_) => false,
            LabelText::Localized(s) => s.resolve(data, env),
        }
    }
}

/* ////
    impl<T: Data> DynLabel<T> {
        pub fn new(label_closure: impl FnMut(&T, &Env) -> String + 'static) -> DynLabel<T> {
            DynLabel {
                label_closure: Box::new(label_closure),
                phantom: Default::default(),
            }
        }

        fn get_layout(&mut self, t: &mut PietText, env: &Env, data: &T) -> PietTextLayout {
            let text = (self.label_closure)(data, env);

            let font_name = env.get(theme::FONT_NAME);
            let font_size = env.get(theme::TEXT_SIZE_NORMAL);

            // TODO: caching of both the format and the layout
            let font = t.new_font_by_name(font_name, font_size).build().unwrap();
            t.new_text_layout(&font, &text).build().unwrap()
        }
    }

    impl<T: Data> Widget<T> for DynLabel<T> {
        fn paint(&mut self, paint_ctx: &mut PaintCtx, base_state: &BaseState, data: &T, env: &Env) {
            let font_size = env.get(theme::TEXT_SIZE_NORMAL);

            let align = UnitPoint::LEFT;
            let origin = align.resolve(Rect::from_origin_size(
                Point::ORIGIN,
                Size::new(
                    base_state.size().width,
                    base_state.size().height + (font_size * 1.2) / 2.,
                ),
            ));

            let text_layout = self.get_layout(paint_ctx.text(), env, data);
            paint_ctx.draw_text(&text_layout, origin, &env.get(theme::LABEL_COLOR));
        }

        fn layout(
            &mut self,
            layout_ctx: &mut LayoutCtx,
            bc: &BoxConstraints,
            data: &T,
            env: &Env,
        ) -> Size {
            bc.debug_check("DynLabel");

            let font_size = env.get(theme::TEXT_SIZE_NORMAL);
            let text_layout = self.get_layout(layout_ctx.text(), env, data);
            // This magical 1.2 constant helps center the text vertically in the rect it's given
            bc.constrain(Size::new(text_layout.width(), font_size * 1.2))
        }

        fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

        fn update(&mut self, ctx: &mut UpdateCtx, _old_data: Option<&T>, _data: &T, _env: &Env) {
            ctx.invalidate();
        }
    }
*/ ////

impl<T> From<String> for LabelText<T> {
    fn from(src: String) -> LabelText<T> {
        LabelText::Specific(src)
    }
}

impl<T> From<&str> for LabelText<T> {
    fn from(src: &str) -> LabelText<T> {
        LabelText::Specific(String::from_str(src).expect("label text fail")) ////
        ////LabelText::Specific(src.to_string())
    }
}

impl<T> From<LocalizedString<T>> for LabelText<T> {
    fn from(src: LocalizedString<T>) -> LabelText<T> {
        LabelText::Localized(src)
    }
}
