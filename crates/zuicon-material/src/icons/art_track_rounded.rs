// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ArtTrackRounded)]
pub fn art_track_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ArtTrackRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21 13h-6c-.55 0-1-.45-1-1s.45-1 1-1h6c.55 0 1 .45 1 1s-.45 1-1 1zm0-6h-6c-.55 0-1 .45-1 1s.45 1 1 1h6c.55 0 1-.45 1-1s-.45-1-1-1zm-6 10h6c.55 0 1-.45 1-1s-.45-1-1-1h-6c-.55 0-1 .45-1 1s.45 1 1 1zm-3-8v6c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V9c0-1.1.9-2 2-2h6c1.1 0 2 .9 2 2zm-2.1 5.2l-1.26-1.68c-.2-.26-.59-.27-.8-.01L6.5 14.26l-.85-1.03c-.2-.25-.58-.24-.78.01l-.74.95c-.26.33-.02.81.39.81H9.5c.41 0 .65-.47.4-.8z"/>
        </SvgIcon>
    }
}
