// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CirclePlus {}

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

impl Component for CirclePlus {
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
                data-icon={ "CirclePlus" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 0q141 0 272 36t244 104 207 160 161 207 103 245 37 272q0 141-36 272t-104 244-160 207-207 161-245 103-272 37q-141 0-272-36t-245-103-207-160-160-208-103-244-37-273q0-141 36-272t104-244 160-207 207-161T752 37t272-37zm0 1920q124 0 238-32t214-90 181-140 140-181 91-214 32-239q0-124-32-238t-90-214-140-181-181-140-214-91-239-32q-124 0-238 32t-214 90-181 140-140 181-91 214-32 239q0 124 32 238t90 214 140 182 181 140 214 90 239 32zm64-961h448v128h-448v449H960v-449H512V959h448V512h128v447z" />
            </svg>
        }
    }
}