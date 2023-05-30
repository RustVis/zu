// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html, MouseEvent};
use zu::button::{Button, Variant};
use zu::icon_button::IconButton;
use zu::r#box::Box;
use zu::styles::color::Color;
use zu::styles::size::Size;
use zuicon_material::{AddShoppingCart, Alarm, Delete, Send};

use crate::components::demo_box::DemoBox;

#[function_component(ButtonPage)]
pub fn button_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Button"}</h1>
        <p>{"Buttons communicate actions that users can take."}</p>

        <h2>{"Basic button"}</h2>
        <p>{"The Button comes with three variants: text (default), contained, and outlined."}</p>
        <DemoBox>
        <Button variant={Variant::Text}>{"Text"}</Button>
        <Button variant={Variant::Contained}>{"Contained"}</Button>
        <Button variant={Variant::Outlined}>{"Outlined"}</Button>
        </DemoBox>

        <h3>{"Text button"}</h3>
        <p>{"Text buttons are typically used for less-pronounced actions, including those located: in dialogs, in cards."}</p>
        <DemoBox>
        <Button>{"Primary"}</Button>
        <Button disabled={true}>{"Disabled"}</Button>
        <Button href="#text-buttons">{"Link"}</Button>
        </DemoBox>

        <h3>{"Contained button"}</h3>
        <p>{"Contained buttons are high-emphasis, distinguished by their use of elevation and fill. "}</p>
        <DemoBox>
        <Button variant={Variant::Contained}>{"Contained"}</Button>
        <Button variant={Variant::Contained} disabled={true}>
            {"Disabled"}
        </Button>
        <Button variant={Variant::Contained} href="#contained-buttons">
            {"Link"}
        </Button>
        </DemoBox>

        <p>{"You can remove the elevation with the disableElevation prop."}</p>
        <DemoBox>
        <Button variant={Variant::Contained} disable_elevation={true}>
            {"Disable elevation"}
        </Button>
        </DemoBox>

        <h3>{"Outlined button"}</h3>
        <p>{"Outlined buttons are medium-emphasis buttons."}</p>
        <DemoBox>
        <Button variant={Variant::Outlined}>{"Primary"}</Button>
        <Button variant={Variant::Outlined} disabled={true}>
            {"Disabled"}
        </Button>
        <Button variant={Variant::Outlined} href="#outlined-buttons">
            {"Link"}
        </Button>
        </DemoBox>

        <h2>{"Handling clicks"}</h2>
        <p>{"All components accept an onClick handler that is applied to the root DOM element."}</p>
        <DemoBox>
        <Button
            on_click={|event: MouseEvent| {
                event.prevent_default();
                log::info!("clicked!");
            }}
            >
            {"Click me"}
        </Button>
        </DemoBox>

        <h2>{"Color"}</h2>
        <DemoBox>
        <Button color={Color::Secondary}>{"Secondary"}</Button>
        <Button variant={Variant::Contained} color={Color::Success}>
            {"Success"}
        </Button>
        <Button variant={Variant::Outlined} color={Color::Error}>
            {"Error"}
        </Button>
        </DemoBox>
        <p>{"In addition to using the default button colors, you can add custom ones, or disable any you don't need."}</p>

        <h2>{"Sizes"}</h2>
        <p>{"For larger or smaller buttons, use the size prop."}</p>
        <DemoBox>
            <Box>
            <div>
                <Button size={Size::Small}>{"Small"}</Button>
                <Button size={Size::Medium}>{"Medium"}</Button>
                <Button size={Size::Large}>{"Large"}</Button>
            </div>
            <div>
                <Button variant={Variant::Outlined} size={Size::Small}>
                    {"Small"}
                </Button>
                <Button variant={Variant::Outlined} size={Size::Medium}>
                    {"Medium"}
                </Button>
                <Button variant={Variant::Outlined} size={Size::Large}>
                    {"Large"}
                </Button>
            </div>
            <div>
                <Button variant={Variant::Contained} size={Size::Small}>
                    {"Small"}
                </Button>
                <Button variant={Variant::Contained} size={Size::Medium}>
                    {"Medium"}
                </Button>
                <Button variant={Variant::Contained} size={Size::Large}>
                    {"Large"}
                </Button>
            </div>
        </Box>
        </DemoBox>

        <h2>{"Buttons with icons and label"}</h2>
        <p>{"Sometimes you might want to have icons for certain buttons to enhance \
            the UX of the application as we recognize logos more easily than plain text. "}</p>
        <DemoBox>
        <Button variant={Variant::Outlined} start_icon={html!{<Delete />}}>
            {"Delete"}
        </Button>
        <Button variant={Variant::Contained} end_icon={html!{<Send />}}>
            {"Send"}
        </Button>
        </DemoBox>

        <h2>{"Icon button"}</h2>
        <p>{"Icon buttons are commonly found in app bars and toolbars."}</p>
        <DemoBox>
        <IconButton aria_label="delete">
            <Delete />
        </IconButton>
        <IconButton aria_label="delete" disabled={true} color={Color::Primary}>
            <Delete />
        </IconButton>
        <IconButton color={Color::Secondary} aria_label="add an alarm">
            <Alarm />
        </IconButton>
        <IconButton color={Color::Primary} aria_label="add to shopping cart">
            <AddShoppingCart />
        </IconButton>
        </DemoBox>

        </div>
    }
}
