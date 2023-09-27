// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FileSass {}

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

impl Component for FileSass {
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
                data-icon={ "FileSass" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M256 2048H128V0h1115l549 549v859h-128V640h-512V128H256v1920zM1280 512h293l-293-293v293zM512 384H384V256h128v128zm0 256H384V512h128v128zM384 768h128v128H384V768zm102 807q0 21 8 36t25 28q21 17 47 27t49 23q29 15 57 32t51 40 37 51 14 65q0 50-20 82t-54 53-74 28-84 8q-16 0-38-2t-47-6-45-11-34-16v-106q13 13 33 23t42 17 45 12 40 4q21 0 44-3t41-12 31-25 13-44q0-30-21-50t-54-38-69-35-70-40-53-57-22-81q0-46 21-78t54-53 73-29 80-10q32 0 72 3t70 19v101q-30-21-65-29t-71-8q-19 0-41 3t-41 13-31 25-13 40zm558-7q86 0 130 42t44 129v298h-102v-71q-47 82-140 82-63 0-103-34t-41-100q0-36 11-62t30-45 48-28 61-16l136-20q0-47-22-72t-71-26q-42 0-79 15t-69 42v-91q38-23 80-33t87-10zm74 244l-97 13q-38 5-64 20t-26 58q0 34 23 50t54 16q25 0 45-9t34-25 23-37 8-46v-40zm330-112q0 22 16 35t41 23 54 21 53 27 41 41 17 64q0 38-19 64t-47 43-64 23-66 7q-33 0-66-5t-64-20v-96q29 21 61 32t68 12q14 0 30-2t31-8 23-17 10-29q0-22-16-35t-42-24-53-20-54-27-41-41-17-64q0-36 17-62t45-42 61-24 64-8q61 0 114 19v91q-50-33-111-33-13 0-28 2t-28 9-21 17-9 27zm378 0q0 22 16 35t41 23 54 21 53 27 41 41 17 64q0 39-18 65t-47 42-63 23-67 7q-33 0-66-5t-64-20v-96q28 21 60 32t68 12q14 0 30-2t31-8 23-17 10-29q0-22-16-35t-41-24-54-20-53-27-41-40-17-65q0-36 17-62t45-42 61-24 64-8q60 0 114 19v91q-50-33-111-33-12 0-27 2t-29 9-22 17-9 27z" />
            </svg>
        }
    }
}
