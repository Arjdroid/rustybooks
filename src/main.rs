use gpui::{
    div, prelude::*, px, rgb, size, App, AppContext, Bounds, SharedString, ViewContext,
    WindowBounds, WindowOptions,
};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x121212))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0xff5252))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(div().size_8().bg(rgb(0xcc6666)))
                    .child(div().size_8().bg(rgb(0xb5bd68)))
                    .child(div().size_8().bg(rgb(0x81a2be)))
                    .child(div().size_8().bg(rgb(0xf0c674)))
                    .child(div().size_8().bg(rgb(0x000000)))
                    .child(div().size_8().bg(rgb(0xffffff))),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| HelloWorld {
                    text: "World".into(),
                })
            },
        )
        .unwrap();
    });
}
