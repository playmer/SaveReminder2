// Copyright 2019 Joshua T. Fisher.
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

#![windows_subsystem = "windows"]

mod widgets;

use std::time::Duration;

use soloud::*;

use druid::widget::prelude::*;
use druid::widget::{Button, Checkbox, DisabledIf, Flex, Label, LensWrap, Parse, TextBox};
use druid::{AppLauncher, Data, Env, EventCtx, FontDescriptor, FontFamily, Lens, LensExt, LocalizedString, TimerToken, UnitPoint, Widget, WidgetExt, WindowDesc};

use crate::widgets::widget::*;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

const C_WINDOW_TITLE: LocalizedString<SaveReminderState> = LocalizedString::new("SaveReminder");

#[derive(Clone, Data, Lens)]
struct SaveReminderState {
    minutes_to_wait: u64,
    repeat: bool,
    timer_started : bool,
    main_window_disabled : bool
}

struct TimerUserData{
    soloud : Soloud,
    alarm : audio::Wav
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(C_WINDOW_TITLE)
        .window_size((400.0, 400.0))
        .resizable(false);
        
    // create the initial app state
    let initial_state = SaveReminderState {
        minutes_to_wait: 5,
        repeat: true,
        timer_started: false,
        main_window_disabled : false
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
    let label = Label::new(C_WINDOW_TITLE)
    .with_font(FontDescriptor::new(FontFamily::SERIF).with_size(32.0))
    .align_horizontal(UnitPoint::CENTER);

    //////////////////////////
    // Textbox
    //////////////////////////
    let mut input_row = Flex::row();

    input_row.add_child(Label::new("Remind in X Minutes:"));
    let textbox = LensWrap::new(
        Parse::new(TextBox::new()),
        SaveReminderState::minutes_to_wait.map(|x| Some(*x), |x, y| *x = y.unwrap_or(0)),
    );
    
    // Disable the textbox if the timer has started.
    input_row.add_child(DisabledIf::new(textbox, |data, _env| {
        data.timer_started
    }));
        
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

    let start_button = Button::new("Start").on_click(|_context, data: &mut SaveReminderState, _env| {
        data.timer_started = true;
    });

    let start_button = DisabledIf::new(start_button, |data, _env| {
        data.timer_started
    });

    button_row.add_child(start_button);

    let stop_button = Button::new("Stop").on_click(|_context, data: &mut SaveReminderState, _env|{
        data.timer_started = false;
    });

    let stop_button = DisabledIf::new(stop_button, |data, _env| {
        !data.timer_started
    });

    button_row.add_child(stop_button);

    //////////////////////////
    // Timer
    //////////////////////////

    let mut user_data = TimerUserData {
        soloud : Soloud::default().unwrap(),
        alarm : audio::Wav::default()
    };

    user_data.alarm.load("alarm.wav").unwrap();

    let timer = TimerWidget::new(user_data, |ctx : &mut UpdateCtx, user_data : &mut TimerUserData, timer_token : &mut Option<TimerToken>, old_data : &SaveReminderState, data : &SaveReminderState, _env : &Env| {
        // Need to make a new timer, since the user hit start.
        if !old_data.timer_started && data.timer_started {
            *timer_token = Some(ctx.request_timer(Duration::from_secs(60 * data.minutes_to_wait)));
        }

        // Need to wipe out the timer, since the user hit stop.
        if old_data.timer_started && !data.timer_started {
            *timer_token = None;
        }

        // Need to stop sound, user just acked the timer.
        if old_data.main_window_disabled && !data.main_window_disabled {
            user_data.soloud.stop_all();
        }
    }, |_context : &mut EventCtx, user_data : &mut TimerUserData, data : &mut SaveReminderState, _env : &Env| {
        data.main_window_disabled = true;
        
        user_data.soloud.set_looping(user_data.soloud.play(&user_data.alarm), true);

        // Make a modal Window?
        //ctx.new_sub_window(
        //    WindowConfig::default()
        //        //.show_titlebar(false)
        //        .set_level(WindowLevel::Modal)
        //        .window_size((240.0,130.0))
        //        .resizable(false)
        //        .set_position(
        //            ctx.window().get_position() + (100.0, 100.0)).set_window_state(WindowState::Restored),
        //         build_modal_timer_go_off_widget(),
        //    data,
        //    env.clone(),
        //);
        //ctx.

        // Bring window to Front?
        //let handle = _context.window();//.set_window_state(WindowState::Restored);
        //handle.set_window_state(WindowState::Restored);
        data.timer_started = false;
    });

    button_row.add_child(timer);

    //////////////////////////
    // Vertical Layout
    //////////////////////////
    let main_column = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(input_row)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(repeat_row)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button_row)
        .align_vertical(UnitPoint::CENTER);

    let main_column = DisabledIf::new(main_column , |data, _env| {
            data.main_window_disabled
    });
    
    let ack_button = Button::new("Acknowledge Timer").on_click(|_context, data: &mut SaveReminderState, _env|{
        data.main_window_disabled = false;

        if data.repeat {
            data.timer_started = true
        }
    });

    let ack_button = DisabledIf::new(ack_button, |data, _env| {
        !data.main_window_disabled
    });

    Flex::column()
        .with_child(main_column)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(ack_button)
        .align_vertical(UnitPoint::CENTER)
}