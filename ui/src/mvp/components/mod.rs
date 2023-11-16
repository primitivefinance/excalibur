pub mod button;
pub mod containers;
pub mod input;
pub mod logos;
pub mod styles;

use button::*;
use containers::*;
use iced::{Color, Element, Renderer};
use input::*;
use logos::*;
use styles::*;

// These components should return View messages.
use super::{view::Message, *};

/// Renders a gray text label in lowercase.
pub fn label_item<'a>(t: String) -> Text<'a> {
    let content = t.to_lowercase();
    text(content).size(16).style(Color::from_rgb(0.5, 0.5, 0.5))
}

/// Renders white text in the DAGGERSQUARE font.
pub fn data_item<'a>(t: String) -> Text<'a> {
    text(t).font(FONT_DAGGERSQUARE).style(Color::WHITE)
}

/// Renders a column with a label and an element.
pub fn labeled<'a, T: Into<Element<'a, Message>>>(
    label: String,
    element: T,
) -> Element<'a, Message> {
    let mut content = Column::new().push(label_item(label)).push(element.into());
    content = content.spacing(8);
    content.into()
}

/// Renders a row of labeled controls, where each control has a label.
pub fn labeled_controls<'a, T: Into<Element<'a, Message>>>(
    controls: Vec<(String, T)>,
) -> Element<'a, Message> {
    let mut content = Row::new();
    for (label, control) in controls {
        content = content.push(labeled(label, control));
    }
    content.spacing(4).into()
}

/// Renders a column with a label and a piece of data with the DAGGERSQUARE
/// font.
pub fn labeled_data<'a>(label: String, data: String) -> Element<'a, Message, Renderer> {
    let mut content = Column::new()
        .push(label_item(label))
        .push(data_item(data).size(20));
    content = content.spacing(8);
    content.into()
}

/// Renders a nice blue button.
pub fn action_button<'a>(label: String) -> iced::widget::Button<'a, Message> {
    let content = text(label)
        .size(16)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .style(Color::WHITE);
    let action_button_style = CustomButtonStyle::new()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(35, 88, 226))
        .hovered()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(88, 135, 255))
        .pressed()
        .border_radius(5.0.into())
        .background_color(Color::from_rgb8(11, 63, 197));
    button(content).style(action_button_style.as_custom())
}

/// Container that groups actions or settings with a label and a row of
/// controls.
pub fn controls_container<'a, T: Into<Element<'a, Message>>>(
    label: String,
    actions: Vec<T>,
) -> Element<'a, Message> {
    let mut content = Column::new().push(label_item(label));
    let mut row = Row::new().spacing(4);
    for action in actions {
        row = row.push(action.into());
    }
    content = content.push(row);
    content.spacing(8).into()
}

/// Containers that groups multiple labeled data pieces under a label
pub fn labeled_data_container<'a>(
    label: String,
    data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message> {
    let mut content = Column::new().push(label_item(label));
    content = content.push(labeled_data_row(data, max_elements));
    content.into()
}

/// Renders a row of labeled data elements using labeled_data. Specify the
/// maximum amount of elements in the row, if the total amount of elements
/// exceeds the value, it will push a new row to the column.
pub fn labeled_data_row<'a>(
    label_data: Vec<(String, String)>,
    max_elements: usize,
) -> Element<'a, Message, Renderer> {
    let mut content = Column::new();
    let mut row = Row::new().spacing(4);
    let mut i = 0;
    for (label, data) in label_data {
        row = row.push(labeled_data(label, data));
        i += 1;
        if i == max_elements {
            content = content.push(row);
            row = Row::new().spacing(4);
            i = 0;
        }
    }
    content = content.push(row);
    content.spacing(8).into()
}

/// A container that spaces the elements in a row out so they fill the space.
pub fn space_between_row<'a, T: Into<Element<'a, Message, Renderer>>>(
    elements: Vec<T>,
) -> Element<'a, Message> {
    let mut content = Row::new()
        .spacing(8)
        .width(Length::Shrink)
        .align_items(alignment::Alignment::Center);

    // The first element should be wrapped in a column that has align_items with
    // Start The last element should have the same but with End alignment.
    // All other elements should have center alignment.
    let len = elements.len();
    for (i, element) in elements.into_iter().enumerate() {
        content = content.push(
            Column::new()
                .align_items(match i {
                    0 => alignment::Alignment::Start,
                    _ if i == len - 1 => alignment::Alignment::End,
                    _ => alignment::Alignment::Center,
                })
                .push(element.into())
                .width(Length::FillPortion(1)),
        );
    }
    content.into()
}

/// Creates a row of two 50% width columns with the given elements.
/// todo: replace proper spacing and padding sizes.
pub fn dual_column<'a, T: Into<Element<'a, Message>>>(
    first_column: Vec<T>,
    second_column: Vec<T>,
) -> Row<'a, Message> {
    let first_column = Column::with_children(first_column.into_iter().map(|e| e.into()).collect())
        .height(Length::Fill)
        .width(Length::FillPortion(2))
        .spacing(16);

    let second_column =
        Column::with_children(second_column.into_iter().map(|e| e.into()).collect())
            .height(Length::Fill)
            .width(Length::FillPortion(2))
            .spacing(16);

    Row::new()
        .width(Length::Fill)
        .spacing(8)
        .align_items(alignment::Alignment::Center)
        .push(first_column)
        .push(second_column)
}