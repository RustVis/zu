// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Annotation {}

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

impl Component for Annotation {
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
                data-icon={ "Annotation" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1115 1024h677l-128 128H987l-314 313q45 36 70 88t25 111q0 53-20 99t-55 81-82 55-99 21q-53 0-99-20t-81-55-55-81-21-100q0-52 20-99t55-81 81-55 100-21l35 2 258-258H478q-35 59-94 93t-128 35q-53 0-99-20t-81-55-55-81-21-100q0-52 20-99t55-81 81-55 100-21q52 0 99 20t81 55 55 82 21 99h421l512-512h182l-512 512zm-859 128q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10zm256 640q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10zM1920 256h128v128h-128V256zM2048 0v128h-128V0h128zm-384 0h128v128h-128V0zm-128 128h-128V0h128v128zm-256 0h-128V0h128v128zm-256 0H896V0h128v128zm-256 0H640V0h128v128zm-256 0H384V0h128v128zm0 256H384V256h128v128zm0 256H384V512h128v128zm1408-128h128v128h-128V512zm0 256h128v128h-128V768zm0 256h128v128h-128v-128zm0 256h128v128h-128v-128zm0 256h128v128h-128v-128zm-256 0h128v128h-128v-128zm-256 0h128v128h-128v-128zm-256 0h128v128h-128v-128zm-256 0h128v128H896v-128z" />
            </svg>
        }
    }
}
