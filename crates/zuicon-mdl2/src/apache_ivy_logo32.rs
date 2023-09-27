// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ApacheIvyLogo32 {}

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

impl Component for ApacheIvyLogo32 {
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
                data-icon={ "ApacheIvyLogo32" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 945q-17 20-31 42t-30 43q-24 34-50 68t-56 65-64 57-74 42q-44 18-92 31t-96 14q-11 0-21-2t-21-4q127 134 213 295t137 339q3 11 3 22 0 31-23 49t-53 19q-29 0-43-13t-25-38q-6-14-9-28t-9-29q-9-28-19-55t-22-55q-46-112-109-215t-147-192q-3 53-10 106t-20 104q-13 55-47 110t-80 107-95 95-96 76q-28-22-55-67t-51-97-42-105-29-87q-33 18-90 32t-128 26-148 19-149 15-131 10-96 5q15-36 38-90t53-117 64-129 71-125 72-105 69-72q-31-35-58-74t-54-79Q265 762 168 548T0 110q15-5 33-8t37-4 38-2 35-1q42 0 103 3t125 10 123 17 99 23q46 16 105 41t117 59 109 73 77 84q90-31 178-68t172-84 160-102 143-128q34 52 53 109t30 116 14 121 6 121q2 66 25 134t63 130 92 112 111 79z" />
            </svg>
        }
    }
}
