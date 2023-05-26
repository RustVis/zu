// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};

use crate::svg_icon::SvgIcon;

#[function_component(Add)]
pub fn add() -> Html {
    html! {
        <SvgIcon icon="Add">
            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
        </SvgIcon>
    }
}
