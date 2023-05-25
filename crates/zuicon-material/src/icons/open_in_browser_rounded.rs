// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OpenInBrowserRounded)]
pub fn open_in_browser_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="OpenInBrowserRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M19 4H5c-1.11 0-2 .9-2 2v12c0 1.1.9 2 2 2h3c.55 0 1-.45 1-1s-.45-1-1-1H5V8h14v10h-3c-.55 0-1 .45-1 1s.45 1 1 1h3c1.1 0 2-.9 2-2V6c0-1.1-.89-2-2-2zm-7.35 6.35l-2.79 2.79c-.32.32-.1.86.35.86H11v5c0 .55.45 1 1 1s1-.45 1-1v-5h1.79c.45 0 .67-.54.35-.85l-2.79-2.79c-.19-.2-.51-.2-.7-.01z"/>
        </SvgIcon>
    }
}
