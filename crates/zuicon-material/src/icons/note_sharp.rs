// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoteSharp)]
pub fn note_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoteSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M22 10l-6-6H2v16h20V10zm-7-4.5l5.5 5.5H15V5.5z"/>
        </SvgIcon>
    }
}
