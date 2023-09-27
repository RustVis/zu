// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Lifesaver {}

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

impl Component for Lifesaver {
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
                data-icon={ "Lifesaver" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 0q141 0 272 36t245 103 207 160 160 208 103 245 37 272q0 141-36 272t-103 245-160 207-208 160-245 103-272 37q-141 0-272-36t-245-103-207-160-160-208-103-244-37-273q0-141 36-272t103-245 160-207 208-160T751 37t273-37zm896 1024q0-72-11-143t-35-140q-61 20-121 40t-122 40q33 98 33 203 0 106-33 203l243 80q23-69 34-140t12-143zm-1792 0q0 72 11 143t35 140q61-20 121-40t122-40q-33-97-33-203 0-105 33-203l-243-80q-23 69-34 140t-12 143zm384 0q0 106 40 199t110 162 163 110 199 41q106 0 199-40t162-110 110-163 41-199q0-106-40-199t-110-162-163-110-199-41q-106 0-199 40T663 662 553 825t-41 199zm795-850q-69-23-140-34t-143-12q-72 0-143 11t-140 35q20 61 40 121t40 122q97-33 203-33 105 0 203 33l80-243zM741 1874q69 23 140 34t143 12q72 0 143-11t140-35q-20-61-40-121t-40-122q-98 33-203 33-106 0-203-33l-80 243z" />
            </svg>
        }
    }
}
