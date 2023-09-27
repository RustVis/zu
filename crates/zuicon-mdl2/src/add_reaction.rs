// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AddReaction {}

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

impl Component for AddReaction {
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
                data-icon={ "AddReaction" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1792 896q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10zm1408 128q0 142-36 272t-103 245-160 207-208 160-245 103-272 37q-142 0-272-36t-245-103-207-160-160-208-103-245-37-272q0-141 36-272t103-245 160-207 208-160 245-103 272-37q141 0 272 36t245 103 207 160 160 208 103 245 37 272zm-1024 896q123 0 237-32t214-90 182-141 140-181 91-214 32-238q0-123-32-237t-90-214-141-182-181-140-214-91-238-32q-124 0-238 32t-213 90-182 141-140 181-91 214-32 238q0 124 32 238t90 213 141 182 181 140 214 91 238 32zm384-1024q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10zm-384 640q73 0 141-20t128-57 106-90 81-118l115 58q-41 81-101 147t-134 112-159 71-177 25q-92 0-177-25t-159-71-134-112-101-147l115-58q33 65 80 118t107 90 127 57 142 20zM512 512v384h384v128H512v384H384v-384H0V896h384V512h128z" />
            </svg>
        }
    }
}
