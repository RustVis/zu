// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod name;
mod person;
mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::shape_variant::ShapeVariant;
use variant::variant_class;

// Re-export
pub use person::Person;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// Used in combination with src or srcSet to provide an alt attribute for the rendered img element.
    #[prop_or_default]
    pub alt: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or(false)]
    pub color_default: bool,

    #[prop_or_default]
    pub component: AttrValue,

    /// Setup avatar background color and value based on specified name.
    #[prop_or_default]
    pub name: AttrValue,

    // TODO(Shaohua): Add img_props
    //pub img_props: AttrValue,
    /// The sizes attribute for the img element.
    #[prop_or_default]
    pub sizes: AttrValue,

    /// The src attribute for the img element.
    #[prop_or_default]
    pub src: AttrValue,

    /// The srcSet attribute for the img element.
    #[prop_or_default]
    pub src_set: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: ShapeVariant,
}

#[function_component(Avatar)]
pub fn avatar(props: &Props) -> Html {
    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };
    let has_image = !props.src.is_empty() || !props.src_set.is_empty();

    let root_cls = classes!(
        "ZuAvatar-root",
        props.classes.as_str().to_owned(),
        variant_class(props.variant),
        if has_image {
            ""
        } else {
            "ZuAvatar-colorDefault"
        },
    );
    // TODO(Shaohua): Load image first.

    let img_src = if props.src.is_empty() {
        None
    } else {
        Some(props.src.to_string())
    };
    let src_set = if props.src_set.is_empty() {
        None
    } else {
        Some(props.src_set.to_string())
    };
    let alt = if props.alt.is_empty() {
        None
    } else {
        Some(props.alt.to_string())
    };
    let sizes = if props.sizes.is_empty() {
        None
    } else {
        Some(props.sizes.to_string())
    };

    let mut styles = if props.style.is_empty() {
        vec![]
    } else {
        vec![props.style.to_string()]
    };

    if !props.name.is_empty() {
        let named_color = name::name_to_color(props.name.as_str());
        styles.push(format!("background-color: {named_color};"));
    }

    let abbr_name: String = if props.name.is_empty() {
        String::new()
    } else {
        let abbr_name = name::abbr_name(props.name.as_str());
        log::info!("abbr_name: {abbr_name}");
        abbr_name
    };

    let style = if styles.is_empty() {
        None
    } else {
        Some(styles.join(";"))
    };

    html! {
        <@{component.to_owned()} class={root_cls} style={style}>
            if !props.children.is_empty() {
                {for props.children.iter()}
            } else if has_image {
                <img class="ZuAvatar-img"
                    src={img_src}
                    src-set={src_set}
                    alt={alt}
                    sizes={sizes} />
            } else if !abbr_name.is_empty() {
                {abbr_name}
            } else {
                <Person classes="ZuAvatar-fallback" />
            }
        </@>
    }
}
