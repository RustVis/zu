// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct RedEye12icon {}

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

impl Component for RedEye12icon {
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
                data-icon={ "RedEye12Icon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 512q150 0 292 39t272 107 246 163 214 203q-98 110-213 203t-246 162-273 108-292 39q-150 0-292-39t-272-107-247-162T0 1024q97-109 213-203t246-162 273-108 292-39zm0 878q76 0 142-29t116-78 79-116 29-143q0-76-29-142t-78-116-116-79-143-29q-76 0-142 29t-116 78-79 116-29 143q0 76 29 142t78 116 116 79 143 29zm-375-54q-55-66-84-146t-29-166q0-86 29-166t84-146q-134 53-251 132t-221 180q103 100 220 179t252 133zm1222-312q-103-100-220-179t-252-133q55 66 84 146t29 166q0 86-29 166t-84 146q134-53 251-132t221-180zm-847-171q35 0 66 13t54 37 37 55 14 66q0 35-13 66t-37 55-55 36-66 14q-35 0-66-13t-55-37-36-54-14-67q0-35 13-66t37-54 54-37 67-14z" />
            </svg>
        }
    }
}
