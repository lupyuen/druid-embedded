//! Extended implementation for #[derive(Data)] to support static Widgets and Windows on embedded platforms

use quote::{quote, format_ident};

/// Given an Application State type name (e.g. `State`) and struct, derive the static Widgets and Windows
pub fn derive_widget(state_type: syn::Ident, state_struct: syn::DataStruct) -> Result<proc_macro2::TokenStream, syn::Error> {
    //  println!("state_type: {:#?}", state_type); ////
    //  println!("state_struct: {:#?}", state_struct); ////
    //  Compose the identifiers.
    let state_type_upper = state_type.to_string().to_uppercase();
    let data_state = format_ident!("DATA_{}", state_type_upper);
    let all_widgets_state = format_ident!("ALL_WIDGETS_{}", state_type_upper);
    let all_windows_state = format_ident!("ALL_WINDOWS_{}", state_type_upper);
    let all_handlers_state = format_ident!("ALL_HANDLERS_{}", state_type_upper);

    //  Compose the init values for the state e.g. `count: 0 as i32, s: "".to_string()`
    let mut init = quote! {};
    if let syn::Fields::Named(fields) = state_struct.fields {
        for field in &fields.named {
            let field_name = &field.ident;
            let field_type = &field.ty;
            let field_type_str = quote! { #field_type }.to_string();
            //  println!("field_name: {:#?}", field_name);
            //  println!("field_type: {:#?}", field_type);
            let field_value = match field_type_str.as_str() {
                "i32" => quote! { 0 as i32 },
                "u32" => quote! { 0 as u32 },
                "f32" => quote! { 0.0 as f32 },
                "f64" => quote! { 0.0 as f64 },
                "String" => quote! { "".to_string() },
                _ => { assert!(false, "unknown field type"); quote! {} }
            };
            init.extend(
                quote! { #field_name: #field_value, }
            );  //  e.g. `count: 0 as i32,`
        }    
    }

    //  Compose the traits for static Widgets and Windows.
    let res = quote! {
        use druid_shell::WinHandler;

        /// Handle a touch event at the (x,y) coordinates
        pub fn handle_touch(x: u16, y: u16) {
            let mut ctx = druid::DruidContext::new();
            let handler = unsafe { &mut #all_handlers_state[1] };  //  Assume first window has ID 1
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
        static mut #data_state: #state_type = #state_type { #init };  //  TODO: Generated based on `#state_type`
        
        /// Static list of Widgets for embedded platforms
        static mut #all_widgets_state: [ druid::WidgetType<#state_type>; druid::MAX_WIDGETS ] = [ 
            druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None,
            druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None,
        ];
        
        /// ALL_WINDOWS[i] is the WindowBox for the Window with window ID i. i=0 is not used.
        static mut #all_windows_state: [ druid::WindowBox<#state_type>; druid::MAX_WINDOWS ] = [
            druid::WindowBox::<#state_type>( druid::WindowType::None ), 
            druid::WindowBox::<#state_type>( druid::WindowType::None ), 
            druid::WindowBox::<#state_type>( druid::WindowType::None ), 
        ];
        
        /// ALL_HANDLERS[i] is the Window Handler for the Window with window ID i. i=0 is not used.
        static mut #all_handlers_state: [ druid::DruidHandler<#state_type>; druid::MAX_WINDOWS ] = [
            druid::DruidHandler::<#state_type> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
            druid::DruidHandler::<#state_type> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
            druid::DruidHandler::<#state_type> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
        ];
        
        /// Specialised Trait to reference Widgets statically on embedded platforms
        impl druid::GlobalWidgets<#state_type> for druid::WidgetBox<#state_type> {
            /// Fetch the static Widgets for the Data type
            fn get_widgets(&self) -> &'static mut [ druid::WidgetType<#state_type> ] {
                unsafe { &mut #all_widgets_state }
            }
            /// Add a Widget for the Data type
            fn add_widget(&self, widget: druid::WidgetType<#state_type>) {
                assert!((self.0 as usize) < druid::MAX_WIDGETS, "too many widgets");
                unsafe { #all_widgets_state[self.0 as usize] = widget; }        
            }    
        }
        
        /// Specialised Trait to reference Windows and Window Handlers statically on embedded platforms
        impl druid::GlobalWindows<#state_type> for druid::AppState<#state_type> {
            fn add_window(&self, window_id: druid::WindowId, window: druid::WindowBox<#state_type>) {
                unsafe { #all_windows_state[window_id.0 as usize] = window; }
            }
            fn add_handler(&self, window_id: druid::WindowId, handler: druid::DruidHandler<#state_type>) {
                unsafe { #all_handlers_state[window_id.0 as usize] = handler; }
            }
            fn get_handle(&self, window_id: druid::WindowId) -> druid::WindowHandle<druid::DruidHandler<#state_type>> {
                let handler = unsafe { #all_handlers_state[window_id.0 as usize].clone() };
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
            fn set_data(&self, data: #state_type) {
                unsafe { #data_state = data; }
            }
            fn window_event(
                &mut self, 
                window_id: druid::WindowId,
                ctx: &mut druid::EventCtx<#state_type>, 
                event: &druid::Event, 
            ) {
                unsafe { 
                    #all_windows_state[window_id.0 as usize].event(
                        ctx, 
                        event, 
                        &mut #data_state,  //  Data
                        &Env {}           //  Env
                    );
                }
            }
            fn window_update(
                &mut self, 
                window_id: druid::WindowId,
                ctx: &mut druid::UpdateCtx<#state_type>, 
            ) {
                unsafe { 
                    #all_windows_state[window_id.0 as usize].update(
                        ctx,
                        &mut #data_state,  //  Data
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
                    #all_windows_state[window_id.0 as usize].layout(
                        layout_ctx, 
                        &mut #data_state,  //  Data
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
                    #all_windows_state[window_id.0 as usize].paint(
                        paint_ctx, 
                        &mut #data_state,  //  Data
                        &Env {}           //  Env
                    ); 
                }
            }
            fn window_has_active(
                &mut self,
                window_id: druid::WindowId,
            ) -> bool {
                unsafe { 
                    #all_windows_state[window_id.0 as usize].has_active() 
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
        assert!((self.0 as usize)< druid::MAX_WIDGETS, "too many widgets");
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