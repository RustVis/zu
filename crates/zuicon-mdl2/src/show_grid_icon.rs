// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ShowGridIcon {}

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

impl Component for ShowGridIcon {
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
                data-icon={ "ShowGridIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1664 1024V896h128v128h-128zm0-1024v128h-128V0h128zm256 1152h-128v-128h128v128zM1408 0v128h-128V0h128zm-256 1024V896h128v128h-128zM896 0v128H768V0h128zm512 1024V896h128v128h-128zm512-256v128h-128V768h128zm-896 128v128H896V896h128zm0-256v128H896V640h128zm0-256v128H896V384h128zM1152 0v128h-128V0h128zm768 512v128h-128V512h128zM896 1255v-103h128v26q-67 33-128 77zm1024-999v128h-128V256h128zm0-256v128h-128V0h128zM0 1408v-128h128v128H0zm0 256v-128h128v128H0zm0-512v-128h128v128H0zm256-256v128H128V896h128zM0 768h128v128H0V768zm256 1152v-128h128v128H256zm256 0v-128h128v128H512zm0-1024v128H384V896h128zM640 0v128H512V0h128zM384 0v128H256V0h128zm640 256H896V128h128v128zM768 896v128H640V896h128zM128 0v128H0V0h128zM0 1920v-128h128v128H0zM128 512v128H0V512h128zm0-256v128H0V256h128zm1920 1536h-128q0-66-21-122t-59-103-87-82-107-60-118-36-120-13q-59 0-120 12t-118 37-106 59-87 82-59 103-22 123H768q0-84 26-157t71-133 107-108 133-79 148-50 155-17q77 0 154 17t149 49 132 80 107 107 72 134 26 157zm-640-256q53 0 100 20t81 54 55 82 20 100q0 53-20 100t-54 81-82 55-100 20q-53 0-100-20t-81-54-55-82-20-100q0-53 20-100t54-81 82-55 100-20zm0 384q26 0 49-10t41-27 28-41 10-50q0-26-10-49t-27-41-41-28-50-10q-26 0-49 10t-41 27-28 41-10 50q0 26 10 49t27 41 41 28 50 10z" />
            </svg>
        }
    }
}
