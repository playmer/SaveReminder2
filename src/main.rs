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





/*


struct TimerWidget {
    timer_id: TimerToken,
    minutes_to_wait: u64,
    repeat: bool
}

impl Widget<u32> for TimerWidget {
    fn event(&mut self, context: &mut EventCtx, event: &Event, data: &mut u32, env: &Env) {
        match event {
            Event::WindowConnected => {
                // Start the timer when the application launches
                self.timer_id = context.request_timer(minutes_to_wait);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    self.adjust_box_pos(context.size());
                    context.request_layout();
                    self.timer_id = context.request_timer(minutes_to_wait);
                }
            }
            _ => (),
        }
    }
}






*/

use std::time::Duration;

use druid::piet::Text;
use druid::widget::{Button, Checkbox, DisabledIf, Flex, Label, LensWrap, Parse, TextBox};
use druid::{AppLauncher, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, Lens, LensExt, LocalizedString, TimerToken, UnitPoint, Widget, WidgetExt, WindowConfig, WindowDesc};

use druid_shell::{WindowLevel};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<SaveReminderState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct SaveReminderState {
    minutes_to_wait: u64,
    repeat: bool,
    timer_started : bool
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

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

fn build_modal_error_widget() -> impl Widget<SaveReminderState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new("Error: entered value isn't a number.")
    .align_horizontal(UnitPoint::CENTER);
    let ok_button = Button::new("Ok").on_click(|context, data: &mut SaveReminderState, _env| {
        context.window().close();
    });

    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(ok_button)
        .align_vertical(UnitPoint::CENTER)
}

fn build_root_widget() -> impl Widget<SaveReminderState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new("SaveReminder")
    .with_font(FontDescriptor::new(FontFamily::SERIF).with_size(32.0))
    .align_horizontal(UnitPoint::CENTER);

    let mut input_row = Flex::row();

    //let parser = Parse::new(TextBox::new());
    //input_row.add_child();


    input_row.add_child(Label::new("Remind in X Minutes:"));
    // a textbox that modifies `minutes_to_wait`.
    let textbox = TextBox::new()
        .with_placeholder("5")
        .with_text_size(18.0);
        
    let textbox = LensWrap::new(
        Parse::new(TextBox::new()),
        SaveReminderState::minutes_to_wait.map(|x| {
            String::from("")
        }, |x, y| {
            *x = y.parse::<u64>().unwrap();
        })
    );

    input_row.add_child(textbox);
        
    let mut repeat_row = Flex::row();
    repeat_row.add_child(Label::new("Should Repeat Reminder:"));
    repeat_row.add_child(LensWrap::new(Checkbox::new(""), SaveReminderState::repeat));

    let mut button_row = Flex::row();

    let start_button = Button::new("Start").on_click(|context, data: &mut SaveReminderState, env| {
        data.timer_started = true;
        //match data.minutes_to_wait.parse::<u64>() {
        //    Ok(minutes_to_wait_num) => {
        //        println!("{}", minutes_to_wait_num);
        //        //let mut timer = context.request_timer(Duration::from_secs(minutes_to_wait_num * 60));
        //        data.timer_started = true;
        //    }
        //    Err(_e) => {
        //        data.minutes_to_wait = 5;
        //        context.new_sub_window(
        //            WindowConfig::default()
        //                .show_titlebar(false)
        //                .set_level(WindowLevel::Modal)
        //                .window_size((240.0,100.0))
        //                .set_position(
        //                    context.window().get_position() + (100.0, 100.0)),
        //            build_modal_error_widget(),
        //            data.clone(),
        //            env.clone(),
        //        );
        //    },
        //}
    });

    let start_button = DisabledIf::new(start_button, |data, env| {
        data.timer_started
    });

    button_row.add_child(start_button);

    let stop_button = Button::new("Stop").on_click(|context, data: &mut SaveReminderState, env|{
        data.timer_started = false;
    });

    let stop_button = DisabledIf::new(stop_button, |data, env| {
        !data.timer_started
    });

    button_row.add_child(stop_button);

    // arrange the two widgets vertically, with some padding
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