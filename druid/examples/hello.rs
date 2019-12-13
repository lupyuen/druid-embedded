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

#![no_main] ////
#![no_std]  //  This program will run on embedded platforms
use core::panic::PanicInfo; ////  Import `PanicInfo` type which is used by `panic()` below
use druid::widget::{Align, Button, Column, Flex, Label, Padding};
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};
use cortex_m_rt::entry; ////

#[entry]
fn main() -> ! {
    //  Build a new window
    let main_window = WindowDesc::<u32,Flex<u32>>::new(ui_builder);
    //  Application state is initially 0
    let data = 0_u32;
    //  Launch the window with the initial application state
    AppLauncher::<u32,Flex<u32>>::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
    loop {} ////
}

/// Build the UI for the window. The application state consists of 1 value: `count` of type `u32`.
fn ui_builder() -> Flex<u32> {  //  `u32` is the window state
    //  Create a line of text based on a counter value
    let text =
        LocalizedString::<u32>::new("hello-counter")
        .with_arg(
            "count", 
            //  Closure that will fetch the counter value...
            |data: &u32, _env| (*data).into()
        );
    //  Create a label widget to display the text
    let label = Label::<u32>::new(text);
    //  Create a button widget to increment the counter
    let button = Button::<u32>::new(
        "increment", 
        //  Closure that will be called when button is tapped...
        |_ctx, data, _env| *data += 1
    );

    //  Create a column for the UI
    let mut col = Column::new::<u32>();
    //  Add the label widget to the column, centered with padding
    col.add_child::<Align::<u32>>(
        Align::<u32>::centered(
            Padding::<u32>::new(5.0, label)
        ),
        1.0
    );
    //  Add the button widget to the column, with padding
    col.add_child::<Padding::<u32>>(
        Padding::<u32>::new(5.0, button), 
        1.0
    );
    //  Return the column containing the label and button widgets
    col
}

///  This function is called on panic, like an assertion failure. We display the filename and line number and pause in the debugger. From https://os.phil-opp.com/freestanding-rust-binary/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //  Display the filename and line number to the Semihosting Console.
    //console::print("panic ");
    if let Some(location) = info.location() {
        let file = location.file();
        let line = location.line();
        //console::print("at ");       console::buffer(&file);
        //console::print(" line ");    console::printint(line as i32);
        //console::print("\n");        console::flush();
    } else {
        //console::print("no loc\n");  console::flush();
    }
    //  Pause in the debugger.
    //bkpt();
    //  Loop forever so that device won't restart.
    loop {}
}

/*
#[entry]
fn main() -> ! {
    //  Build a new window
    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    //  Launch the window
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
    loop {}
}

/// Build the UI for the window. The window state consists of 1 value: `count` of type `u32`.
fn ui_builder() -> impl Widget<u32> {  //  `u32` is the window state
    //  Create a line of text based on a counter value
    let text =
        LocalizedString::new("hello-counter")
        .with_arg(
            "count", 
            |data: &u32, _env| (*data).into()
        );
    //  Create a label widget to display the text
    let label = Label::new(text);
    //  Create a button widget to increment the counter
    let button = Button::new(
        "increment", 
        |_ctx, data, _env| *data += 1
    );

    //  Create a column for the UI
    let mut col = Column::new();
    //  Add the label and button widgets to the column
    col.add_child(
        Align::centered(
            Padding::new(5.0, label)
        ),
        1.0
    );
    col.add_child(
        Padding::new(5.0, button), 
        1.0
    );
    col
}
*/