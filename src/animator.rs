
use iced::Alignment;
use iced::widget::{Column, image, text, container, Image};

use crate::os::Message;
use std::env;
use std::path::PathBuf;

use crate::env::GOCO_ROOT;

#[derive(Debug, PartialEq)]
pub struct Animation{
    image_index: usize,
    text_index: usize,
    frames: [Vec<u8>; IMAGE_FRAMES as usize],
}

const TEXT_FRAMES: usize = 4;
const IMAGE_FRAMES: usize = 2;

impl Animation {

    pub fn new() -> Self {
        let i0 = std::fs::read("./assets/a1.png").unwrap();
        let i1 = std::fs::read("./assets/a2.png").unwrap();
        Self {
            text_index: 0,
            image_index: 0,
            frames: [i0.clone(), i1],
        }
    }

    /// Returns the console's included gmaestick insertion icon to display when no
    /// gamestick is detected.
    /// 
    /// Reads from the `GOCO_ROOT` environment variable to determine the base directory
    /// for finding the `assets/insert.gif` file.
    fn get_image(i: usize) -> PathBuf {
        PathBuf::from(format!("{}/assets/a{}.png", env::var_os(GOCO_ROOT).unwrap_or(".".into()).to_string_lossy(), i))
    }

    pub fn get_text(&self) -> &str {
        match self.text_index {
            0 => "Please Insert GAMESTICK",
            1 => "Please Insert GAMESTICK .",
            2 => "Please Insert GAMESTICK . .",
            3 => "Please Insert GAMESTICK . . .",
            _ => panic!("unreachable text animation index: {}", self.text_index)
        }
    }

    /// Updates the animator to access the next image.
    pub fn next(&mut self) {
        self.text_index += 1;
        self.image_index = 0;
        // reset the frame to replay the animation if exceeding last frame count
        if self.text_index >= TEXT_FRAMES {
            self.text_index = 0;
        }
        // reset the frame to replay the animation if exceeding last frame count
        if self.image_index >= IMAGE_FRAMES {
            self.image_index = 0;
        }
    }

    /// Accesses the current frame index.
    fn get_current_frame(&self) -> &Vec<u8> {
        &self.frames[self.image_index]
    }
}

use iced::widget::image::Handle;
use iced::Length;

impl<'a> Animation {
    pub fn container(title: Option<&str>) -> Column<'a, Message> {
        match title {
            Some(s) => iced::widget::column![text(s).size(50)].spacing(20),
            None => iced::widget::column![].spacing(20)
        }
    }

    /// Assembles the container to display an empty slot for a [Game] in the console's main library screen.
    /// The function returns a blank icon and no text, but in the same format as a valid game would be.
    pub fn draw(&self) -> Column<'a, Message> {
        let img: Image = image(Handle::from_memory(self.get_current_frame().clone()));
        Self::container(None)
            .push(
                container(image(Handle::from_memory(self.get_current_frame().clone()))
                .width(Length::Fill)
                .height(Length::Fill))
                .center_x()         
            )
            .push(
                text("")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .align_items(Alignment::Center)
    }
}