use dioxus::prelude::*;

fn main() {
    dioxus::mobile::launch(app);
}

fn app() -> Element {
    rsx! {
        div { "Hello from Dioxus mobile frontend!" }
    }
}
