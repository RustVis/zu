// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub enable_color_scheme: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(ScopedCssBaseline)]
pub fn scoped_css_baseline(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
