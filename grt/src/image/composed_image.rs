use std::rc::Rc;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;

use image::Image;
use resource::ResourceBuilder;
use io::{TextRenderer, Quad};
use ui::{AnimationState, Size};
use util::Point;

use serde_json;
use serde_yaml;

const GRID_DIM: i32 = 3;
const GRID_LEN: i32 = GRID_DIM * GRID_DIM;

#[derive(Debug)]
pub struct ComposedImage {
    images: Vec<Rc<Image>>,

    size: Size,
    middle_size: Size,
}

impl ComposedImage {
    pub fn new(builder: ComposedImageBuilder,
               images: &HashMap<String, Rc<Image>>) -> Result<Rc<Image>, Error> {
        if builder.grid.len() as i32 != GRID_LEN {
            return Err(Error::new(ErrorKind::InvalidData,
                format!("Composed image grid must be length {}", GRID_LEN)));
        }

        let mut images_vec: Vec<Rc<Image>> = Vec::new();
        for id in builder.grid {
           let image = images.get(&id);
           if let None = image {
                return Err(Error::new(ErrorKind::InvalidData,
                    format!("Unable to locate sub image {}", id)));
           }

           let image = image.unwrap();
           images_vec.push(Rc::clone(image));
        }

        // verify heights make sense for the grid
        let mut total_height = 0;
        for y in 0..GRID_DIM {
            let row_height = images_vec.get((y * GRID_DIM) as usize)
                .unwrap().get_size().height;

            for x in 0..GRID_DIM {
                let height = images_vec.get((y * GRID_DIM + x) as usize)
                    .unwrap().get_size().height;

                if height != row_height {
                    return Err(Error::new(ErrorKind::InvalidData,
                         format!("All images in row {} must have the same height", y)));
                }
            }
            total_height += row_height;
        }

        //verify widths make sense for the grid
        let mut total_width = 0;
        for x in 0..GRID_DIM {
            let col_width = images_vec.get(x as usize).unwrap().get_size().width;

            for y in 0..GRID_DIM {
                let width = images_vec.get((y * GRID_DIM + x) as usize)
                    .unwrap().get_size().width;

                if width != col_width {
                    return Err(Error::new(ErrorKind::InvalidData,
                        format!("All images in col {} must have the same width", x)));
                }
            }
            total_width += col_width;
        }

        let middle_size = *images_vec.get((GRID_LEN / 2) as usize).unwrap().get_size();

        Ok(Rc::new(ComposedImage {
            images: images_vec,
            size: Size::new(total_width, total_height),
            middle_size,
        }))
    }
}

impl Image for ComposedImage {
    fn draw_text_mode(&self, renderer: &mut TextRenderer, state: &AnimationState,
                      position: &Point) {
        let x = position.x;
        let y = position.y;
        renderer.set_cursor_pos(x, y);

        let mut cur_x = x;
        let mut cur_y = y;
        for (index, image) in self.images.iter().enumerate() {
            let index = index as i32;
            image.draw_text_mode(renderer, state, &Point::new(cur_x, cur_y));

            if index % GRID_DIM == GRID_DIM - 1 {
                cur_x = x;
                cur_y += image.get_size().height;
            }
        }
    }

    //// Renders text for this composed image to the given coordinates.
    //// This method assumes that 'GRID_DIM' equals 3 for simplicity
    //// and performance purposes.
    fn fill_text_mode(&self, renderer: &mut TextRenderer, state: &AnimationState,
                      position: &Point, size: &Size) {
        let fill_size = *size - (self.size - self.middle_size);
        let mut draw_pos = Point::from(position);
        let mut draw_size = Size::from(&fill_size);

        unsafe {
            let image = self.images.get_unchecked(0);
            image.draw_text_mode(renderer, state, &draw_pos);

            let image = self.images.get_unchecked(1);
            draw_size.set_height(image.get_size().height);
            draw_pos.add_x(image.get_size().width);
            image.fill_text_mode(renderer, state, &draw_pos, &draw_size);

            let image = self.images.get_unchecked(2);
            draw_pos.add_x(fill_size.width);
            image.draw_text_mode(renderer, state, &draw_pos);

            let image = self.images.get_unchecked(3);
            draw_pos.set_x(position.x);
            draw_pos.add_y(image.get_size().height);
            draw_size.set(image.get_size().width, fill_size.height);
            image.fill_text_mode(renderer, state, &draw_pos, &draw_size);

            let image = self.images.get_unchecked(4);
            draw_pos.add_x(image.get_size().width);
            image.fill_text_mode(renderer, state, &draw_pos, &fill_size);

            let image = self.images.get_unchecked(5);
            draw_pos.add_x(fill_size.width);
            draw_size.set_width(image.get_size().width);
            image.fill_text_mode(renderer, state, &draw_pos, &draw_size);

            let image = self.images.get_unchecked(6);
            draw_pos.add_y(fill_size.height);
            draw_pos.set_x(position.x);
            image.draw_text_mode(renderer, state, &draw_pos);

            let image = self.images.get_unchecked(7);
            draw_pos.add_x(image.get_size().width);
            draw_size.set(fill_size.width, image.get_size().height);
            image.fill_text_mode(renderer, state, &draw_pos, &draw_size);

            let image = self.images.get_unchecked(8);
            draw_pos.add_x(fill_size.width);
            image.draw_text_mode(renderer, state, &draw_pos);
        }
    }

    fn get_quads(&self, state: &AnimationState, position: &Point, size: &Size) -> Vec<Quad> {
        let fill_size = *size - (self.size - self.middle_size);
        let mut draw_pos = Point::from(position);
        let mut draw_size = Size::from(&fill_size);

        let mut quads: Vec<Quad> = Vec::with_capacity(9);
        unsafe {
            let image = self.images.get_unchecked(0);
            quads.append(&mut image.get_quads(state, &draw_pos, image.get_size()));
            // image.draw_text_mode(renderer, state, &draw_pos);

            let image = self.images.get_unchecked(1);
            draw_size.set_height(image.get_size().height);
            draw_pos.add_x(image.get_size().width);
            quads.append(&mut image.get_quads(state, &draw_pos, &draw_size));
            // image.fill_text_mode(renderer, state, &draw_pos, &draw_size);

            let image = self.images.get_unchecked(2);
            draw_pos.add_x(fill_size.width);
            quads.append(&mut image.get_quads(state, &draw_pos, image.get_size()));
            // image.draw_text_mode(renderer, state, &draw_pos);

            let image = self.images.get_unchecked(3);
            draw_pos.set_x(position.x);
            draw_pos.add_y(image.get_size().height);
            draw_size.set(image.get_size().width, fill_size.height);
            quads.append(&mut image.get_quads(state, &draw_pos, &draw_size));
            // image.fill_text_mode(renderer, state, &draw_pos, &draw_size);

            let image = self.images.get_unchecked(4);
            draw_pos.add_x(image.get_size().width);
            quads.append(&mut image.get_quads(state, &draw_pos, &fill_size));
            // image.fill_text_mode(renderer, state, &draw_pos, &fill_size);

            let image = self.images.get_unchecked(5);
            draw_pos.add_x(fill_size.width);
            draw_size.set_width(image.get_size().width);
            quads.append(&mut image.get_quads(state, &draw_pos, &draw_size));
            // image.fill_text_mode(renderer, state, &draw_pos, &draw_size);

            let image = self.images.get_unchecked(6);
            draw_pos.add_y(fill_size.height);
            draw_pos.set_x(position.x);
            quads.append(&mut image.get_quads(state, &draw_pos, image.get_size()));
            // image.draw_text_mode(renderer, state, &draw_pos);

            let image = self.images.get_unchecked(7);
            draw_pos.add_x(image.get_size().width);
            draw_size.set(fill_size.width, image.get_size().height);
            quads.append(&mut image.get_quads(state, &draw_pos, &draw_size));
            // image.fill_text_mode(renderer, state, &draw_pos, &draw_size);

            let image = self.images.get_unchecked(8);
            draw_pos.add_x(fill_size.width);
            quads.append(&mut image.get_quads(state, &draw_pos, image.get_size()));
            // image.draw_text_mode(renderer, state, &draw_pos);
        }

        quads
    }

    fn get_size(&self) -> &Size {
        &self.size
    }
}

#[derive(Deserialize, Debug)]
pub struct ComposedImageBuilder {
    pub id: String,
    pub grid: Vec<String>,
}

impl ResourceBuilder for ComposedImageBuilder {
    fn owned_id(&self) -> String {
        self.id.to_owned()
    }

    fn from_json(data: &str) -> Result<ComposedImageBuilder, Error> {
        let resource: ComposedImageBuilder = serde_json::from_str(data)?;

        Ok(resource)
    }

    fn from_yaml(data: &str) -> Result<ComposedImageBuilder, Error> {
        let resource: Result<ComposedImageBuilder, serde_yaml::Error> = serde_yaml::from_str(data);

        match resource {
            Ok(resource) => Ok(resource),
            Err(error) => Err(Error::new(ErrorKind::InvalidData, format!("{}", error)))
        }
    }
}
