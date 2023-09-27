// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Lightbulb {}

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

impl Component for Lightbulb {
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
                data-icon={ "Lightbulb" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 0q97 0 187 25t168 71 143 110 110 142 71 169 25 187q0 145-55 269t-157 225q-83 82-127 183t-45 219v256q0 40-15 75t-41 61-61 41-75 15H832q-40 0-75-15t-61-41-41-61-15-75v-256q0-118-44-219t-128-183q-102-101-157-225t-55-269q0-97 25-187t71-168 110-143T604 96t169-71T960 0zm128 1920q26 0 45-19t19-45v-192H768v192q0 26 19 45t45 19h256zm67-384q13-129 66-234t143-196q83-84 127-183t45-219q0-119-45-224t-124-183-183-123-224-46q-119 0-224 45T553 297 430 480t-46 224q0 119 44 218t128 184q90 91 143 196t66 234h390z" />
            </svg>
        }
    }
}
