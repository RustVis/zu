// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct RainShowersDay {}

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

impl Component for RainShowersDay {
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
                data-icon={ "RainShowersDay" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1042 1827q14 30 14 61 0 33-12 62t-35 51-51 34-62 13q-33 0-62-12t-51-35-34-51-13-62q0-31 14-61l146-291 146 291zm384-256q14 30 14 61 0 33-12 62t-35 51-51 34-62 13q-33 0-62-12t-51-35-34-51-13-62q0-31 14-61l146-291 146 291zm335-535q63 17 115 52t91 85 60 109 21 126q0 80-30 149t-82 122-123 83-149 30h-108q17-29 27-61t14-67h67q53 0 99-20t82-55 55-81 20-100q0-55-20-102t-57-81-84-53-102-20q-12-82-51-152t-97-122-134-81-159-29q-77 0-146 25t-127 69-98 106-61 135q-44-38-97-58t-111-21q-66 0-124 25t-102 68-69 102-25 125q0 66 25 124t68 102 102 69 125 25h96l-64 128h-32q-93 0-174-35t-142-96-96-142-36-175q0-67 19-129t54-116 84-96 111-69q-12-49-12-102 0-93 35-174t96-142 142-96 175-36q65 0 125 18t113 51 95 81 70 106q90 3 172 33t152 83 122 123 80 157zM723 921q61-103 158-173t215-95q-45-66-114-103t-150-38q-66 0-124 25t-102 69-69 102-25 124q0 32 8 67 28-3 56-3 74 0 147 25zm-467-25H0V768h256v128zm123-426L198 289l91-91 181 181-91 91zm517-214H768V0h128v256zm389 214l-91-91 181-181 91 91-181 181z" />
            </svg>
        }
    }
}
