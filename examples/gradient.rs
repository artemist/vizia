use vizia::*;

fn main() {
    let window_description = WindowDescription::new().with_title("Gradient");
    Application::new(window_description, |cx| {
        Element::new(cx).background_gradient(
            LinearGradient::new(GradientDirection::TopToBottom)
                .with_stop((0.0, Color::red()))
                .with_stop((1.0, Color::blue())),
        );
    })
    .run();
}
