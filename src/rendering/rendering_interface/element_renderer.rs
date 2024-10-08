use skia_safe::{font_style::{Slant, Width}, Canvas, Color, Font, FontMgr, FontStyle, Paint, PaintStyle, Point, Rect};

use crate::rendering::elements::{common_types::{Position, Size}, styles::{Dimension, Directions, FontFamily, FontStyle as CustomFontStyle, FontWeight}};

use super::custom_to_skia_types_mapper::{map_custom_to_skia_font_style, map_custom_to_skia_font_weight};


pub struct ElementRenderer {
    
}

impl ElementRenderer {
    pub fn render_element(
        canvas: &Canvas,
        position: Position, 
        size: Size, 
        background_color: Color,
        border_width: Dimension,
        border_color: Color
    ) {
        if (size.width <= 0.0) || (size.height <= 0.0) {
            return;
        }

        let row_rect = Rect::from_point_and_size(
            Point::new(position.x,
                       position.y),
            (size.width,
             size.height)
        );
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(background_color);
        canvas.draw_rect(row_rect, &paint);
         
        if border_width.value == 0.0 {
            // return; // Draw all borders for now
        }
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint.set_stroke_width(border_width.value);
        paint.set_color(border_color);
        canvas.draw_rect(row_rect, &paint);
    }

    pub fn render_scrollbar(
        canvas: &Canvas,
        position: Position,
        size: Size,
        directions: Directions,
        current_position: f32, // Between 0.0 and 1.0
        thumb_scrollbar_width_ratio: f32,
    ) {
        // Draw outer rectangle
        let mut paint = Paint::default();
        let scrollbar_rect = Rect::from_point_and_size(
            Point::new(position.x,
                       position.y),
            (size.width,
             size.height)
        );
        paint.set_style(PaintStyle::Fill);
        paint.set_color(Color::from_argb(255, 200, 200, 200));

        canvas.draw_rect(scrollbar_rect, &paint);

        // Draw thumb
        let thumb_width = size.width * thumb_scrollbar_width_ratio;
        let thumb_size = Size {
            width: thumb_width,
            height: size.height * 0.8,
        };
        let max_left_position = position.x;
        let max_right_position = position.x + size.width - thumb_width;

        let thumb_position = Position {
            x: max_left_position + (current_position * (max_right_position - max_left_position)),
            y: position.y,
        };

        let thumb_rect = Rect::from_point_and_size(
            Point::new(thumb_position.x,
                       thumb_position.y),
            (thumb_size.width,
             thumb_size.height)
        );

        paint.set_color(Color::from_argb(255, 100, 100, 100));
        canvas.draw_rect(thumb_rect, &paint);
    }

    pub fn render_text(
        canvas: &Canvas,
        position: Position, 
        size: Size, 
        text_color: Color,
        font_size: f32,
        font_weight: FontWeight,
        font_family: FontFamily,
        font_style: CustomFontStyle,
        text_content: String,
    ) {
        let font_mgr = FontMgr::default();
        let slant: Slant = map_custom_to_skia_font_style(&font_style);
        let weight = map_custom_to_skia_font_weight(&font_weight);
        let font_style = FontStyle::new(weight, Width::from(20), slant);
        
        let typeface = font_mgr.match_family_style(font_family.to_string(), font_style)
            .expect("Unable to create typeface");

        let font = Font::new(typeface, font_size);

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(text_color);

        let position = Point::new(position.x, position.y);

        canvas.draw_str(text_content, position, &font, &paint);
    }
}