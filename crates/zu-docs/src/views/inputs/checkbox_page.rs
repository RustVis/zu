// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, Callback, Html};
use zu::checkbox::Checkbox;
use zu::form_control_label::FormControlLabel;
use zu::form_group::FormGroup;
use zu::styles::color::Color;
use zu::styles::size::Size;
use zuicon_material::Bookmark as BookmarkIcon;
use zuicon_material::BookmarkBorder as BookmarkBorderIcon;
use zuicon_material::Favorite as FavoriteIcon;
use zuicon_material::FavoriteBorder as FavoriteBorderIcon;

use crate::components::demo_box::DemoBox;

#[function_component(CheckboxPage)]
pub fn checkbox_page() -> Html {
    let handle_change = Callback::from(|checked: bool| {
        log::info!("handle change, is checked: {checked}");
    });

    html! {
        <div class="container">

        <h1>{"Checkbox"}</h1>
        <p>{"Checkboxes can be used to turn an option on or off."}</p>

        <h2>{"Box checkboxes"}</h2>
        <DemoBox>
        <Checkbox default_checked={true} />
        <Checkbox />
        <Checkbox disabled={true} />
        <Checkbox disabled={true} checked={true} />
        </DemoBox>

        <h2>{"Label"}</h2>
        <p>{"You can provide a label to the Checkbox thanks to the FormControlLabel component."}</p>
        <DemoBox>
        <FormGroup>
            <FormControlLabel
                control={html!{<Checkbox default_checked={true} />}}
                label={html!{"Label"}} />
            <FormControlLabel
                required={true}
                control={html!{<Checkbox />}}
                label={html!{"Required"}} />
            <FormControlLabel
                disabled={true}
                control={html!{<Checkbox />}}
                label={html!{"Disabled"}} />
        </FormGroup>
        </DemoBox>

        <h2>{"Size"}</h2>
        <DemoBox>
        <Checkbox default_checked={true} size={Size::Small} />
        <Checkbox default_checked={true} />
        <Checkbox default_checked={true} size={Size::Large} />
        </DemoBox>

        <h2>{"Color"}</h2>
        <DemoBox>
        <Checkbox default_checked={true} />
        <Checkbox default_checked={true} color={Color::Secondary} />
        <Checkbox default_checked={true} color={Color::Success} />
        <Checkbox default_checked={true} color={Color::Default} />
        <Checkbox default_checked={true} />
        </DemoBox>

        <h2>{"Icon"}</h2>
        <DemoBox>
        <Checkbox icon={html!{<FavoriteBorderIcon />}}
            checked_icon={html!{<FavoriteIcon />}} />
        <Checkbox
            icon={html!{<BookmarkBorderIcon />}}
            checked_icon={html!{<BookmarkIcon />}}
            />
        </DemoBox>

        <h2>{"Controlled"}</h2>
        <p>{"You can control the checkbox with the checked and onChange props:"}</p>
        <DemoBox>
        <Checkbox
            checked={true}
            on_change={handle_change}
            />
        </DemoBox>

        </div>
    }
}
