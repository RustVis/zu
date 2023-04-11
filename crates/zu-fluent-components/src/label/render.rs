// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use yew::{function_component, html, Html};

use crate::label::types::LabelProps;

#[function_component(Label)]
pub fn render(props: &LabelProps) -> Html {
    html! {
        <label>
            { props.children.clone() }
        </label>
    }
}
