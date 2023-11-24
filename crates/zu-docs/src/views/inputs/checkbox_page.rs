// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, use_state, Callback, Html};
use zu::boxed::Box;
use zu::checkbox::Checkbox;
use zu::form_control::FormControl;
use zu::form_control_label::FormControlLabel;
use zu::form_group::FormGroup;
use zu::form_label::FormLabel;
use zu::styles::color::Color;
use zu::styles::position::Position;
use zu::styles::size::Size;
use zuicon_material::Bookmark as BookmarkIcon;
use zuicon_material::BookmarkBorder as BookmarkBorderIcon;
use zuicon_material::Favorite as FavoriteIcon;
use zuicon_material::FavoriteBorder as FavoriteBorderIcon;

use crate::components::demo_box::DemoBox;

fn basic_sections() -> Html {
    let handle_change = Callback::from(|checked: bool| {
        log::info!("handle change, is checked: {checked}");
    });

    html! {
        <>
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
        </>
    }
}

#[function_component(IndeterminateSection)]
fn indeterminate_section() -> Html {
    let child1_checked = use_state(|| false);
    let child2_checked = use_state(|| true);
    let on_child1_changed = {
        let child1_checked_clone = child1_checked.clone();
        Callback::from(move |checked: bool| {
            log::info!("on child1 changed: {checked}");
            child1_checked_clone.set(checked);
        })
    };
    let on_child2_changed = {
        let child2_checked_clone = child2_checked.clone();
        Callback::from(move |checked: bool| {
            log::info!("on child2 changed: {checked}");
            child2_checked_clone.set(checked);
        })
    };

    html! {
        <>
        <h2>{"Indeterminate"}</h2>
        <p>{"A checkbox input can only have two states in a form: checked or unchecked. \
        It either submits its value or doesn't. Visually, there are three states a checkbox can be in: \
        checked, unchecked, or indeterminate."}</p>
        <DemoBox>
        <div>
          <FormControlLabel
            label="Parent"
            control={html!{
            <Checkbox
                default_checked={true}
                indeterminate={!(*child1_checked && *child2_checked)}
              />
            }}
          />
          <Box style="display: flex; flex-direction: column; margin-left: 3px;">
            <FormControlLabel
                label="Child 1"
                control={html!{
                    <Checkbox default_checked={false} on_change={Some(on_child1_changed)} />
                }}
            />
            <FormControlLabel
                label="Child 2"
                control={html!{
                    <Checkbox default_checked={true} on_change={Some(on_child2_changed)} />
                }}
            />
            </Box>
        </div>
        </DemoBox>
        </>
    }
}

fn form_group_section() -> Html {
    // TODO(Shaohua): Add form group

    html! {
        <>
        <h2>{"FormGroup"}</h2>
        <p>{"FormGroup is a helpful wrapper used to group selection control components."}</p>
        </>
    }
}

fn placement_section() -> Html {
    html! {
        <>
        <h2>{"Label placement"}</h2>
        <p>{"You can change the placement of the label:"}</p>
        <DemoBox>
        <FormControl component="fieldset">
            <FormLabel component="legend">{"Label placement"}</FormLabel>
            <FormGroup aria_label="position" row={true}>
                <FormControlLabel
                  value="top"
                  control={html!{<Checkbox />}}
                  label="Top"
                  label_position={Position::Top}
                />
                <FormControlLabel
                  value="start"
                  control={html!{<Checkbox />}}
                  label="Start"
                  label_position={Position::Start}
                />
                <FormControlLabel
                  value="bottom"
                  control={html!{<Checkbox />}}
                  label="Bottom"
                  label_position={Position::Bottom}
                />
                <FormControlLabel
                  value="end"
                  control={html!{<Checkbox />}}
                  label="End"
                  label_position={Position::End}
                />
            </FormGroup>
        </FormControl>
        </DemoBox>
        </>
    }
}

#[function_component(CheckboxPage)]
pub fn checkbox_page() -> Html {
    html! {
        <div class="container">

        <h1>{"Checkbox"}</h1>
        <p>{"Checkboxes can be used to turn an option on or off."}</p>

        {basic_sections()}
        <IndeterminateSection />
        {form_group_section()}
        {placement_section()}

        </div>
    }
}
