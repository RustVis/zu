// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Border {}

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

impl Component for Border {
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
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("1em") }
                height={ props.height.unwrap_or("1em") }
                focusable={ "false" }
                data-icon={ "border" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M0 0h.969v.5H1v.469H.969V1H.5V.969H0V0zm2.844 1h-.938V0h.938v1zm1.875 0H3.78V0h.938v1zm1.875 0h-.938V0h.938v1zm.937 0V.969H7.5V.5h.031V0h.938v.5H8.5v.469h-.031V1H7.53zm2.813 0h-.938V0h.938v1zm1.875 0h-.938V0h.938v1zm1.875 0h-.938V0h.938v1zM15.5 1h-.469V.969H15V.5h.031V0H16v.969h-.5V1zM1 1.906v.938H0v-.938h1zm6.5.938v-.938h1v.938h-1zm7.5 0v-.938h1v.938h-1zM1 3.78v.938H0V3.78h1zm6.5.938V3.78h1v.938h-1zm7.5 0V3.78h1v.938h-1zM1 5.656v.938H0v-.938h1zm6.5.938v-.938h1v.938h-1zm7.5 0v-.938h1v.938h-1zM.969 8.5H.5v-.031H0V7.53h.5V7.5h.469v.031H1v.938H.969V8.5zm1.875 0h-.938v-1h.938v1zm1.875 0H3.78v-1h.938v1zm1.875 0h-.938v-1h.938v1zm1.875-.031V8.5H7.53v-.031H7.5V7.53h.031V7.5h.938v.031H8.5v.938h-.031zm1.875.031h-.938v-1h.938v1zm1.875 0h-.938v-1h.938v1zm1.875 0h-.938v-1h.938v1zm1.406 0h-.469v-.031H15V7.53h.031V7.5h.469v.031h.5v.938h-.5V8.5zM0 10.344v-.938h1v.938H0zm7.5 0v-.938h1v.938h-1zm8.5-.938v.938h-1v-.938h1zM0 12.22v-.938h1v.938H0zm7.5 0v-.938h1v.938h-1zm8.5-.938v.938h-1v-.938h1zM0 14.094v-.938h1v.938H0zm7.5 0v-.938h1v.938h-1zm8.5-.938v.938h-1v-.938h1zM.969 16H0v-.969h.5V15h.469v.031H1v.469H.969v.5zm1.875 0h-.938v-1h.938v1zm1.875 0H3.78v-1h.938v1zm1.875 0h-.938v-1h.938v1zm.937 0v-.5H7.5v-.469h.031V15h.938v.031H8.5v.469h-.031v.5H7.53zm2.813 0h-.938v-1h.938v1zm1.875 0h-.938v-1h.938v1zm1.875 0h-.938v-1h.938v1zm.937 0v-.5H15v-.469h.031V15h.469v.031h.5V16h-.969z"/>
            </svg>
        }
    }
}
