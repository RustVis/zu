// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SpellcheckRounded)]
pub fn spellcheck_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SpellcheckRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M13.12 16c.69 0 1.15-.69.9-1.32L9.77 3.87C9.56 3.34 9.06 3 8.5 3s-1.06.34-1.27.87L2.98 14.68c-.25.63.22 1.32.9 1.32.4 0 .76-.25.91-.63L5.67 13h5.64l.9 2.38c.15.37.51.62.91.62zm-6.69-5L8.5 5.48 10.57 11H6.43zm14.46 1.29l-7.39 7.39-2.97-2.97c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41l3.68 3.68c.39.39 1.02.39 1.41 0l8.08-8.09c.39-.39.39-1.02 0-1.41-.38-.39-1.02-.39-1.4-.01z"/>
        </SvgIcon>
    }
}
