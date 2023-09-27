// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MailTentativeMirrored {}

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

impl Component for MailTentativeMirrored {
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
                data-icon={ "MailTentativeMirrored" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M0 384v640h128V648l896 447 896-447v888H512v128h1536V384H0zm1024 569L143 512h1762l-881 441zm-640 967H256v128h128v-128zm-64-768q-53 0-99 20t-82 55-55 81-20 100q0 46 14 81t35 63 47 50 46 45 36 45 14 52v48h128v-48q0-47-14-81t-35-63-47-50-46-45-36-45-14-52q0-27 10-50t27-40 41-28 50-10q27 0 50 10t40 27 28 41 10 50h128q0-53-20-99t-55-82-81-55-100-20z" />
            </svg>
        }
    }
}
