// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AnalyticsReport {}

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

impl Component for AnalyticsReport {
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
                data-icon={ "AnalyticsReport" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M128 1024q0 149 46 285t131 250 203 197 260 126v132q-169-44-310-138t-243-226-158-291-57-335q0-141 36-272t104-244 160-207 207-161T752 37t272-37q135 0 260 33t236 95 203 149 161 193 110 230 51 257l-154-154q-38-148-120-272t-198-214-255-139-294-50q-124 0-238 32t-213 90-182 141-140 181-91 214-32 238zm896-512q-106 0-199 40T663 662 553 825t-41 199q0 103 38 196t112 166l-91 91q-91-91-139-208t-48-245q0-88 23-170t64-153 100-129 130-100 153-65 170-23q75 0 147 17t137 51 123 80 102 108h-174q-70-62-155-95t-180-33zm603 256l421 421v859H896V768h731zm37 219v165h165l-165-165zm256 933v-640h-384V896h-512v1024h896zm-128-512v384h-128v-384h128zm-640 384v-512h128v512h-128zm256 0v-256h128v256h-128z" />
            </svg>
        }
    }
}
