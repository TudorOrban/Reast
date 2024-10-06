use skia_safe::{Canvas, Color, Point};

use crate::rendering::{
    layout::{
        allocate_space::allocate_space_to_children,
        estimate_size::{estimate_leaf_container_size, estimate_parent_container_size},
    },
    rendering_interface::element_renderer::ElementRenderer,
};

use super::{
    common_types::{OptionalSize, Position, Size},
    element::{Element, ElementType, EventType},
    element_id_generator::IDGenerator,
    styles::Styles,
};

pub struct Container {
    _id: String,
    position: Position,
    size: Size,
    natural_size: Size,
    requested_size: OptionalSize,
    styles: Styles,
    pub children: Vec<Box<dyn Element>>,
}

impl Container {
    pub fn new() -> Self {
        let id = IDGenerator::get();
        Self {
            _id: id,
            position: Position::default(),
            size: Size::default(),
            natural_size: Size::default(),
            requested_size: OptionalSize::default(),
            styles: Styles::default(),
            children: Vec::new(),
        }
    }

    pub fn add_children(&mut self, children: Vec<Box<dyn Element>>) -> &mut Self {
        self.children.extend(children);
        self
    }

    pub fn set_styles(&mut self, styles: Styles) -> &mut Self {
        self.styles = styles;
        self
    }
}

impl Element for Container {
    fn render(&self, canvas: &Canvas) {
        ElementRenderer::render_element(
            canvas,
            self.position,
            self.size,
            self.styles.background_color.unwrap_or(Color::TRANSPARENT),
            self.styles.border.unwrap_or_default().width,
            self.styles.border.unwrap_or_default().color,
        );

        for child in &self.children {
            child.render(canvas);
        }
    }

    fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }

    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType) {
        for child in &mut self.children {
            child.handle_event(cursor_position, event_type);
        }
    }

    fn set_id(&mut self, id: String) {
        self._id = id;
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn add_child(&mut self, child: Box<dyn Element>) {
        self.children.push(child);
    }

    fn get_id(&self) -> String {
        self._id.clone()
    }

    fn get_element_type(&self) -> ElementType {
        ElementType::Container
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_styles(&self) -> Styles {
        self.styles
    }

    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>> {
        Some(&mut self.children)
    }

    // Layout system
    fn set_natural_size(&mut self, size: Size) {
        self.natural_size = size;
    }

    fn set_requested_size(&mut self, requested_size: OptionalSize) {
        self.requested_size = requested_size;
    }

    fn get_natural_size(&self) -> Size {
        self.natural_size
    }

    fn get_requested_size(&self) -> OptionalSize {
        self.requested_size.clone()
    }

    // Traverse the tree from leaves to root and estimate the size of each container.
    fn estimate_size(&mut self) {
        if !self.children.is_empty() {
            for child in &mut self.children {
                child.estimate_size();
            }

            estimate_parent_container_size(self);
        } else {
            estimate_leaf_container_size(self);
        }
    }

    fn allocate_space(&mut self, allocated_position: Position, allocated_size: Size) {
        self.position = allocated_position;
        self.size = allocated_size;

        allocate_space_to_children(self, allocated_position, allocated_size);
    }
}
