// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Orientation {}

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

impl Component for Orientation {
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
                data-icon={ "Orientation" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1408 1664H0V768h1408v896zm-128-768H128v640h1152V896zM128 640H0V512h128v128zm0-256H0V256h128v128zm0-256H0V0h128v128zm768 0H768V0h128v128zm-512 0H256V0h128v128zm256 0H512V0h128v128zm256 256H768V256h128v128zm0 256H768V512h128v128zm731-512q102 102 180 197t132 200 81 226 28 273q0 141-36 272t-103 245-160 207-208 160-245 103-272 37v-128q123 0 237-32t214-90 182-141 140-181 91-214 32-238q0-133-25-242t-74-204-120-182-165-177v293h-128V0h512v128h-293z" />
            </svg>
        }
    }
}
