// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Header3 {}

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

impl Component for Header3 {
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
                data-icon={ "Header3" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M768 384h128v1408H768v-640H128v640H0V384h128v640h640V384zm851 674q67 7 125 32t102 67 69 99 25 127q0 103-39 179t-106 128-153 76-181 26q-80 0-160-15t-149-59v-167q138 108 315 108 61 0 118-15t100-48 70-81 26-118q0-69-22-116t-61-76-88-45-105-22-111-7-106-1V998h98q51 0 101-7t94-20 77-43 53-71 20-109q0-62-18-106t-51-72-80-41-107-13q-76 0-143 25t-128 72V462q71-42 150-60t160-18q74 0 142 20t121 61 83 102 31 142q0 139-70 223t-202 122v4z" />
            </svg>
        }
    }
}
