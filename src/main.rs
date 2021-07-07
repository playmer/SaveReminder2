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

use druid::widget::prelude::*;
use druid::piet::Text;
use druid::widget::{Button, Click, Controller, ControllerHost, Checkbox, DisabledIf, Flex, Label, LabelText, LensWrap, Parse, TextBox};
use druid::{AppLauncher, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, Lens, LensExt, LocalizedString, TimerToken, UnitPoint, Widget, WidgetExt, WindowConfig, WindowDesc};

use druid_shell::{WindowLevel};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

const cWINDOW_TITLE: LocalizedString<SaveReminderState> = LocalizedString::new("SaveReminder");

#[derive(Clone, Data, Lens)]
struct SaveReminderState {
    minutes_to_wait: u64,
    repeat: bool,
    timer_started : bool,
}


/*
pub struct TimerEvent<T> {
    /// A closure that will be invoked when the child widget is clicked.
    action: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
    timer_token : Option<TimerToken>
}

impl<T: Data> TimerEvent<T> {
    /// Create a new clickable [`Controller`] widget.
    pub fn new(action: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> Self {
        TimerEvent {
            timer_token: None,
            action: Box::new(action),
        }
    }

    pub fn set_timer(&mut self, timer_token : TimerToken) {
        self.timer_token = Some(timer_token);
    }
}

impl<T: Data, W: Widget<T>> Controller<T, W> for TimerEvent<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Timer(id) => {
                if self.timer_token.is_some()  && *id == self.timer_token.unwrap() {
                    (self.action)(ctx, data, env);
                }

                self.timer_token = None;
            }
            _ => {}
        }

        child.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &T,
        env: &Env,
    ) {
        if let LifeCycle::HotChanged(_) | LifeCycle::FocusChanged(_) = event {
            ctx.request_paint();
        }

        child.lifecycle(ctx, event, data, env);
    }
}

*/

struct TimerWidget<T>
{
    timer_token : Option<TimerToken>,
    timer_start_stop: Box<dyn Fn(&mut UpdateCtx, &T, &T, &TimerWidget<T>, &Env) -> Option<Option<TimerToken>>>,
    end_timer: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
}


impl<T: Data> TimerWidget<T> {
    pub fn new(timer_start_stop: impl Fn(&mut UpdateCtx, &T, &T, &TimerWidget<T>, &Env) -> Option<Option<TimerToken>> + 'static,
               end_timer: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> Self {
        TimerWidget {
            timer_token : None,
            timer_start_stop : Box::new(timer_start_stop),
            end_timer : Box::new(end_timer)
        }
    }

    pub fn set_timer(&mut self, timer_token : TimerToken) {
        self.timer_token = Some(timer_token);
    }
}

impl<T: Data> Widget<T> for TimerWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Timer(id) => {
                if self.timer_token.is_some() && *id == self.timer_token.unwrap() {
                    (self.end_timer)(ctx, data, env);
                }

                self.timer_token = None;
            }
            _ => (),
        }
    }


    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {

        match (self.timer_start_stop)(ctx, old_data, data, self, env) {
            Some(value) => {
                self.timer_token = value;
                
            },
            _ => (),
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.constrain((0.0, 0.0))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
    }
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
    // timer
    //////////////////////////
    let timer = TimerWidget::new(|ctx : &mut UpdateCtx, old_data : &SaveReminderState, data : &SaveReminderState, widget : &TimerWidget<SaveReminderState>, env : &Env| {
        // Need to make a new timer, since the user hit start.
        if !old_data.timer_started && data.timer_started {
            return Some(Some(ctx.request_timer(Duration::from_secs(data.minutes_to_wait))));
        }

        // Need to wipe out the timer, since the user hit stop.
        if !old_data.timer_started && data.timer_started {
            return Some(None);
        }

        return None;
    }, |ctx : &mut EventCtx, data : &mut SaveReminderState, env : &Env| {
        data.timer_started = false;
    });

    button_row.add_child(timer);

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