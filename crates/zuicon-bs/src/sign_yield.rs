// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SignYield {}

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

impl Component for SignYield {
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
                data-icon={ "sign-yield" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M5.506 6.232V7H5.11v-.76L4.44 5h.44l.424.849h.016L5.748 5h.428l-.67 1.232ZM6.628 5v2h-.396V5h.396Zm.684 1.676h.895V7H6.919V5h1.288v.324h-.895v.513h.842v.303h-.842v.536Zm1.521-.013h.848V7H8.437V5h.396v1.663Z"/>
  <path fill-rule="evenodd" d="M9.804 7V5h.73c.607 0 .894.364.894.995 0 .636-.291 1.005-.895 1.005h-.73Zm.676-1.677h-.28v1.353h.28c.372 0 .54-.222.54-.674 0-.45-.169-.68-.54-.68Z"/>
  <path fill-rule="evenodd" d="M7.022 14.434a1.131 1.131 0 0 0 1.96 0l6.857-11.667c.457-.778-.092-1.767-.98-1.767H1.144c-.889 0-1.437.99-.98 1.767l6.857 11.667Zm.98-.434a.13.13 0 0 1-.064-.016.146.146 0 0 1-.054-.057L1.027 2.26a.177.177 0 0 1-.002-.183.164.164 0 0 1 .054-.06A.116.116 0 0 1 1.145 2h13.713a.12.12 0 0 1 .066.017c.018.01.038.03.055.06a.176.176 0 0 1-.003.183L8.12 13.927a.146.146 0 0 1-.054.057.13.13 0 0 1-.063.016Z"/>
            </svg>
        }
    }
}
