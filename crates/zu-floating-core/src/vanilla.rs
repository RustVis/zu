// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

//! This module is designed for unittest only.

use std::any::Any;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::platform::{Boundary, Element, ElementRects, Platform, RootBoundary};
use crate::types::{Dimensions, Length, LengthTrait, Rect, Strategy};

pub type VanillaElementId = i32;

#[derive(Debug)]
pub struct VanillaElement {
    id: VanillaElementId,
    rect: Rect,
}

impl PartialEq for VanillaElement {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl LengthTrait for VanillaElement {
    fn length(&self, length: Length) -> f64 {
        self.rect.length(length)
    }

    fn set_length(&mut self, length: Length, val: f64) {
        self.rect.set_length(length, val);
    }
}

impl Element for VanillaElement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct VanillaPlatform {
    elements: BTreeMap<VanillaElementId, Rc<RefCell<VanillaElement>>>,
    viewport: Rect,
}

const REF_ELEMENT_ID: i32 = 1;
const FLOATING_ELEMENT_ID: i32 = 2;

impl Default for VanillaPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl VanillaPlatform {
    #[must_use]
    pub fn new() -> Self {
        let mut elements = BTreeMap::new();
        elements.insert(
            REF_ELEMENT_ID,
            Rc::new(RefCell::new(VanillaElement {
                id: REF_ELEMENT_ID,
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 80.0,
                    height: 80.0,
                },
            })),
        );
        elements.insert(
            FLOATING_ELEMENT_ID,
            Rc::new(RefCell::new(VanillaElement {
                id: FLOATING_ELEMENT_ID,
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 50.0,
                    height: 50.0,
                },
            })),
        );
        Self {
            elements,
            viewport: Rect {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
        }
    }

    // #[must_use]
    // pub fn floating_element(&self) -> &Rc<dyn Element> {
    //     self.elements.get(&FLOATING_ELEMENT_ID).unwrap()
    // }
    //
    // #[must_use]
    // pub fn reference_element(&self) -> &Rc<dyn Element> {
    //     self.elements.get(&REF_ELEMENT_ID).unwrap()
    // }
}

impl Platform for VanillaPlatform {
    fn element_rects(
        &self,
        reference_element: &Rc<dyn Element>,
        floating_element: &Rc<dyn Element>,
        _strategy: Strategy,
    ) -> ElementRects {
        let real_reference_element: &VanillaElement = reference_element
            .as_ref()
            .as_any()
            .downcast_ref::<VanillaElement>()
            .unwrap();
        let real_floating_element: &VanillaElement = floating_element
            .as_ref()
            .as_any()
            .downcast_ref::<VanillaElement>()
            .unwrap();

        assert!(self.elements.contains_key(&real_reference_element.id));
        assert!(self.elements.contains_key(&real_floating_element.id));

        ElementRects {
            reference: real_reference_element.rect.clone(),
            floating: real_floating_element.rect.clone(),
        }
    }

    fn dimensions(&self, element: &Rc<dyn Element>) -> Dimensions {
        Dimensions {
            width: element.length(Length::Width),
            height: element.length(Length::Height),
        }
    }

    fn clipping_rect(
        &self,
        _element: &Rc<dyn Element>,
        _boundary: &Boundary,
        _root_boundary: &RootBoundary,
        _strategy: Strategy,
    ) -> Rect {
        assert!(self.viewport.width > 0.0 && self.viewport.height > 0.0);
        todo!()
    }
}
