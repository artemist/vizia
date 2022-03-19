use vizia::*;

fn main() {
    let window_description = WindowDescription::new().with_inner_size(300, 300);
    Application::new(window_description, |cx| {
        Entity::root().set_background_color(cx, Color::rgb(200, 200, 200));

        ZStack::new(cx, |cx| {
            Element::new(cx).size(Pixels(150.0)).background_color(Color::blue());
            Label::new(cx, "TEST").font_size(30.0);
        });
    })
    //.with_scale_policy(WindowScalePolicy::ScaleFactor(2.0))
    .run();
}
