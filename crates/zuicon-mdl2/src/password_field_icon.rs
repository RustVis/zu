// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PasswordFieldIcon {}

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

impl Component for PasswordFieldIcon {
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
                data-icon={ "PasswordFieldIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M0 256h2048v1408H0V256zm1920 1280V384H128v1152h1792zM512 1120q-33 0-62-12t-51-35-34-51-13-62q0-33 12-62t35-51 51-34 62-13q33 0 62 12t51 35 34 51 13 62q0 33-12 62t-35 51-51 34-62 13zm512 0q-33 0-62-12t-51-35-34-51-13-62q0-33 12-62t35-51 51-34 62-13q33 0 62 12t51 35 34 51 13 62q0 33-12 62t-35 51-51 34-62 13zm512 0q-33 0-62-12t-51-35-34-51-13-62q0-33 12-62t35-51 51-34 62-13q33 0 62 12t51 35 34 51 13 62q0 33-12 62t-35 51-51 34-62 13z" />
            </svg>
        }
    }
}
