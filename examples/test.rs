use vizia::*;

#[derive(Lens)]
pub struct AppData {
    val: f32,
}

pub enum AppEvent {
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetValue(val) => {
                    self.val = *val;
                }
            }
        }
    }
}

const STYLE: &str = r#"
    element {
        background-color: red;
        transition: background-color 1.0 0.0;
    }

    element:hover {
        background-color: blue;
        transition: background-color 1.0 0.0;
    }

    element:active {
        background-color: green;
        transition: background-color 1.0 0.0;
    }
"#;

fn main() {
    let mut window_description = WindowDescription::new().with_inner_size(300, 300);
    Application::new(window_description, |cx| {
        //cx.add_theme(STYLE);

        Entity::root().set_background_color(cx, Color::rgb(200, 200, 200));

        Element::new(cx).size(Pixels(150.0)).background_color(Color::blue()).on_geo_changed(
            |cx, geo| {
                Label::new(cx, &format!("{}", cx.cache.get_width(cx.current)));
            },
        );
        // AppData { val: 0.5 }.build(cx);

        // HStack::new(cx, |cx| {
        //     Slider::new(cx, AppData::val, Orientation::Horizontal)
        //         .on_changing(|cx, val| cx.emit(AppEvent::SetValue(val)));
        //     Textbox::new(cx, AppData::val.map(|val| format!("{:.2}", val)))
        //         .on_submit(|cx, txt| {
        //             if let Ok(val) = txt.parse::<f32>() {
        //                 let val = val.clamp(0.0, 1.0);
        //                 cx.emit(AppEvent::SetValue(val));
        //             }
        //         })
        //         .width(Pixels(100.0));
        // })
        // .height(Auto)
        // .col_between(Pixels(20.0))
        // .child_space(Pixels(20.0));
    })
    .with_scale_policy(WindowScalePolicy::ScaleFactor(1.0))
    .run();
}
