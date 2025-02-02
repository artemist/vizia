use vizia::*;

const STYLE: &'static str = r#"
element {
    background-image: "sample.png";
    width: 1s;
    height: 1s;
}
"#;

fn main() {
    Application::new(WindowDescription::default(), |cx| {
        cx.add_theme(STYLE);
        cx.set_image_loader(|cx, path| {
            if path.starts_with("https://") {
                let path = path.to_string();
                cx.spawn(move |cx| {
                    let data = reqwest::blocking::get(&path).unwrap().bytes().unwrap();
                    cx.load_image(
                        path.clone(),
                        image::load_from_memory_with_format(
                            &data,
                            image::guess_format(&data).unwrap(),
                        )
                        .unwrap(),
                        ImageRetentionPolicy::DropWhenUnusedForOneFrame,
                    )
                    .unwrap();
                });
            } else if path == "sample.png" {
                cx.load_image(
                    path.to_owned(),
                    image::load_from_memory_with_format(
                        include_bytes!("../resources/sample-hut-400x300.png"),
                        image::ImageFormat::Png,
                    )
                    .unwrap(),
                    ImageRetentionPolicy::DropWhenUnusedForOneFrame,
                );
            } else {
                panic!();
            }
        });

        Element::new(cx);
        Image::new(cx, "https://download.samplelib.com/png/sample-bumblebee-400x300.png");
        Label::new(cx, "Wait for the image to load :)");
    })
    .run()
}
