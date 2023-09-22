// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, html_nested, ChildrenWithProps, Html};
use zu::stack::Stack;
use zu::styles::color::Color;
use zu::styles::flex_direction::FlexDirection;
use zu::styles::orientation::Orientation;
use zu::styles::size::Size;
use zu::styles::spacing::Spacing;
use zu::toggle_button::ToggleButton;
use zu::toggle_button_group::ToggleButtonGroup;
use zuicon_material::{
    ArrowDropDown, FormatAlignCenter, FormatAlignJustify, FormatAlignLeft, FormatAlignRight,
    FormatBold, FormatColorFill, FormatItalic, FormatUnderlined, Laptop, PhoneAndroid, Tv,
    ViewList, ViewModule, ViewQuilt,
};

use crate::components::demo_box::DemoBox;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}

fn create_exclusive_section_view() -> Html {
    let alignment = Alignment::Left;

    html! {
        <>
        <h2>{"Exclusive selection"}</h2>
        <p>{"With exclusive selection, selecting one option deselects any other."}</p>
        <p>{"In this example, text justification toggle buttons present options for left, center, right,\
         and fully justified text (disabled), with only one item available for selection at a time."}</p>

        <DemoBox>
        <ToggleButtonGroup<Alignment>
            value={alignment}
            exclusive={true}
            aria_label="text alignment">
            <ToggleButton<Alignment> value={Alignment::Left} aria_label="left aligned">
                <FormatAlignLeft />
            </ToggleButton<Alignment>>
            <ToggleButton<Alignment> value={Alignment::Center} aria_label="centered">
                <FormatAlignCenter />
            </ToggleButton<Alignment>>
            <ToggleButton<Alignment> value={Alignment::Right} aria_label="right aligned">
                <FormatAlignRight />
            </ToggleButton<Alignment>>
            <ToggleButton<Alignment> value={Alignment::Justify} aria_label="justified" disabled=true>
                <FormatAlignJustify />
            </ToggleButton<Alignment>>
        </ToggleButtonGroup<Alignment>>
        </DemoBox>
        </>
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum FontFormat {
    Bold,
    Italic,
    Underlined,
    Color,
}

fn create_multiple_selection_view() -> Html {
    let formats = &[FontFormat::Bold, FontFormat::Italic];

    html! {
        <>
        <h2>{"Multiple selection"}</h2>
        <p>{"Multiple selection allows for logically-grouped options, like bold, italic, \
        and underline, to have multiple options selected."}</p>

        <DemoBox>
        <ToggleButtonGroup<FontFormat>
            values={formats.to_vec()}
            aria_label="text formatting"
            >
            <ToggleButton<FontFormat> value={FontFormat::Bold} aria_label="bold">
                <FormatBold />
            </ToggleButton<FontFormat>>
            <ToggleButton<FontFormat> value={FontFormat::Italic} aria_label="italic">
                <FormatItalic />
            </ToggleButton<FontFormat>>
            <ToggleButton<FontFormat> value={FontFormat::Underlined} aria_label="underlined">
                <FormatUnderlined />
            </ToggleButton<FontFormat>>
            <ToggleButton<FontFormat> value={FontFormat::Color} aria_label="color" disabled=true>
                <FormatColorFill />
                <ArrowDropDown />
            </ToggleButton<FontFormat>>
        </ToggleButtonGroup<FontFormat>>
        </DemoBox>
        </>
    }
}

fn create_alignment_buttons() -> ChildrenWithProps<ToggleButton<Alignment>> {
    ChildrenWithProps::new(vec![
        html_nested! {
           <ToggleButton<Alignment> value={Alignment::Left} aria_label="left aligned">
               <FormatAlignLeft />
           </ToggleButton<Alignment>>
        },
        html_nested! {
            <ToggleButton<Alignment> value={Alignment::Center} aria_label="centered">
                <FormatAlignCenter />
            </ToggleButton<Alignment>>
        },
        html_nested! {
            <ToggleButton<Alignment> value={Alignment::Right} aria_label="right aligned">
                <FormatAlignRight />
            </ToggleButton<Alignment>>
        },
        html_nested! {
            <ToggleButton<Alignment> value={Alignment::Justify} aria_label="justified" disabled=true>
                <FormatAlignJustify />
            </ToggleButton<Alignment>>
        },
    ])
}

fn create_size_view() -> Html {
    let value = Alignment::Left;

    html! {
        <>
        <h2>{"Size"}</h2>
        <p>{"For larger or smaller buttons, use the size prop."}</p>

        <DemoBox>
        <Stack spacing={Spacing::Small}>
            <ToggleButtonGroup<Alignment>
                size={Size::Small}
                value={value}
                aria_label="Small sizes">
                {create_alignment_buttons()}
            </ToggleButtonGroup<Alignment>>
            <ToggleButtonGroup<Alignment>
                value={value}
                aria_label="Medium sizes">
                {create_alignment_buttons()}
            </ToggleButtonGroup<Alignment>>
            <ToggleButtonGroup<Alignment>
                size={Size::Large}
                value={value}
                aria_label="Large sizes">
                {create_alignment_buttons()}
            </ToggleButtonGroup<Alignment>>
        </Stack>
        </DemoBox>
        </>
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Platform {
    Web,
    Android,
    Ios,
}

fn create_color_view() -> Html {
    let platform = Platform::Web;
    html! {
        <>
        <h2>{"Color"}</h2>
        <DemoBox>
        <ToggleButtonGroup<Platform>
            color={Color::Primary}
            value={platform}
            exclusive=true
            aria_label="Platform"
            >
            <ToggleButton<Platform> value={Platform::Web}>{"Web"}</ToggleButton<Platform>>
            <ToggleButton<Platform> value={Platform::Android}>{"Android"}</ToggleButton<Platform>>
            <ToggleButton<Platform> value={Platform::Ios}>{"iOS"}</ToggleButton<Platform>>
        </ToggleButtonGroup<Platform>>
        </DemoBox>
        </>
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ViewMode {
    List,
    Module,
    Quilt,
}

fn create_vertical_buttons_view() -> Html {
    let view = ViewMode::List;

    html! {
        <>
        <h2>{"Vertical buttons"}</h2>
        <p>{"The buttons can be stacked vertically with the orientation prop set to `vertical`."}</p>

        <DemoBox>
        <ToggleButtonGroup<ViewMode>
            orientation={Orientation::Vertical}
            value={view}
            exclusive=true
            >
            <ToggleButton<ViewMode> value={ViewMode::List} aria_label="list">
                <ViewList />
            </ToggleButton<ViewMode>>
            <ToggleButton<ViewMode> value={ViewMode::Module} aria_label="module">
                <ViewModule />
            </ToggleButton<ViewMode>>
            <ToggleButton<ViewMode> value={ViewMode::Quilt} aria_label="quilt">
                <ViewQuilt />
            </ToggleButton<ViewMode>>
        </ToggleButtonGroup<ViewMode>>
        </DemoBox>
        </>
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Device {
    Laptop,
    Tv,
    Phone,
}

fn create_enforce_value_view() -> Html {
    let alignment = Alignment::Left;
    let device = Device::Laptop;

    html! {
        <>
        <h2>{"Enforce value set"}</h2>
        <p>{"If you want to enforce that at least one button must be active, you can adapt your handleChange function."}</p>

        <DemoBox>
          <Stack direction={FlexDirection::Row} spacing={Spacing::Small}>
            <ToggleButtonGroup<Alignment>
                value={alignment}
                exclusive=true
                aria_label="text alignment"
                >
                <ToggleButton<Alignment> value={Alignment::Left} aria_label="left aligned">
                    <FormatAlignLeft />
                </ToggleButton<Alignment>>
                <ToggleButton<Alignment> value={Alignment::Center} aria_label="centered">
                    <FormatAlignCenter />
                </ToggleButton<Alignment>>
                <ToggleButton<Alignment> value={Alignment::Right} aria_label="right aligned">
                    <FormatAlignRight />
                </ToggleButton<Alignment>>
            </ToggleButtonGroup<Alignment>>
            <ToggleButtonGroup<Device>
                value={device}
                aria_label="device"
                >
                <ToggleButton<Device> value={Device::Laptop} aria_label="laptop">
                    <Laptop />
                </ToggleButton<Device>>
                <ToggleButton<Device> value={Device::Tv} aria_label="tv">
                    <Tv />
                </ToggleButton<Device>>
                <ToggleButton<Device> value={Device::Phone} aria_label="phone">
                    <PhoneAndroid />
                </ToggleButton<Device>>
            </ToggleButtonGroup<Device>>
        </Stack>
        </DemoBox>
        </>
    }
}

#[function_component(ToggleButtonPage)]
pub fn toggle_button_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Toggle Button"}</h1>
        <p>{"To emphasize groups of related Toggle buttons, a group should share a common container. \
        The ToggleButtonGroup controls the selected state of its child buttons \
        when given its own value prop."}</p>

        {create_exclusive_section_view()}
        {create_multiple_selection_view()}
        {create_size_view()}
        {create_color_view()}
        {create_vertical_buttons_view()}
        {create_enforce_value_view()}

        </div>
    }
}
