// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MyMoviesTv {}

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

impl Component for MyMoviesTv {
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
                data-icon={ "MyMoviesTV" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 896v832q0 40-15 75t-41 61-61 41-75 15H320q-40 0-75-15t-61-41-41-61-15-75v-507q0-37 1-67t2-59 1-60-4-67q-2-21-6-38t-8-34-10-32-11-38L22 541l1738-434 124 497L713 896h1207zM681 508l-321 80 352 176 321-80-352-176zm543 129l322-81-352-175-322 80 352 176zm-1046 4l61 241 282-70-343-171zm1489-379l-282 71 342 171-60-242zm125 762H256v704q0 26 19 45t45 19h1408q26 0 45-19t19-45v-704z" />
            </svg>
        }
    }
}
