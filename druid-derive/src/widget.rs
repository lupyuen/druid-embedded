//! Extended implementation for #[derive(Data)] to support static Widgets and Windows on embedded platforms

use quote::quote;

/// Given an Application State type name (e.g. `State`) and struct, derive the static Widgets and Windows
pub fn derive_widget(state_type: syn::Ident, state_struct: syn::DataStruct) -> Result<proc_macro2::TokenStream, syn::Error> {
    println!("state_type: {:#?}", state_type); ////
    println!("state_struct: {:#?}", state_struct); ////
    let res = quote! {
        use druid_shell::WinHandler;

        /// Handle a touch event at the (x,y) coordinates
        pub fn handle_touch(x: u16, y: u16) {
            let mut ctx = druid::DruidContext::new();
            let handler = unsafe { &mut ALL_HANDLERS_STATE[1] };  //  Assume first window has ID 1
            handler.mouse_down(
                &druid_shell::MouseEvent {
                    pos: druid::Point::new(x as f64, y as f64),
                    count: 1,
                    button: druid_shell::MouseButton::Left,
                },
                &mut ctx,
            );
            handler.mouse_up(
                &druid_shell::MouseEvent {
                    pos: druid::Point::new(x as f64, y as f64),
                    count: 0,
                    button: druid_shell::MouseButton::Left,
                },
                &mut ctx,
            );
        }
        
        /// DATA is the Application Data
        static mut DATA_STATE: State = State { count: 0 };  //  Generated based on `State`
        
        /// Static list of Widgets for embedded platforms
        static mut ALL_WIDGETS_STATE: [ druid::WidgetType<State>; druid::MAX_WIDGETS ] = [ 
            druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None,
            druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None,
        ];
        
        /// ALL_WINDOWS[i] is the WindowBox for the Window with window ID i. i=0 is not used.
        static mut ALL_WINDOWS_STATE: [ druid::WindowBox<State>; druid::MAX_WINDOWS ] = [
            druid::WindowBox::<State>( druid::WindowType::None ), 
            druid::WindowBox::<State>( druid::WindowType::None ), 
            druid::WindowBox::<State>( druid::WindowType::None ), 
        ];
        
        /// ALL_HANDLERS[i] is the Window Handler for the Window with window ID i. i=0 is not used.
        static mut ALL_HANDLERS_STATE: [ druid::DruidHandler<State>; druid::MAX_WINDOWS ] = [
            druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
            druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
            druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
        ];
        
        /// Specialised Trait to reference Widgets statically on embedded platforms
        impl druid::GlobalWidgets<State> for druid::WidgetBox<State> {
            /// Fetch the static Widgets for the Data type
            fn get_widgets(&self) -> &'static mut [ druid::WidgetType<State> ] {
                unsafe { &mut ALL_WIDGETS_STATE }
            }
            /// Add a Widget for the Data type
            fn add_widget(&self, widget: druid::WidgetType<State>) {
                assert!(self.0 < druid::MAX_WIDGETS as u32, "too many widgets");
                unsafe { ALL_WIDGETS_STATE[self.0 as usize] = widget; }        
            }    
        }
        
        /// Specialised Trait to reference Windows and Window Handlers statically on embedded platforms
        impl druid::GlobalWindows<State> for druid::AppState<State> {
            fn add_window(&self, window_id: druid::WindowId, window: druid::WindowBox<State>) {
                unsafe { ALL_WINDOWS_STATE[window_id.0 as usize] = window; }
            }
            fn add_handler(&self, window_id: druid::WindowId, handler: druid::DruidHandler<State>) {
                unsafe { ALL_HANDLERS_STATE[window_id.0 as usize] = handler; }
            }
            fn get_handle(&self, window_id: druid::WindowId) -> druid::WindowHandle<druid::DruidHandler<State>> {
                let handler = unsafe { ALL_HANDLERS_STATE[window_id.0 as usize].clone() };
                druid::WindowHandle(
                    druid::PlatformWindowHandle {
                        window_id: window_id.0,
                        state: druid::PlatformWindowState {
                            window_id: window_id.0,
                            handler,                
                        }            
                    }
                )
            }
            fn set_data(&self, data: State) {
                unsafe { DATA_STATE = data; }
            }
            fn window_event(
                &mut self, 
                window_id: druid::WindowId,
                ctx: &mut druid::EventCtx<State>, 
                event: &druid::Event, 
            ) {
                unsafe { 
                    ALL_WINDOWS_STATE[window_id.0 as usize].event(
                        ctx, 
                        event, 
                        &mut DATA_STATE,  //  Data
                        &Env {}           //  Env
                    );
                }
            }
            fn window_update(
                &mut self, 
                window_id: druid::WindowId,
                ctx: &mut druid::UpdateCtx<State>, 
            ) {
                unsafe { 
                    ALL_WINDOWS_STATE[window_id.0 as usize].update(
                        ctx,
                        &mut DATA_STATE,  //  Data
                        &Env {}           //  Env
                    ); 
                }
            }
            fn window_layout(
                &mut self,
                window_id: druid::WindowId,
                layout_ctx: &mut druid::LayoutCtx,
            ) {
                unsafe { 
                    ALL_WINDOWS_STATE[window_id.0 as usize].layout(
                        layout_ctx, 
                        &mut DATA_STATE,  //  Data
                        &Env {}           //  Env
                    ); 
                }
            }
            fn window_paint(
                &mut self, 
                window_id: druid::WindowId,
                paint_ctx: &mut druid::PaintCtx, 
            ) {
                unsafe { 
                    ALL_WINDOWS_STATE[window_id.0 as usize].paint(
                        paint_ctx, 
                        &mut DATA_STATE,  //  Data
                        &Env {}           //  Env
                    ); 
                }
            }
            fn window_has_active(
                &mut self,
                window_id: druid::WindowId,
            ) -> bool {
                unsafe { 
                    ALL_WINDOWS_STATE[window_id.0 as usize].has_active() 
                }
            }
        }
    };
    Ok(res)
}

/*
////////////////////////////// TODO: Generate via Data trait

use druid_shell::WinHandler;

/// Handle a touch event at the (x,y) coordinates
pub fn handle_touch(x: u16, y: u16) {
    let mut ctx = druid::DruidContext::new();
    let handler = unsafe { &mut ALL_HANDLERS_STATE[1] };  //  Assume first window has ID 1
    handler.mouse_down(
        &druid_shell::MouseEvent {
            pos: druid::Point::new(x as f64, y as f64),
            count: 1,
            button: druid_shell::MouseButton::Left,
        },
        &mut ctx,
    );
    handler.mouse_up(
        &druid_shell::MouseEvent {
            pos: druid::Point::new(x as f64, y as f64),
            count: 0,
            button: druid_shell::MouseButton::Left,
        },
        &mut ctx,
    );
}

/// DATA is the Application Data
static mut DATA_STATE: State = State { count: 0 };  //  Generated based on `State`

/// Static list of Widgets for embedded platforms
static mut ALL_WIDGETS_STATE: [ druid::WidgetType<State>; druid::MAX_WIDGETS ] = [ 
    druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None,
    druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None,
];

/// ALL_WINDOWS[i] is the WindowBox for the Window with window ID i. i=0 is not used.
static mut ALL_WINDOWS_STATE: [ druid::WindowBox<State>; druid::MAX_WINDOWS ] = [
    druid::WindowBox::<State>( druid::WindowType::None ), 
    druid::WindowBox::<State>( druid::WindowType::None ), 
    druid::WindowBox::<State>( druid::WindowType::None ), 
];

/// ALL_HANDLERS[i] is the Window Handler for the Window with window ID i. i=0 is not used.
static mut ALL_HANDLERS_STATE: [ druid::DruidHandler<State>; druid::MAX_WINDOWS ] = [
    druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
    druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
    druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
];

/// Specialised Trait to reference Widgets statically on embedded platforms
impl druid::GlobalWidgets<State> for druid::WidgetBox<State> {
    /// Fetch the static Widgets for the Data type
    fn get_widgets(&self) -> &'static mut [ druid::WidgetType<State> ] {
        unsafe { &mut ALL_WIDGETS_STATE }
    }
    /// Add a Widget for the Data type
    fn add_widget(&self, widget: druid::WidgetType<State>) {
        assert!(self.0 < druid::MAX_WIDGETS as u32, "too many widgets");
        unsafe { ALL_WIDGETS_STATE[self.0 as usize] = widget; }        
    }    
}

/// Specialised Trait to reference Windows and Window Handlers statically on embedded platforms
impl druid::GlobalWindows<State> for druid::AppState<State> {
    fn add_window(&self, window_id: druid::WindowId, window: druid::WindowBox<State>) {
        unsafe { ALL_WINDOWS_STATE[window_id.0 as usize] = window; }
    }
    fn add_handler(&self, window_id: druid::WindowId, handler: druid::DruidHandler<State>) {
        unsafe { ALL_HANDLERS_STATE[window_id.0 as usize] = handler; }
    }
    fn get_handle(&self, window_id: druid::WindowId) -> druid::WindowHandle<druid::DruidHandler<State>> {
        let handler = unsafe { ALL_HANDLERS_STATE[window_id.0 as usize].clone() };
        druid::WindowHandle(
            druid::PlatformWindowHandle {
                window_id: window_id.0,
                state: druid::PlatformWindowState {
                    window_id: window_id.0,
                    handler,                
                }            
            }
        )
    }
    fn set_data(&self, data: State) {
        unsafe { DATA_STATE = data; }
    }
    fn window_event(
        &mut self, 
        window_id: druid::WindowId,
        ctx: &mut druid::EventCtx<State>, 
        event: &druid::Event, 
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].event(
                ctx, 
                event, 
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            );
        }
    }
    fn window_update(
        &mut self, 
        window_id: druid::WindowId,
        ctx: &mut druid::UpdateCtx<State>, 
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].update(
                ctx,
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            ); 
        }
    }
    fn window_layout(
        &mut self,
        window_id: druid::WindowId,
        layout_ctx: &mut druid::LayoutCtx,
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].layout(
                layout_ctx, 
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            ); 
        }
    }
    fn window_paint(
        &mut self, 
        window_id: druid::WindowId,
        paint_ctx: &mut druid::PaintCtx, 
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].paint(
                paint_ctx, 
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            ); 
        }
    }
    fn window_has_active(
        &mut self,
        window_id: druid::WindowId,
    ) -> bool {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].has_active() 
        }
    }
}
*/