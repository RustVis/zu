// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SpeakerNotesOffSharp)]
pub fn speaker_notes_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SpeakerNotesOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M1.27 1.73L0 3l2.01 2.01L2 22l4-4h9l5.73 5.73L22 22.46 1.27 1.73zM8 14H6v-2h2v2zm-2-3V9l2 2H6zm16-9H4.08L10 7.92V6h8v2h-7.92l1 1H18v2h-4.92l6.99 6.99H22V2z"/>
        </SvgIcon>
    }
}
