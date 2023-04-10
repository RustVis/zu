// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CheckedOutByOther12icon {}

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

impl Component for CheckedOutByOther12icon {
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
                data-icon={ "CheckedOutByOther12Icon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M939 171q129 0 249 33t224 95 190 146 147 190 94 225 34 249q0 129-33 249t-95 224-146 191-190 147-225 94-249 34q-130 0-250-33t-224-95-190-147-147-190-94-224-34-250q0-129 33-249t95-224 147-190 190-147 224-94 250-34zm0 1740q111 0 213-28t192-81 162-126 125-162 81-191 29-214q0-110-28-212t-81-192-126-162-163-126-191-81-213-29q-111 0-213 28t-192 81-162 126-125 162-81 191-29 214q0 111 28 213t81 192 125 162 163 126 191 80 214 29zm256-1058h170v683H683v-171h391L537 828l121-120 537 537V853z" />
            </svg>
        }
    }
}
