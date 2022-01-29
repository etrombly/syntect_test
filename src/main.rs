use yew::prelude::*;
use yew::virtual_dom::VNode;
use web_sys::Node;
use once_cell::sync::Lazy;
use instant::Instant;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

static PS: Lazy<SyntaxSet> = Lazy::new(|| {
    SyntaxSet::load_defaults_newlines()
});

static TS: Lazy<ThemeSet> = Lazy::new(|| {
    ThemeSet::load_defaults()
});

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
struct RawHTMLProps {
    pub inner_html: String,
}

struct RawHTML;

impl Component for RawHTML {
    type Message = ();
    type Properties = RawHTMLProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&ctx.props().inner_html[..]);

        let node = Node::from(div);
        let vnode = VNode::VRef(node);
        vnode
    }
}

#[function_component(App)]
fn app() -> Html {
    let now = Instant::now();
    let syntax = PS.find_syntax_by_extension("rs").unwrap();
    let syntax_time = format!("Syntax load: {}", now.elapsed().as_millis());
    let now = Instant::now();
    let theme = &TS.themes["base16-ocean.dark"];
    let theme_time = format!("Theme load: {}", now.elapsed().as_millis());
    let s = "pub struct Wow { hi: u64 }\nfn blah() -> u64 {}";
    let now = Instant::now();
    let html = highlighted_html_for_string(s, &PS, &syntax, theme);
    let high_time = format!("Highlight time: {}", now.elapsed().as_millis());
    html! {
        <>
        <h1>{ syntax_time }</h1>
        <h1>{ theme_time }</h1>
        <h1>{ high_time }</h1>
        <RawHTML inner_html={ html } />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}