// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};
use zu_util::prop::attr_optional;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or(false)]
    pub error: bool,

    /// Display group of elements in a compact row.
    #[prop_or(false)]
    pub row: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(FormGroup)]
pub fn form_group(props: &Props) -> Html {
    let cls = classes!(
        "ZuFormGroup-root",
        if props.row { "ZuFormGroup-row" } else { "" },
        if props.error { "ZuFormGroup-error" } else { "" },
    );

    html! {
        <div class={cls} style={attr_optional(&props.style)}>
            {for props.children.iter()}
        </div>
    }
}
