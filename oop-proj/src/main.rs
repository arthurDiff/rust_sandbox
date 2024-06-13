fn main() {
    let screen = oop_proj::Screen {
        components: vec![
            Box::new(oop_proj::Button {
                width: 10,
                height: 10,
                label: String::from("hi button"),
            }),
            Box::new(oop_proj::TextField {
                font_size: 10,
                placeholder: String::from("placeholder over here"),
            }),
        ],
    };
    screen.run();
}
