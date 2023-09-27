// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CollegeHoops {}

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

impl Component for CollegeHoops {
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
                data-icon={ "CollegeHoops" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 0q133 0 255 34t230 96 194 150 150 195 97 229 34 256q0 133-34 255t-96 230-150 194-195 150-229 97-256 34q-133 0-255-34t-230-96-194-150-150-195-97-229T0 960q0-133 34-255t96-230 150-194 195-150 229-97T960 0zm451 1024q10 128 59 246t135 215q81-98 127-216t58-245h-379zm379-128q-11-127-57-245t-128-216q-85 97-134 215t-60 246h379zm-274-554q-103-93-228-147t-264-65v766h258q11-155 70-297t164-257zM896 130q-138 11-263 65T404 342q104 115 163 257t71 297h258V130zM315 1485q85-97 134-215t60-246H130q11 127 57 245t128 216zm194-589q-10-128-59-246T315 435q-81 98-127 216t-58 245h379zm-105 682q103 93 228 147t264 65v-766H638q-11 155-70 297t-164 257zm620 212q138-11 263-65t229-147q-104-115-163-257t-71-297h-258v766z" />
            </svg>
        }
    }
}
