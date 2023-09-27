// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AnalyticsQuery {}

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

impl Component for AnalyticsQuery {
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
                data-icon={ "AnalyticsQuery" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 1024h-128q0-123-32-237t-90-214-141-182-181-140-214-91-238-32q-124 0-238 32t-213 90-182 141-140 181-91 214-32 238q0 124 32 238t90 213 141 182 181 140 214 91 238 32v128q-142 0-272-36t-245-103-207-160-160-208-103-245-37-272q0-141 36-272t103-245 160-207 208-160T752 37t272-37q141 0 272 36t245 103 207 160 160 208 103 245 37 272zM1024 384q133 0 249 50t204 137 137 203 50 250h-128q0-106-40-199t-110-162-163-110-199-41q-106 0-199 40T663 662 553 825t-41 199q0 103 38 196t112 166l-91 91q-91-91-139-208t-48-245q0-133 50-249t137-204 203-137 250-50zm0 384q53 0 99 20t82 55 55 81 20 100q0 53-20 99t-55 82-81 55-100 20q-53 0-99-20t-82-55-55-81-20-100q0-53 20-99t55-82 81-55 100-20zm0 384q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10z" />
            </svg>
        }
    }
}
