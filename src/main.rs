use instant::Instant;
use once_cell::sync::Lazy;
use syntect::dumps::from_binary;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use yew::prelude::*;
use yew::Html;
use web_sys::Element;
use web_sys::HtmlTextAreaElement;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

static PS: Lazy<SyntaxSet> = Lazy::new(|| {
    let ss_bytes = include_bytes!("../assets/ss.bin");
    from_binary(ss_bytes)
});

static TS: Lazy<ThemeSet> = Lazy::new(|| {
    let ts_bytes = include_bytes!("../assets/ts.bin");
    from_binary(ts_bytes)
});

pub enum Msg {
    Update(InputEvent),
}

pub struct App {
    div: Element,
    time: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let div = gloo_utils::document().create_element("div").unwrap();
        Self {
            div,
            time: "0".into(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(e) => {
                let now = Instant::now();
                let event: Event = e.dyn_into().unwrap_throw();
                let event_target = event.target().unwrap_throw();
                let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
                let value = target.value();
                let syntax = PS.find_syntax_by_extension("rs").unwrap();
                let theme = &TS.themes["base16-ocean.dark"];
                let html = highlighted_html_for_string(&value, &PS, syntax, theme);
                self.div.set_inner_html(&html);
                self.time = format!("Highlight time: {}", now.elapsed().as_millis());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tmp = self.div.clone();
        let html = Html::VRef(tmp.into());
        html! {
            <>
            <h1>{ &self.time }</h1>
            <textarea oninput={ctx.link().callback(|e: InputEvent| Msg::Update(e))} style="width: 100%; max-width: 100%;" rows="20"></textarea>
            { html }
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
