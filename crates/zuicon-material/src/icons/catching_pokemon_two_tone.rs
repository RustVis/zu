// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CatchingPokemonTwoTone)]
pub fn catching_pokemon_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CatchingPokemonTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,4c4.08,0,7.45,3.05,7.94,7h-4.06C15.43,9.27,13.86,8,12,8s-3.43,1.27-3.87,3H4.06C4.55,7.05,7.92,4,12,4z" opacity=".3"/><path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M12,4c4.08,0,7.45,3.05,7.94,7h-4.06 C15.43,9.27,13.86,8,12,8s-3.43,1.27-3.87,3H4.06C4.55,7.05,7.92,4,12,4z M14,12c0,1.1-0.9,2-2,2s-2-0.9-2-2c0-1.1,0.9-2,2-2 S14,10.9,14,12z M12,20c-4.08,0-7.45-3.05-7.94-7h4.06c0.44,1.73,2.01,3,3.87,3s3.43-1.27,3.87-3h4.06C19.45,16.95,16.08,20,12,20z"/>
        </SvgIcon>
    }
}
