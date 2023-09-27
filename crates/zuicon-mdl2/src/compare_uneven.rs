// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CompareUneven {}

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

impl Component for CompareUneven {
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
                data-icon={ "CompareUneven" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1817 1152h103v128h-22q-20 57-56 104t-85 81-104 52-117 19q-60 0-117-18t-105-52-84-81-56-105h-22v-128h103l187-560-418-139v1213q169 11 317 75t272 179h179v128H128v-128h179q123-114 272-178t317-76V410L502 279l163 489h103v128h-22q-20 57-56 104t-85 81-104 52-117 19q-60 0-117-18t-105-52-84-81-56-105H0V768h103l187-560-262-87L69 0l827 275V0h128v318l867 288-41 122-197-66 164 490zM606 896H162q35 60 94 94t128 34q69 0 128-34t94-94zm-76-128L384 330 238 768h292zm430 1024q-118 0-231 32t-213 96h888q-99-63-213-95t-231-33zm576-1078l-146 438h292l-146-438zm222 566h-444q35 60 94 94t128 34q69 0 128-34t94-94z" />
            </svg>
        }
    }
}
