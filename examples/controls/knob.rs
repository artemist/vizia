use vizia::*;

const STYLE: &str = r#"

    knob {
        width: 76px;
        height: 76px;
        background-color: #262a2d;
        border-radius: 38px;
        border-width: 2px;
        border-color: #363636;
    }
    
    knob .track {
        background-color: #ffb74d;
    }

"#;

#[derive(Lens)]
pub struct AppData {
    value: f32,
}

#[derive(Debug)]
pub enum AppEvent {
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetValue(value) => {
                    self.value = *value;
                }
            }
        }
    }
}

fn main() {
    Application::new(WindowDescription::new().with_title("Knob"), |cx| {
        cx.add_theme(STYLE);

        if cx.data::<AppData>().is_none() {
            AppData { value: 0.2 }.build(cx);
        }

        Knob::new(cx, 0.5, AppData::value, false).on_changing(|cx, val| {
            cx.emit(AppEvent::SetValue(val));
        });
        Knob::new(cx, 0.5, AppData::value, true).on_changing(|cx, val| {
            cx.emit(AppEvent::SetValue(val));
        });

        //ArcTrack::new(cx).width(Pixels(50.0)).height(Pixels(50.0)).space(Pixels(20.0));
    })
    .run();
}
