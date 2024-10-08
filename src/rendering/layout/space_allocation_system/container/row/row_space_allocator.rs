use crate::rendering::elements::{
    common_types::{Position, Size},
    container::Container,
    element::Element,
    styles::{Directions, Overflow},
};

use super::{position_allocator, size_allocator};

pub fn allocate_space_to_children_row_flex(
    container: &mut Container,
    allocated_position: Position,
    allocated_size: Size,
) {
    // Unwrap styles
    let padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();
    let align_items = container.get_styles().align_items.unwrap_or_default();
    let flex_wrap = container.get_styles().flex_wrap.unwrap_or_default();
    let overflow = container.get_styles().overflow.unwrap_or_default();

    // Prepare AlignItems y computations
    let children_max_height_index = position_allocator::find_max_child_height_index(container);
    let max_height_child = &container.children[children_max_height_index];
    let children_max_height = max_height_child.get_effective_size().height;
    let max_height_child_margin = max_height_child.get_styles().margin.unwrap_or_default();

    // Prepare Overflow x computations
    // let container_starting_x = allocated_position.x + padding.left.value;
    // let container_ending_x = allocated_position.x + allocated_size.width - padding.right.value;
    let effective_horizontal_space = allocated_size.width - padding.horizontal();
    let requested_width = size_allocator::precompute_requested_children_width(container);
    if overflow == Overflow::Auto && requested_width > allocated_size.width {
        container.scrollbar_state.thumb_scrollbar_width_ratio =
            effective_horizontal_space / requested_width;
        container.scrollbar_state.is_overflowing = Directions {
            horizontal: true,
            vertical: false,
        };
    }
    let current_scroll_position_x = container.scrollbar_state.current_scroll_position.x;

    let overflow_width = requested_width - allocated_size.width;
    let scrollbar_offset = overflow_width * current_scroll_position_x;
    
    // Start allocating space to children
    let mut current_position = allocated_position;
    current_position.x += padding.left.value;
    current_position.y += padding.top.value;

    if overflow == Overflow::Auto {
        current_position.x -= scrollbar_offset;
    }

    for child in &mut container.children {
        let child_effective_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();

        let child_allocated_position = position_allocator::determine_allocated_position(
            flex_wrap,
            overflow,
            align_items,
            spacing,
            current_position,
            child_effective_size,
            children_max_height,
            max_height_child_margin,
            child_margin,
        );
        
        let child_allocated_size = size_allocator::determine_allocated_size(
            child_effective_size,
            flex_wrap,
            overflow,
        );

        child.allocate_space(child_allocated_position, child_allocated_size);

        current_position.x = child_allocated_position.x + child_allocated_size.width + child_margin.right.value;
    }
}
