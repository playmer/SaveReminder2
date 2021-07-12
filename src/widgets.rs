
pub mod widget {
    use druid::widget::prelude::*;
    use druid::{Data, Env, Event, EventCtx, TimerToken, Widget};
    pub struct TimerWidget<T, D>
    {
        timer_token : Option<TimerToken>,
        timer_start_stop: Box<dyn Fn(&mut UpdateCtx, &mut D, &mut Option<TimerToken>, &T, &T, &Env)>,
        end_timer: Box<dyn Fn(&mut EventCtx, &mut D, &mut T, &Env)>,
        user_data : D,
    }

    impl<T: Data, D> TimerWidget<T, D> {
        pub fn new(user_data : D,
                timer_start_stop: impl Fn(&mut UpdateCtx, &mut D, &mut Option<TimerToken>, &T, &T, &Env) + 'static,
                end_timer: impl Fn(&mut EventCtx, &mut D, &mut T, &Env) + 'static) -> Self {
            TimerWidget {
                timer_token : None,
                timer_start_stop : Box::new(timer_start_stop),
                end_timer : Box::new(end_timer),
                user_data : user_data
            }
        }
    }

    impl<T: Data, D> Widget<T> for TimerWidget<T, D> {
        fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
            match event {
                Event::Timer(id) => {
                    if self.timer_token.is_some() && *id == self.timer_token.unwrap() {
                        (self.end_timer)(ctx, &mut self.user_data, data, env);
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
            (self.timer_start_stop)(ctx, &mut self.user_data, &mut self.timer_token, old_data, data, env);
        }

        fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
            bc.constrain((0.0, 0.0))
        }

        fn paint(&mut self, _ctx: &mut PaintCtx, _data: &T, _env: &Env) {
        }
    }
}
