// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PythonLogoBlueIcon {}

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

impl Component for PythonLogoBlueIcon {
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
                data-icon={ "PythonLogoBlueIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1510 274v465q0 51-19 96t-52 79-77 54-96 20H778q-61 0-116 24t-98 66-67 96-25 117v223H305q-57 0-100-20t-74-54-53-79-33-95q-15-61-25-123t-10-126q0-63 10-125t25-122q14-56 41-101t67-78 88-51 109-18h672v-62H534V274q0-66 14-110t45-72 76-44 109-28Q835 9 894 5t117-5q64 0 128 4t127 16q48 9 92 30t77 53 54 76 21 95zm-763 62q19 0 36-7t29-20 20-30 7-36q0-19-7-35t-20-30-29-20-36-8q-19 0-36 7t-29 20-20 30-7 36q0 19 7 36t19 29 30 20 36 8z" />
            </svg>
        }
    }
}
