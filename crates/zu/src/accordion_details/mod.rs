// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(AccordionDetails)]
pub fn accordion_details(props: &Props) -> Html {
    let root_cls = "ZuAccordionDetails-root";

    let style = if props.style.is_empty() {
        None
    } else {
        Some(props.style.to_string())
    };

    html! {
        <div class={root_cls} style={style}>
            {for props.children.iter()}
        </div>
    }
}
