// Copyright 2019 The Druid Authors.
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

use std::time::Duration;

use druid::piet::Text;
use druid::widget::{Button, Checkbox, DisabledIf, Flex, Label, LensWrap, Parse, TextBox};
use druid::{AppLauncher, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, Lens, LensExt, LocalizedString, TimerToken, UnitPoint, Widget, WidgetExt, WindowConfig, WindowDesc};

use druid_shell::{WindowLevel};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

const cWINDOW_TITLE: LocalizedString<SaveReminderState> = LocalizedString::new("SaveReminder");

#[derive(Clone, Data, Lens)]
struct SaveReminderState {
    minutes_to_wait: u64,
    repeat: bool,
    timer_started : bool
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(cWINDOW_TITLE)
        .window_size((400.0, 400.0))
        .resizable(false);

    // create the initial app state
    let initial_state = SaveReminderState {
        minutes_to_wait: 5,
        repeat: true,
        timer_started: false
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<SaveReminderState> {
    
    //////////////////////////
    // Title
    //////////////////////////
    let label = Label::new(cWINDOW_TITLE)
    .with_font(FontDescriptor::new(FontFamily::SERIF).with_size(32.0))
    .align_horizontal(UnitPoint::CENTER);


    //////////////////////////
    // Textbox
    //////////////////////////
    let mut input_row = Flex::row();

    input_row.add_child(Label::new("Remind in X Minutes:"));

    input_row.add_child(LensWrap::new(
        Parse::new(TextBox::new()),
        SaveReminderState::minutes_to_wait.map(|x| Some(*x), |x, y| *x = y.unwrap_or(0)),
    ));
        
    //////////////////////////
    // Repeat Checkbox
    //////////////////////////
    let mut repeat_row = Flex::row();
    repeat_row.add_child(Label::new("Should Repeat Reminder:"));
    repeat_row.add_child(LensWrap::new(Checkbox::new(""), SaveReminderState::repeat));

    
    //////////////////////////
    // Buttons
    //////////////////////////
    let mut button_row = Flex::row();

    let start_button = Button::new("Start").on_click(|context, data: &mut SaveReminderState, _env| {
        data.timer_started = true;
    });

    let start_button = DisabledIf::new(start_button, |data, _env| {
        data.timer_started
    });

    button_row.add_child(start_button);

    let stop_button = Button::new("Stop").on_click(|context, data: &mut SaveReminderState, _env|{
        data.timer_started = false;
    });

    let stop_button = DisabledIf::new(stop_button, |data, _env| {
        !data.timer_started
    });

    button_row.add_child(stop_button);

    //////////////////////////
    // Vertical Layout
    //////////////////////////
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(input_row)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(repeat_row)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button_row)
        .align_vertical(UnitPoint::CENTER)
}