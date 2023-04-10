// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FogIcon {}

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

impl Component for FogIcon {
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
                data-icon={ "FogIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1761 1036q63 17 115 52t91 85 60 110 21 125q0 33-6 65t-18 63h-139q17-29 26-61t9-67q0-55-20-101t-57-81-83-54-102-20q-12-82-51-152t-98-122-134-81-159-29q-77 0-146 25t-127 69-98 106-61 135q-44-38-97-58t-111-21q-66 0-124 25t-102 68-69 102-25 125q0 52 17 101t49 91H173q-22-45-33-93t-12-99q0-93 35-174t96-142 142-96 175-36q74 0 147 25 39-65 92-117t117-88 136-56 148-20q93 0 178 29t158 81 126 125 83 161zM704 1536q-26 0-45-19t-19-45q0-26 19-45t45-19h896q26 0 45 19t19 45q0 26-19 45t-45 19H704zm1152 128q26 0 45 19t19 45q0 26-19 45t-45 19H320q-26 0-45-19t-19-45q0-26 19-45t45-19h1536zm-384 256q26 0 45 19t19 45q0 26-19 45t-45 19H576q-26 0-45-19t-19-45q0-26 19-45t45-19h896z" />
            </svg>
        }
    }
}
