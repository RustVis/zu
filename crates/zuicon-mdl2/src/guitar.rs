// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Guitar {}

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

impl Component for Guitar {
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
                data-icon={ "Guitar" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 320q0 26-19 45l-288 288q-19 19-45 19-29 0-48-22l-502 502h70q26 0 45 19t19 45v128q0 40-15 75t-41 61-61 41-75 15q-26 0-45 19t-19 45q0 93-35 174t-96 143-142 96-175 35q-119 0-224-45t-183-124-123-183-46-224q0-93 35-174t96-143 142-96 175-35q26 0 45-19t19-45q0-53 20-99t55-82 81-55 100-20q66 0 120 30t92 84l418-418q-22-19-22-48 0-26 19-45l288-288q19-19 45-19t45 19l256 256q19 19 19 45zm-896 960H992q-26 0-45-19l-160-160q-19-19-19-45t19-45l99-99q-14-36-46-58t-72-22q-27 0-50 10t-40 27-28 41-10 50q0 40-15 75t-41 61-61 41-75 15q-66 0-124 25t-102 69-69 102-25 124q0 93 35 174t96 142 142 96 175 36q66 0 124-25t101-68 69-102 26-125q0-40 15-75t41-61 61-41 75-15q26 0 45-19t19-45v-64zm-160-154l566-566-70-70-566 566 70 70zm704-608l198-198-166-166-198 198 166 166zM355 1469l90-90 224 224-90 90-224-224zm192-192l90-90 224 224-90 90-224-224z" />
            </svg>
        }
    }
}
