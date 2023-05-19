// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::virtual_dom::VNode;
use yew::{function_component, html, AttrValue, Children, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    /// The icon to display as the expand indicator.
    #[prop_or_default]
    pub expand_icon: VNode,

    // pub focus_visible_class_name: AttrValue,
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(AccordionSummary)]
pub fn accordion_summary(_props: &Props) -> Html {
    html! {
        <div>
        </div>
    }
}
