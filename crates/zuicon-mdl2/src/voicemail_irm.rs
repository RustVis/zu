// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct VoicemailIrm {}

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

impl Component for VoicemailIrm {
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
                data-icon={ "VoicemailIRM" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1192 1152q-62 57-107 128H384q-80 0-149-30t-122-82-83-122T0 896q0-79 30-149t82-122 122-83 150-30q79 0 149 30t122 82 83 123 30 149q0 72-26 137t-74 119h524zm-808 0q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20zm1055-136q-31 9-60 20t-58 27q-20-39-30-81t-11-86q0-79 30-149t82-122 122-83 150-30q79 0 149 30t122 82 83 123 30 149q0 62-20 119t-56 108q-50-40-110-69 27-34 42-74t16-84q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 63 31 120zm161 136q93 0 174 35t143 96 96 142 35 175q0 93-35 174t-96 143-142 96-175 35q-93 0-174-35t-143-96-96-142-35-175q0-93 35-174t96-143 142-96 175-35zm0 768q66 0 124-25t101-68 69-102 26-125q0-66-25-124t-69-101-102-69-124-26q-66 0-124 25t-102 69-69 102-25 124q0 66 25 124t68 102 102 69 125 25zm-192-384h384v128h-384v-128z" />
            </svg>
        }
    }
}
