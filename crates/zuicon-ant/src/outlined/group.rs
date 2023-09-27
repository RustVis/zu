// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Group {}

#[derive(Properties, Debug, Clone, PartialEq, Eq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,

    #[prop_or_default]
    pub width: Option<&'static str>,

    #[prop_or_default]
    pub height: Option<&'static str>,

    #[prop_or_default]
    pub color: Option<&'static str>,

    #[prop_or_default]
    pub fill: Option<&'static str>,

    #[prop_or_default]
    pub spin: bool,

    #[prop_or_default]
    pub rotate: i16,
}

impl Component for Group {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        // TODO(Shaohua): Do not generate style attribute if it is empty.
        let mut style = String::new();
        if props.rotate != 0 {
            style += &format!("transform: rotate({}deg);", props.rotate);
        }
        html! {
            <svg
                xmlns={ "http://www.w3.org/2000/svg" }
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("16") }
                height={ props.height.unwrap_or("16") }
                focusable={ "false" }
                data-icon={ "group" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <defs><style type="text/css"></style></defs><path d="M912 820.1V203.9c28-9.9 48-36.6 48-67.9 0-39.8-32.2-72-72-72-31.3 0-58 20-67.9 48H203.9C194 84 167.3 64 136 64c-39.8 0-72 32.2-72 72 0 31.3 20 58 48 67.9v616.2C84 830 64 856.7 64 888c0 39.8 32.2 72 72 72 31.3 0 58-20 67.9-48h616.2c9.9 28 36.6 48 67.9 48 39.8 0 72-32.2 72-72 0-31.3-20-58-48-67.9zM888 112c13.3 0 24 10.7 24 24s-10.7 24-24 24-24-10.7-24-24 10.7-24 24-24zM136 912c-13.3 0-24-10.7-24-24s10.7-24 24-24 24 10.7 24 24-10.7 24-24 24z m0-752c-13.3 0-24-10.7-24-24s10.7-24 24-24 24 10.7 24 24-10.7 24-24 24z m704 680H184V184h656v656z m48 72c-13.3 0-24-10.7-24-24s10.7-24 24-24 24 10.7 24 24-10.7 24-24 24z" p-id="13901"></path><path d="M288 474h448c8.8 0 16-7.2 16-16V282c0-8.8-7.2-16-16-16H288c-8.8 0-16 7.2-16 16v176c0 8.8 7.2 16 16 16z m56-136h336v64H344v-64zM288 758h448c8.8 0 16-7.2 16-16V566c0-8.8-7.2-16-16-16H288c-8.8 0-16 7.2-16 16v176c0 8.8 7.2 16 16 16z m56-136h336v64H344v-64z" p-id="13902"></path>
            </svg>
        }
    }
}
