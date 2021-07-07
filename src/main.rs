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

#![windows_subsystem = "windows"]

use std::time::Duration;

use soloud::*;

use druid::widget::prelude::*;
use druid::widget::{Button, Checkbox, DisabledIf, Flex, Label, LensWrap, Parse, TextBox};
use druid::{AppLauncher, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, Lens, LensExt, LocalizedString, TimerToken, UnitPoint, Widget, WidgetExt, WindowDesc};

//use druid_shell::{WindowState};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

const C_WINDOW_TITLE: LocalizedString<SaveReminderState> = LocalizedString::new("SaveReminder");

#[derive(Clone, Data, Lens)]
struct SaveReminderState {
    minutes_to_wait: u64,
    repeat: bool,
    timer_started : bool,
    main_window_disabled : bool
}


struct TimerWidget<T>
{
    timer_token : Option<TimerToken>,
    timer_start_stop: Box<dyn Fn(&mut UpdateCtx, &T, &T, &Env) -> Option<Option<TimerToken>>>,
    end_timer: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
}


impl<T: Data> TimerWidget<T> {
    pub fn new(timer_start_stop: impl Fn(&mut UpdateCtx, &T, &T, &Env) -> Option<Option<TimerToken>> + 'static,
               end_timer: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> Self {
        TimerWidget {
            timer_token : None,
            timer_start_stop : Box::new(timer_start_stop),
            end_timer : Box::new(end_timer),
        }
    }
}

impl<T: Data> Widget<T> for TimerWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Timer(id) => {
                if self.timer_token.is_some() && *id == self.timer_token.unwrap() {
                    (self.end_timer)(ctx, data, env);
                }
                else {
                    println!("missed timer");
                }

                self.timer_token = None;
            }
            _ => (),
        }
    }


    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {

        match (self.timer_start_stop)(ctx, old_data, data, env) {
            Some(value) => {
                self.timer_token = value;
                
            },
            _ => (),
        }
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        bc.constrain((0.0, 0.0))
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &T, _env: &Env) {
    }
}



struct SoundWidget<T>
{
    update_event : Box<dyn Fn(&mut UpdateCtx, &T, &T, &mut Soloud, &mut audio::Wav, &Env)>,
    soloud : Soloud,
    alarm : audio::Wav
}

impl<T: Data> SoundWidget<T> {
    pub fn new(update_event: impl Fn(&mut UpdateCtx, &T, &T, &mut Soloud, &mut audio::Wav, &Env) + 'static) -> Self {
        let mut widget = SoundWidget {
            update_event : Box::new(update_event),
            soloud : Soloud::default().unwrap(),
            alarm : audio::Wav::default()
        };

        widget.alarm.load("alarm.wav").unwrap();

        return widget;
    }
}

impl<T: Data> Widget<T> for SoundWidget<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {
    }


    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        (self.update_event)(ctx, old_data, data, &mut self.soloud, &mut self.alarm, env)
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        bc.constrain((0.0, 0.0))
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &T, _env: &Env) {
    }
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
    let timer = TimerWidget::new(|ctx : &mut UpdateCtx, old_data : &SaveReminderState, data : &SaveReminderState, _env : &Env| {
        // Need to make a new timer, since the user hit start.
        if !old_data.timer_started && data.timer_started {
            return Some(Some(ctx.request_timer(Duration::from_secs(data.minutes_to_wait))));
        }

        // Need to wipe out the timer, since the user hit stop.
        if old_data.timer_started && !data.timer_started {
            return Some(None);
        }

        return None;
    }, |_context : &mut EventCtx, data : &mut SaveReminderState, _env : &Env| {
        data.main_window_disabled = true;

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
    // Sound
    //////////////////////////
    let sound = SoundWidget::new(|_context : &mut UpdateCtx, old_data : &SaveReminderState, data : &SaveReminderState, soloud : &mut Soloud, wav : &mut audio::Wav, _env : &Env| {
        // Need to make sound, user hasn't acked the timer yet.
        if !old_data.main_window_disabled && data.main_window_disabled {
            soloud.set_looping(soloud.play(wav), true);
        }

        // Need to stop sound, user just acked the timer.
        if old_data.main_window_disabled && !data.main_window_disabled {
            soloud.stop_all();
        }
    });

    button_row.add_child(sound);

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