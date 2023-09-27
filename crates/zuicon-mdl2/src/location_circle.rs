// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct LocationCircle {}

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

impl Component for LocationCircle {
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
                data-icon={ "LocationCircle" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 128q123 0 237 32t214 90 182 141 140 181 91 214 32 238q0 123-32 237t-90 214-141 182-181 140-214 91-238 32q-123 0-237-32t-214-90-182-141-140-181-91-214-32-238q0-123 32-237t90-214 141-182 181-140 214-91 238-32zm0 1664q106 0 204-27t183-78 156-120 120-155 77-184 28-204q0-106-27-204t-78-183-120-156-155-120-184-77-204-28q-106 0-204 27t-183 78-156 120-120 155-77 184-28 204q0 106 27 204t78 183 120 156 155 120 184 77 204 28z" />
            </svg>
        }
    }
}
