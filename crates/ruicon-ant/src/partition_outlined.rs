use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PartitionOutlined {}

#[derive(Properties, Debug, Clone, PartialEq)]
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

impl Component for PartitionOutlined {
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
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("1em") }
                height={ props.height.unwrap_or("1em") }
                focusable={ "false" }
                data-icon={ "partition" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <defs><style type="text/css"></style></defs><path d="M640.6 429.8h257.1c7.9 0 14.3-6.4 14.3-14.3V158.3c0-7.9-6.4-14.3-14.3-14.3H640.6c-7.9 0-14.3 6.4-14.3 14.3v92.9H490.6c-3.9 0-7.1 3.2-7.1 7.1v221.5h-85.7v-96.5c0-7.9-6.4-14.3-14.3-14.3H126.3c-7.9 0-14.3 6.4-14.3 14.3v257.2c0 7.9 6.4 14.3 14.3 14.3h257.1c7.9 0 14.3-6.4 14.3-14.3V544h85.7v221.5c0 3.9 3.2 7.1 7.1 7.1h135.7v92.9c0 7.9 6.4 14.3 14.3 14.3h257.1c7.9 0 14.3-6.4 14.3-14.3v-257c0-7.9-6.4-14.3-14.3-14.3h-257c-7.9 0-14.3 6.4-14.3 14.3v100h-78.6v-393h78.6v100c0 7.9 6.4 14.3 14.3 14.3z m53.5-217.9h150V362h-150V211.9zM329.9 587h-150V437h150v150z m364.2 75.1h150v150.1h-150V662.1z" p-id="12594"></path>
            </svg>
        }
    }
}
