// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, Callback, Html};
use zu::form_control::FormControl;
use zu::form_control_label::FormControlLabel;
use zu::form_group::FormGroup;
use zu::form_label::FormLabel;
use zu::styles::color::Color;
use zu::styles::position::Position;
use zu::styles::size::Size;
use zu::switch::Switch;

use crate::components::demo_box::DemoBox;

#[function_component(SwitchPage)]
pub fn switch_page() -> Html {
    let demo_label = "Switch Demo";
    // TODO(Shaohua): Add customized switch

    let handle_change = Callback::from(|checked: bool| {
        log::info!("handle_change() is checked: {checked}");
    });

    html! {
        <div class="container">
        <h1>{"Switch"}</h1>
        <p>{"Switches are the preferred way to adjust settings on mobile. \
            The option that the switch controls, as well as the state it's in, \
            should be made clear from the corresponding inline label."}</p>

        <h2>{"Basic switches"}</h2>
        <DemoBox>
        <Switch aria_label={demo_label} default_checked={true} />
        <Switch aria_label={demo_label} />
        <Switch aria_label={demo_label} disabled={true} default_checked={true} />
        <Switch aria_label={demo_label} disabled={true} />
        </DemoBox>

        <h2>{"Label"}</h2>
        <p>{"You can provide a label to the Switch thanks to the FormControlLabel component."}</p>
        <DemoBox>
        <FormGroup>
            <FormControlLabel control={html!{<Switch default_checked={true} />}}
                label={html!{"Label"}} />
            <FormControlLabel required={true}
                control={html!{<Switch />}}
                label={html!{"Required"}} />
            <FormControlLabel disabled={true}
                control={html!{<Switch />}}
                label={html!{"Disabled"}} />
        </FormGroup>
        </DemoBox>

        <h2>{"Size"}</h2>
        <p>{"Use the size prop to change the size of the switch."}</p>
        <DemoBox>
            <Switch aria_label={demo_label}
                default_checked={true}
                size={Size::Small} />
            <Switch aria_label={demo_label}
                default_checked={true} />
        </DemoBox>

        <h2>{"Color"}</h2>
        <DemoBox>
            <Switch aria_label={demo_label}
                default_checked={true} />
            <Switch aria_label={demo_label}
                default_checked={true}
                color={Color::Secondary} />
            <Switch aria_label={demo_label}
                default_checked={true}
                color={Color::Info} />
            <Switch aria_label={demo_label}
                default_checked={true}
                color={Color::Warning} />
           <Switch aria_label={demo_label}
                default_checked={true}
                color={Color::Error} />
            <Switch aria_label={demo_label}
                default_checked={true}
                color={Color::Default} />
        </DemoBox>

        <h2>{"Controlled"}</h2>
        <p>{"You can control the switch with the checked and onChange props:"}</p>
        <DemoBox>
            <Switch
                checked={true}
                aria_label="Controlled"
                on_change={handle_change}
            />
        </DemoBox>

        <h2>{"Label placement"}</h2>
        <p>{"You can change the placement of the label:"}</p>
        <DemoBox>
        <FormControl component="fieldset">
            <FormLabel component="legend">{"Label placement"}</FormLabel>
            <FormGroup aria_label="position" row={true}>
                <FormControlLabel
                    value="top"
                    control={html!{<Switch color={Color::Primary} />}}
                    label={html!{"Top"}}
                    label_position={Position::Top}
                />
                <FormControlLabel
                    value="start"
                    control={html!{<Switch color={Color::Primary} />}}
                    label={html!{"Start"}}
                    label_position={Position::Start}
                />
                <FormControlLabel
                    value="bottom"
                    control={html!{<Switch color={Color::Primary} />}}
                    label={html!{"Bottom"}}
                    label_position={Position::Bottom}
                />
                <FormControlLabel
                    value="end"
                    control={html!{<Switch color={Color::Primary} />}}
                    label={html!{"End"}}
                    label_position={Position::End}
                />
            </FormGroup>
        </FormControl>
        </DemoBox>

        </div>
    }
}
