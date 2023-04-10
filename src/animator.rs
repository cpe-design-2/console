use iced::{Alignment, Length};
use iced::widget::{Column, image, text, container, image::Handle};

use crate::os::Message;
use crate::env::GOCO_ROOT;
use std::env;

#[derive(Debug, PartialEq)]
pub struct Animation{
    text_index: usize,
    buf: Vec<u8>,
}

const TEXT_FRAMES: usize = 4;

impl Animation {

    pub fn new() -> Self {
        let i0 = std::fs::read(format!("{}/assets/insert.png", env::var_os(GOCO_ROOT).unwrap_or(".".into()).to_string_lossy())).unwrap_or(Vec::new());
        Self {
            text_index: 0,
            buf: i0,
        }
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
        // reset the frame to replay the animation if exceeding last frame count
        if self.text_index >= TEXT_FRAMES {
            self.text_index = 0;
        }
    }

    /// Accesses the current frame index.
    fn get_current_frame(&self) -> &Vec<u8> {
        &self.buf
    }
}

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