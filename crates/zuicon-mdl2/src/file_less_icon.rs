// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FileLessIcon {}

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

impl Component for FileLessIcon {
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
                data-icon={ "FileLessIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M256 1920h128v128H128V0h1115l549 549v859h-128V640h-512V128H256v1792zM1280 512h293l-293-293v293zM512 384H384V256h128v128zM384 512h128v128H384V512zm128 384H384V768h128v128zm0 512h97v630h-97v-630zm430 128q56 0 95 19t65 52 38 77 12 96v42H819q2 71 43 106t109 35q77 0 143-46v89q-39 25-85 33t-91 9q-59 0-102-19t-72-52-43-80-14-103q0-52 16-98t46-82 74-57 99-21zm103 208q0-25-5-48t-18-40-33-27-48-11q-27 0-48 10t-36 28-25 41-13 47h226zm293-68q0 23 17 37t44 26 57 22 57 28 44 44 18 68q0 41-20 69t-50 46-68 24-71 8q-36 0-71-6t-68-21v-102q30 23 64 35t73 12q15 0 32-2t33-8 26-17 10-32q0-24-17-38t-44-26-57-21-57-28-44-43-18-70q0-39 18-66t48-45 65-26 68-8q31 0 62 4t60 16v98q-53-36-118-36-13 0-29 2t-30 10-24 18-10 28zm422 0q0 24 17 38t44 25 57 22 57 28 43 44 18 68q0 42-19 70t-50 45-68 24-71 8q-35 0-71-6t-68-21v-102q30 23 64 35t73 12q15 0 32-2t33-8 26-17 10-32q0-24-17-38t-44-26-57-21-57-28-44-43-18-70q0-39 18-66t48-45 65-26 68-8q31 0 62 4t60 16v98q-53-36-119-36-13 0-29 2t-30 10-23 18-10 28z" />
            </svg>
        }
    }
}
