// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HighlightAltSharp)]
pub fn highlight_alt_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HighlightAltSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,5h-2V3h2V5z M19,3v2h2V3H19z M19,9h2V7h-2V9z M19,13h2v-2h-2V13z M11,21h2v-2h-2V21z M7,5h2V3H7V5z M3,5h2V3H3V5z M3,17h2v-2H3V17z M3,21h2v-2H3V21z M11,5h2V3h-2V5z M3,9h2V7H3V9z M7,21h2v-2H7V21z M3,13h2v-2H3V13z M15,15v6l2.29-2.29L19.59,21 L21,19.59l-2.29-2.29L21,15H15z"/>
        </SvgIcon>
    }
}
