// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, html_nested, ChildrenWithProps, Html};
use zu::button::Button;
use zu::button_group::ButtonGroup;
use zu::r#box::Box;
use zu::styles::button_variant::ButtonVariant;
use zu::styles::color::Color;
use zu::styles::orientation::Orientation;
use zu::styles::size::Size;
use zuicon_material::ArrowDropDown;

use crate::components::demo_box::DemoBox;

fn create_basic_group_view() -> Html {
    html! {
        <>
        <h2>{"Basic button group"}</h2>
        <p>{"The buttons can be grouped by wrapping them with the ButtonGroup component. \
        They need to be immediate children."}</p>

        <DemoBox>
        <ButtonGroup variant={ButtonVariant::Contained} aria_label="outlined primary button group">
            <Button>{"One"}</Button>
            <Button>{"Two"}</Button>
            <Button>{"Three"}</Button>
        </ButtonGroup>
        </DemoBox>
        </>
    }
}

fn create_button_variants_view() -> Html {
    html! {
        <>
        <h2>{"Button variants"}</h2>
        <p>{"All the standard button variants are supported."}</p>
        <DemoBox>
        <ButtonGroup variant={ButtonVariant::Outlined} aria_label="outlined button group">
            <Button>{"One"}</Button>
            <Button>{"Two"}</Button>
            <Button>{"Three"}</Button>
        </ButtonGroup>
        <ButtonGroup variant={ButtonVariant::Text} aria_label="text button group">
            <Button>{"One"}</Button>
            <Button>{"Two"}</Button>
            <Button>{"Three"}</Button>
        </ButtonGroup>
        </DemoBox>
        </>
    }
}

fn create_buttons() -> ChildrenWithProps<Button> {
    ChildrenWithProps::new(vec![
        html_nested! {<Button key="one">{"One"}</Button>},
        html_nested! {<Button key="two">{"Two"}</Button>},
        html_nested! {<Button key="three">{"Three"}</Button>},
    ])
}

fn create_size_colors_view() -> Html {
    html! {
        <>
        <h2>{"Size and colors"}</h2>
        <p>{"The size and color props can be used to control the appearance of the button group."}</p>
        <DemoBox>
        <ButtonGroup size={Size::Small} aria_label="small button group">
            {create_buttons()}
        </ButtonGroup>
        <ButtonGroup color={Color::Secondary} aria_label="medium secondary button group">
            {create_buttons()}
        </ButtonGroup>
        <ButtonGroup size={Size::Large} aria_label="large button group">
            {create_buttons()}
        </ButtonGroup>
        </DemoBox>
        </>
    }
}

fn create_vertical_group_view() -> Html {
    html! {
        <>
        <h2>{"Vertical group"}</h2>
        <p>{"The button group can be displayed vertically using the orientation prop."}</p>

        <DemoBox>
        <Box>
            <ButtonGroup
                orientation={Orientation::Vertical}
                aria_label="vertical outlined button group">
                {create_buttons()}
            </ButtonGroup>
            <ButtonGroup
                orientation={Orientation::Vertical}
                aria_label="vertical contained button group"
                variant={ButtonVariant::Contained}>
                {create_buttons()}
            </ButtonGroup>
            <ButtonGroup
                orientation={Orientation::Vertical}
                aria_label="vertical contained button group"
                variant={ButtonVariant::Text}>
                {create_buttons()}
            </ButtonGroup>
        </Box>
        </DemoBox>
        </>
    }
}

fn create_split_button_view() -> Html {
    // TODO(Shaohua): Create popover
    html! {
        <>
        <h2>{"Split button"}</h2>
        <p>{"ButtonGroup can also be used to create a split button. \
        The dropdown can change the button action (as in this example) or \
        be used to immediately trigger a related action."}</p>

        <DemoBox>
        <ButtonGroup
            variant={ButtonVariant::Contained}
            aria_label="split button">
            <Button >{"Index"}</Button>
            <Button
                size={Size::Small}
                aria_label="select merge strategy"
                >
                <ArrowDropDown />
            </Button>
        </ButtonGroup>
        </DemoBox>
        </>
    }
}

fn create_disabled_elevation_view() -> Html {
    html! {
        <>
        <h2>{"Disabled elevation"}</h2>
        <p>{"You can remove the elevation with the disableElevation prop."}</p>
        <DemoBox>
        <ButtonGroup
            disable_elevation=true
            variant={ButtonVariant::Contained}
            aria_label="Disabled elevation buttons">
            <Button>{"One"}</Button>
            <Button>{"Two"}</Button>
        </ButtonGroup>
        </DemoBox>
        </>
    }
}

#[function_component(ButtonGroupPage)]
pub fn button_group_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Button group"}</h1>

        {create_basic_group_view()}
        {create_button_variants_view()}
        {create_size_colors_view()}
        {create_vertical_group_view()}
        {create_split_button_view()}
        {create_disabled_elevation_view()}

        </div>
    }
}
