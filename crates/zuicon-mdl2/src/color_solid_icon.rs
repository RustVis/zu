// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ColorSolidIcon {}

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

impl Component for ColorSolidIcon {
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
                data-icon={ "ColorSolidIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 0q141 0 272 36t244 104 207 160 161 207 103 245 37 272q0 141-36 272t-104 244-160 207-207 161-245 103-272 37q-53 0-99-20t-82-55-55-81-20-100q0-53 13-103t42-96q6-11 11-17t12-16q25-39 37-71t13-81q0-53-20-99t-55-82-81-55-100-20q-48 0-80 12t-72 38q-10 7-16 12t-17 11q-45 28-95 41t-104 14q-53 0-99-20t-82-55-55-81-20-100q0-141 36-272t104-244 160-207 207-161T752 37t272-37zm384 480q-33 0-62 12t-51 35-34 51-13 62q0 33 12 62t35 51 51 34 62 13q33 0 62-12t51-35 34-51 13-62q0-33-12-62t-35-51-51-34-62-13zM512 928q33 0 62-12t51-35 34-51 13-62q0-33-12-62t-35-51-51-34-62-13q-33 0-62 12t-51 35-34 51-13 62q0 33 12 62t35 51 51 34 62 13zm384-256q33 0 62-12t51-35 34-51 13-62q0-33-12-62t-35-51-51-34-62-13q-33 0-62 12t-51 35-34 51-13 62q0 33 12 62t35 51 51 34 62 13zm384 1024q33 0 62-12t51-35 34-51 13-62q0-33-12-62t-35-51-51-34-62-13q-33 0-62 12t-51 35-34 51-13 62q0 33 12 62t35 51 51 34 62 13zm256-384q33 0 62-12t51-35 34-51 13-62q0-33-12-62t-35-51-51-34-62-13q-33 0-62 12t-51 35-34 51-13 62q0 33 12 62t35 51 51 34 62 13z" />
            </svg>
        }
    }
}
