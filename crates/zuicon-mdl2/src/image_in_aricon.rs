// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ImageInAricon {}

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

impl Component for ImageInAricon {
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
                data-icon={ "ImageInARIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1472 640q-26 0-45-19t-19-45q0-26 19-45t45-19q26 0 45 19t19 45q0 26-19 45t-45 19zM0 1664V256h1792v645l-128-64V384H128v421l192-191 512 512 256-256 128 128-120 60-8-8-166 166 102 102v181L320 794 128 987v549h896v128H0zm1428-594l172-86 172 86-57 115-115-57-114 57-58-115zm441 49l179 90v199h-128v-57l-51 26-57-115 29-14-29-14 57-115zm51 602v-185h128v264l-179 89-57-115 108-53zm-256 128l51-26 57 115-172 86-172-86 58-115 50 26v-57h128v57zm-384-313v184l108 54-57 115-179-89v-264h128zm51-417l57 115-29 14 29 14-57 115-51-26v57h-128v-200l179-89zm269 250l115-57 57 114-108 54v184h-128v-184l-108-54 57-114 115 57z" />
            </svg>
        }
    }
}
