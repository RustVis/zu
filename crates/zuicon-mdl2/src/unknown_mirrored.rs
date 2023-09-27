// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct UnknownMirrored {}

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

impl Component for UnknownMirrored {
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
                data-icon={ "UnknownMirrored" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 4q132 0 254 34t228 96 194 150 149 193 97 229 34 254q0 132-34 254t-96 228-150 194-193 149-229 97-254 34q-132 0-254-34t-228-96-194-150-149-193-97-229T4 960q0-132 34-254t96-228 150-194 193-149 229-97T960 4zm0 1792q115 0 222-30t200-84 169-131 130-169 85-200 30-222q0-115-30-222t-84-200-131-169-169-130-200-85-222-30q-115 0-222 30t-200 84-169 131-130 169-85 200-30 222q0 115 30 222t84 200 131 169 169 130 200 85 222 30zm-64-388h128v128H896v-128zm64-960q66 0 124 25t101 69 69 102 26 124h-128q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 42 19 75t47 63 62 59 61 66 48 80 19 105v64H896v-64q0-42-19-75t-47-63-62-59-61-66-48-80-19-105q0-66 25-124t68-101 102-69 125-26z" />
            </svg>
        }
    }
}
