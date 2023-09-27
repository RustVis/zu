// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ConfigurationSolid {}

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

impl Component for ConfigurationSolid {
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
                data-icon={ "ConfigurationSolid" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1755 512h-475V37l475 475zm-795 520q38 0 71 14t59 40 39 59 15 71q0 38-14 71t-40 59-59 39-71 15q-38 0-71-14t-59-40-39-59-15-71q0-38 14-71t40-59 59-39 71-15zm832-392v1408H128V0h1024v640h640zm-509 632q2-14 3-28t1-28q0-14-1-28t-3-28l185-76-55-134-185 77q-33-46-79-79l77-185-134-55-76 185q-14-2-28-3t-28-1q-14 0-28 1t-28 3l-76-185-134 55 77 185q-46 33-79 79l-185-77-55 134 185 76q-2 14-3 28t-2 28q0 14 1 28t4 28l-185 76 55 134 185-77q33 46 79 79l-77 185 134 55 76-185q14 2 28 3t28 2q14 0 28-1t28-4l76 185 134-55-77-185q46-33 79-79l185 77 55-134-185-76z" />
            </svg>
        }
    }
}
