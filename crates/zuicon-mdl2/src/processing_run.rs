// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ProcessingRun {}

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

impl Component for ProcessingRun {
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
                data-icon={ "ProcessingRun" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1930 630q0 22-2 43t-8 43l123 51-49 118-124-51q-46 74-120 120l51 125-118 49-52-124q-21 5-42 7t-43 3q-22 0-43-2t-43-8l-23 56-111-67 16-39q-74-46-120-120l-125 51-49-118 124-51q-5-21-7-42t-3-44q0-22 2-43t8-42l-124-52 49-118 125 52q23-37 53-67t67-54l-51-124 118-49 51 123q21-5 42-7t44-3q22 0 43 2t42 8l52-123 118 49-51 124q74 46 120 120l124-51 49 118-123 52q5 21 7 42t3 43zm-384 256q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20zm-577 220l139-58 44 106v15l-133 55q7 27 11 54t4 56q0 28-4 55t-11 55l133 55v15l-44 106-139-58q-29 48-68 87t-87 69l58 139-119 49-57-139q-27 7-54 11t-56 4q-28 0-55-4t-55-11l-58 139-118-49 58-140q-97-58-155-155l-140 58-48-118 138-58q-7-27-11-54t-4-56q0-28 4-55t11-55l-138-57 48-119 140 58q58-97 155-155l-58-139 118-49 58 138q27-7 54-11t56-4q28 0 55 4t55 11l57-138 119 49-58 139q97 58 155 155zm-383 548q66 0 124-25t101-68 69-102 26-125q0-66-25-124t-69-101-102-69-124-26q-66 0-124 25t-102 69-69 102-25 124q0 66 25 124t68 102 102 69 125 25zm694 394v-896l747 448-747 448zm128-670v444l370-222-370-222z" />
            </svg>
        }
    }
}
