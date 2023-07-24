// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use stylist::Style;
use yew::{classes, function_component, html, Callback, Html, MouseEvent};
use zu::button::Button;
use zu::icon_button::IconButton;
use zu::loading_button::{LoadingButton, Position};
use zu::r#box::Box;
use zu::stack::Stack;
use zu::styles::button_variant::ButtonVariant as Variant;
use zu::styles::color::Color;
use zu::styles::flex_direction::FlexDirection;
use zu::styles::size::Size;
use zu::styles::spacing::Spacing;
use zu::svg_icon::FontSize;
use zuicon_material::{AddShoppingCart, Alarm, Delete, Fingerprint, Save, Send};

use crate::components::demo_box::DemoBox;

fn create_basic_button_view() -> Html {
    html! {
        <>
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
            <Button variant={Variant::Contained} disabled=true>{"Disabled"}</Button>
            <Button variant={Variant::Contained} href="#contained-buttons">
                {"Link"}
            </Button>
        </DemoBox>

        <p>{"You can remove the elevation with the disableElevation prop."}</p>
        <DemoBox>
            <Button variant={Variant::Contained} disable_elevation=true>
                {"Disable elevation"}
            </Button>
        </DemoBox>

        <h3>{"Outlined button"}</h3>
        <p>{"Outlined buttons are medium-emphasis buttons."}</p>
        <DemoBox>
            <Button variant={Variant::Outlined}>{"Primary"}</Button>
            <Button variant={Variant::Outlined} disabled=true>{"Disabled"}</Button>
            <Button variant={Variant::Outlined} href="#outlined-buttons">{"Link"}</Button>
        </DemoBox>
        </>
    }
}

fn create_handling_clicks_view() -> Html {
    html! {
        <>
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
        </>
    }
}

fn create_colors_view() -> Html {
    html! {
        <>
        <h2>{"Color"}</h2>
        <DemoBox>
            <Button variant={Variant::Contained} color={Color::Primary}>{"Primary"}</Button>
            <Button variant={Variant::Contained} color={Color::Secondary}>{"Secondary"}</Button>
            <Button variant={Variant::Contained} color={Color::Success}>{"Success"}</Button>
            <Button variant={Variant::Contained} color={Color::Info}>{"Info"}</Button>
            <Button variant={Variant::Contained} color={Color::Warning}>{"Warning"}</Button>
            <Button variant={Variant::Contained} color={Color::Error}>{"Error"}</Button>
        </DemoBox>
        <p>{"In addition to using the default button colors, you can add custom ones, or disable any you don't need."}</p>
        </>
    }
}

fn create_sizes_view() -> Html {
    let margin_style = Style::new(
        r#"
        button {
            margin: 8px;
        }
        "#,
    )
    .expect("Failed to create margin-style");

    html! {
        <>
        <h2>{"Sizes"}</h2>
        <p>{"For larger or smaller buttons, use the size prop."}</p>
        <DemoBox>
            <Box classes={classes!(margin_style.get_class_name().to_owned())}>
            <div>
                <Button size={Size::Small}>{"Small"}</Button>
                <Button size={Size::Medium}>{"Medium"}</Button>
                <Button size={Size::Large}>{"Large"}</Button>
            </div>
            <div>
                <Button variant={Variant::Outlined} size={Size::Small}>{"Small"}</Button>
                <Button variant={Variant::Outlined} size={Size::Medium}>{"Medium"}</Button>
                <Button variant={Variant::Outlined} size={Size::Large}>{"Large"}</Button>
            </div>
            <div>
                <Button variant={Variant::Contained} size={Size::Small}>{"Small"}</Button>
                <Button variant={Variant::Contained} size={Size::Medium}>{"Medium"}</Button>
                <Button variant={Variant::Contained} size={Size::Large}>{"Large"}</Button>
            </div>
        </Box>
        </DemoBox>
        </>
    }
}

fn create_icons_and_labels_view() -> Html {
    html! {
        <>
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
        </>
    }
}

fn create_icon_button_sizes_view() -> Html {
    html! {
        <>
        <h3>{"Sizes"}</h3>
        <p>{"For larger or smaller icon buttons, use the size prop."}</p>
        <DemoBox>
            <IconButton aria_label="delete" size={Size::Small}>
                <Delete font_size={FontSize::Inherit} />
            </IconButton>
            <IconButton aria_label="delete" size={Size::Small}>
                <Delete font_size={FontSize::Small} />
            </IconButton>
            <IconButton aria_label="delete" size={Size::Large}>
                <Delete />
            </IconButton>
            <IconButton aria_label="delete" size={Size::Large}>
                <Delete font_size={FontSize::Inherit} />
            </IconButton>
        </DemoBox>
        </>
    }
}

fn create_icon_button_colors_view() -> Html {
    html! {
        <>
        <h3>{"Colors"}</h3>
        <p>{"Use color prop to apply theme color palette to component."}</p>
        <DemoBox>
            <IconButton aria_label="fingerprint" color={Color::Primary}>
                <Fingerprint />
            </IconButton>
            <IconButton aria_label="fingerprint" color={Color::Secondary}>
                <Fingerprint />
            </IconButton>
            <IconButton aria_label="fingerprint" color={Color::Info}>
                <Fingerprint />
            </IconButton>
            <IconButton aria_label="fingerprint" color={Color::Warning}>
                <Fingerprint />
            </IconButton>
            <IconButton aria_label="fingerprint" color={Color::Error}>
                <Fingerprint />
            </IconButton>
            <IconButton aria_label="fingerprint" color={Color::Success}>
                <Fingerprint />
            </IconButton>
        </DemoBox>
        </>
    }
}

fn create_icon_button_view() -> Html {
    html! {
        <>
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

        {create_icon_button_sizes_view()}
        {create_icon_button_colors_view()}

        </>
    }
}

fn create_loading_button_toggle_view() -> Html {
    let margin_style = Style::new(
        r#"
        button {
            margin: 8px;
        }
        "#,
    )
    .expect("Failed to create margin-style");

    let handle_click = Callback::from(|event: MouseEvent| {
        event.prevent_default();
        log::info!("on click");
    });

    // TODO(Shaohua): Replace with state.
    let loading = true;

    html! {
        <>
        <p>{"Toggle the loading switch to see the transition between the different states."}</p>
        <DemoBox>
        <Box classes={classes!(margin_style.get_class_name().to_owned())}>
            <LoadingButton
                size={Size::Small}
                on_click={handle_click.clone()}
                loading={loading}
                variant={Variant::Outlined}
                disabled=true
                >
                <span>{"disabled"}</span>
            </LoadingButton>
            <LoadingButton
                size={Size::Small}
                on_click={handle_click.clone()}
                loading={loading}
                loading_indicator={html!{"Loading…"}}
                variant={Variant::Outlined}
                >
                <span>{"Fetch data"}</span>
            </LoadingButton>
            <LoadingButton
                size={Size::Small}
                color={Color::Secondary}
                on_click={handle_click.clone()}
                loading={loading}
                loading_position={Position::Start}
                start_icon={html!{<Save />}}
                variant={Variant::Contained}
                >
                <span>{"Save"}</span>
            </LoadingButton>
            <LoadingButton
                size={Size::Small}
                on_click={handle_click.clone()}
                end_icon={html!{<Send />}}
                loading={loading}
                loading_position={Position::End}
                variant={Variant::Contained}
                >
                <span>{"Send"}</span>
            </LoadingButton>
        </Box>

        <Box classes={classes!(margin_style.get_class_name().to_owned())}>
            <LoadingButton
                on_click={handle_click.clone()}
                loading={loading}
                variant={Variant::Outlined}
                disabled=true
                >
                <span>{"disabled"}</span>
            </LoadingButton>
            <LoadingButton
                on_click={handle_click.clone()}
                loading={loading}
                loading_indicator={html!{"Loading…"}}
                variant={Variant::Outlined}
                >
                <span>{"Fetch data"}</span>
            </LoadingButton>
            <LoadingButton
                color={Color::Secondary}
                on_click={handle_click.clone()}
                loading={loading}
                loading_position={Position::Start}
                start_icon={html!{<Save />}}
                variant={Variant::Contained}
                >
                <span>{"Save"}</span>
            </LoadingButton>
            <LoadingButton
                on_click={handle_click}
                end_icon={html!{<Send />}}
                loading={loading}
                loading_position={Position::End}
                variant={Variant::Contained}
                >
                <span>{"Send"}</span>
            </LoadingButton>
        </Box>
        </DemoBox>
        </>
    }
}

fn create_loading_button_view() -> Html {
    // TODO(Shaohua): Replace Spacing::Large with Spacing::Num(2)
    html! {
        <>
        <h2>{"Loading button"}</h2>
        <p>{"Loading buttons can show loading state and disable interactions."}</p>
        <DemoBox>
        <Stack direction={FlexDirection::Row} spacing={Spacing::Large}>
            <LoadingButton loading=true variant={Variant::Outlined}>
                {"Submit"}
            </LoadingButton>
            <LoadingButton loading=true loading_indicator={html!{"Loading…"}}
                variant={Variant::Outlined}>
                {"Fetch data"}
            </LoadingButton>
            <LoadingButton loading=true loading_position={Position::Start}
                start_icon={html!{<Save />}} variant={Variant::Outlined}>
                {"Save"}
            </LoadingButton>

            <LoadingButton loading=true loading_position={Position::End}
                end_icon={html!{<Send />}} variant={Variant::Outlined}>
                {"Send"}
            </LoadingButton>
        </Stack>
        </DemoBox>

        {create_loading_button_toggle_view()}

        </>
    }
}

#[function_component(ButtonPage)]
pub fn button_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Button"}</h1>
        <p>{"Buttons communicate actions that users can take."}</p>

        {create_basic_button_view()}
        {create_handling_clicks_view()}
        {create_colors_view()}
        {create_sizes_view()}
        {create_icons_and_labels_view()}
        {create_icon_button_view()}
        {create_loading_button_view()}

        <h2>{"Customization"}</h2>
        <p>{"TODO"}</p>

        <h2>{"Complex button"}</h2>
        <p>{"TODO"}</p>

        </div>
    }
}
