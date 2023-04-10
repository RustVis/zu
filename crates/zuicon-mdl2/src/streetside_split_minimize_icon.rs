// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct StreetsideSplitMinimizeIcon {}

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

impl Component for StreetsideSplitMinimizeIcon {
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
                data-icon={ "StreetsideSplitMinimizeIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1363 504l90 91-429 429-429-429 90-91 276 275V256h126v523l276-275zM1024 0q141 0 271 36t245 104 207 160 161 208 103 244 37 272q0 141-36 271t-104 245-160 207-208 161-244 103-272 37q-141 0-271-36t-245-104-207-160-161-208-103-244-37-272q0-141 36-271t104-245 160-207 208-161T752 37t272-37zM165 1280q10 33 22 64t27 63h972l-23-127H165zm649 256l-42 127h462l-24-127H814zm-135 0H289q59 84 134 152t166 119l90-271zm345 384q129 0 251-36l-17-92H729l-23 70q77 29 157 43t161 15zm373-81q109-50 201-127t161-176h-418l56 303zm437-432q14-31 26-62t23-65h-590l24 127h517zm77-256q9-63 9-127 0-123-32-237t-90-214-141-182-181-140-214-91-238-32q-123 0-237 32t-214 90-182 141-140 181-91 214-32 238q0 64 9 127h1774z" />
            </svg>
        }
    }
}
