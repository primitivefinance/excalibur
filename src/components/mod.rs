//! Combines components into more complex components.
//! All modules in this directory are either underlying component wrappers or
//! the system module that defines most of the styling system.
//!
//! All the components that combine these parts and the styling system live in
//! here.

pub mod logos;
pub mod progress;
pub mod select;
pub mod styles;
pub mod system;
pub mod transactions;

use iced::widget::{button, container, pick_list, text, Button, Column, Container, Row, Text};
use iced::{alignment, Color, Element, Fill, Length, Padding, Renderer};
use styles::*;

use iced_aw::{
    BOOTSTRAP_FONT,
    {bootstrap::icon_to_char, Bootstrap},
};

use self::system::{label, ExcaliburButton, ExcaliburColor, ExcaliburContainer};

use crate::view;

/// Renders a tab-like button
#[allow(dead_code)]
pub fn tab_button<'a, Message>(active: bool, label: String) -> iced::widget::Button<'a, Message>
where
    Message: 'a,
{
    let content = iced::widget::text(label)
        .size(16)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .color(Color::WHITE);

    button(with_lower_indicator(
        active,
        Column::new().push(content).padding(Padding {
            top: 12.0,
            right: 5.0,
            bottom: 12.0,
            left: 5.0,
        }),
    ))
}

pub fn with_lower_indicator<'a, Message>(
    toggle: bool,
    elem: impl Into<Element<'a, Message>>,
) -> Column<'a, Message>
where
    Message: 'a,
{
    let indicator = if toggle {
        container(Column::new())
            .style(move |_theme| ExcaliburContainer::indicator().theme())
            .height(Length::Fixed(2.0))
    } else {
        container(Column::new()).height(Length::Fixed(2.0))
    };

    Column::new()
        .push(elem)
        .push(indicator.width(Length::Fixed(100.0)))
        .align_x(alignment::Alignment::Center)
}
#[allow(dead_code)]
pub fn copyable_text<'a, E: Into<Element<'a, view::ViewMessage>>>(
    label: E,
    value: String,
) -> iced::widget::Button<'a, view::ViewMessage> {
    let copy_button = button(label).padding(0).on_press(view::ViewMessage::Root(
        view::RootMessage::CopyToClipboard(value),
    ));
    copy_button
}

/// For use in the instructions container.
#[allow(dead_code)]
pub fn instruction_text<'a>(value: String) -> Text<'a> {
    label(value).highlight().build()
}

#[allow(dead_code)]
pub fn instructions_inner<'a, Message, T: Into<Element<'a, Message>>>(
    instructions: Vec<T>,
) -> Column<'a, Message>
where
    Message: 'static + Clone,
{
    let mut inner: Column<'a, Message> = Column::new()
        .spacing(Sizes::Sm)
        .padding(Sizes::Sm)
        .width(Length::Fill);

    for instruction in instructions {
        inner = inner.push(instruction);
    }

    inner
}

/// Renders an instructions title, ctaription, an action button and feedback
/// in a card.
/// note: Message must be `Clone` for the submit button to be converted to an
/// Element.
#[allow(dead_code)]
pub fn instructions<'a, Message, T: Into<Element<'a, Message>>>(
    instructions: Vec<T>,
    action: Option<String>,
    feedback: Option<String>,
    on_submit: Option<Message>,
) -> Container<'a, Message>
where
    Message: 'a + Clone + Default,
{
    let mut inner: Column<'a, Message> = Column::new()
        .spacing(Sizes::Sm)
        .padding(Sizes::Sm)
        .width(Length::Fill)
        .push(label("Instructions").title2().build());

    for instruction in instructions {
        inner = inner.push(instruction.into());
    }

    let mut submit: Button<'a, Message> = ExcaliburButton::new()
        .build(
            iced::widget::text(action.unwrap_or("Submit".to_string()))
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
        )
        .padding(Sizes::Md)
        .width(Length::Fill);

    if let Some(on_submit) = on_submit {
        submit = submit.on_press(on_submit)
    }

    let feedback = label(feedback.unwrap_or("No feedback to report".to_string()))
        .highlight()
        .caption2()
        .secondary()
        .build()
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center);

    // Card::new(
    // Column::new()
    // .push(inner)
    // .push(submit)
    // .push(feedback)
    // .spacing(Sizes::Md)
    // .padding(Sizes::Md),
    // )

    Container::new(
        Row::new()
            .spacing(Sizes::Md)
            .align_y(alignment::Alignment::Center)
            .push(inner.width(Length::FillPortion(2)))
            .push(
                Column::new()
                    .spacing(Sizes::Xs)
                    .width(Length::FillPortion(2))
                    .push(submit)
                    .push(feedback),
            ),
    )
}

pub struct DualColumn<'a, Message>
where
    Message: 'a,
{
    pub column_1: Vec<Element<'a, Message>>,
    pub column_2: Vec<Element<'a, Message>>,
    pub spacing: Option<Sizes>,
    pub padding: Option<Padding>,
    pub column_1_alignment: Option<alignment::Alignment>,
    pub column_2_alignment: Option<alignment::Alignment>,
}

impl<'a, Message> Default for DualColumn<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> DualColumn<'a, Message> {
    pub fn new() -> Self {
        Self {
            column_1: vec![],
            column_2: vec![],
            spacing: None,
            padding: None,
            column_1_alignment: None,
            column_2_alignment: None,
        }
    }

    pub fn column_1_alignment(mut self, alignment: alignment::Alignment) -> Self {
        self.column_1_alignment = Some(alignment);
        self
    }

    pub fn column_2_alignment(mut self, alignment: alignment::Alignment) -> Self {
        self.column_2_alignment = Some(alignment);
        self
    }

    pub fn column_1(mut self, column_1: Vec<Element<'a, Message>>) -> Self {
        self.column_1 = column_1;
        self
    }

    pub fn column_2(mut self, column_2: Vec<Element<'a, Message>>) -> Self {
        self.column_2 = column_2;
        self
    }

    pub fn columns(
        mut self,
        column_1: Vec<Element<'a, Message>>,
        column_2: Vec<Element<'a, Message>>,
    ) -> Self {
        self.column_1 = column_1;
        self.column_2 = column_2;
        self
    }

    pub fn spacing(mut self, spacing: Sizes) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn build(self) -> Row<'a, Message> {
        let mut row = Row::new();

        let mut first_column = Column::with_children(self.column_1.into_iter())
            .width(Length::FillPortion(2))
            .align_x(
                self.column_1_alignment
                    .unwrap_or(alignment::Alignment::Start),
            );

        let mut second_column = Column::with_children(self.column_2.into_iter())
            .width(Length::FillPortion(2))
            .align_x(self.column_2_alignment.unwrap_or(alignment::Alignment::End));

        if let Some(spacing) = self.spacing {
            first_column = first_column.spacing(spacing);
            second_column = second_column.spacing(spacing);
        }

        if let Some(padding) = self.padding {
            first_column = first_column.padding(padding);
            second_column = second_column.padding(padding);
        }

        row = row.push(first_column);
        row = row.push(second_column);
        row
    }
}

impl<'a, Message> From<DualColumn<'a, Message>> for Row<'a, Message> {
    fn from(dual_column: DualColumn<'a, Message>) -> Self {
        dual_column.build()
    }
}

#[allow(dead_code)]
pub fn key_value_row<'a, Message>(key: String, value: String) -> Row<'a, Message>
where
    Message: 'a,
{
    let key = label(key).secondary().build();
    let value = label(value).build();
    let mut row = Row::new()
        .push(
            Column::new()
                .width(Length::FillPortion(2))
                .align_x(alignment::Alignment::Start)
                .push(key),
        )
        .push(
            Column::new()
                .width(Length::FillPortion(2))
                .align_x(alignment::Alignment::End)
                .push(value),
        );
    row = row
        .spacing(Sizes::Md as u16)
        .align_y(alignment::Alignment::Center);
    row
}

#[allow(dead_code)]
pub fn space_between<'a, Message>(
    elem_1: Element<'a, Message>,
    elem_2: Element<'a, Message>,
) -> Row<'a, Message>
where
    Message: 'a,
{
    let mut row = Row::new()
        .push(
            Column::new()
                .width(Length::FillPortion(2))
                .align_x(alignment::Alignment::Start)
                .push(elem_1),
        )
        .push(
            Column::new()
                .width(Length::FillPortion(2))
                .align_x(alignment::Alignment::End)
                .push(elem_2),
        );
    row = row.spacing(Sizes::Md).align_y(alignment::Alignment::Center);
    row
}

/// A container for titling and captioning a piece of data.
///
/// Layout:
/// ---------------
/// title
/// Data
/// caption
/// ---------------
///
/// note: Row has their items centered aligned because iced Text elements
/// will save all the space they need for their glyphs. For example,
/// the end of a "p" is allocated, which makes bottom aligned text elements
/// look like they are not on the same bottom, even though their "p" glyphs will
/// be.
pub fn double_labeled_data<'a, Message>(
    data: Text<'a>,
    title: Text<'a>,
    caption: Text<'a>,
) -> Column<'a, Message>
where
    Message: 'a,
{
    Column::new()
        .spacing(Sizes::Sm)
        .push(title)
        .push(data)
        .push(caption)
}

#[allow(dead_code)]
pub fn custom_icon_button<'a>(
    icon: Bootstrap,
    font_size: u16,
) -> iced::widget::Button<'a, view::ViewMessage> {
    let content = text(icon_to_char(icon))
        .font(BOOTSTRAP_FONT)
        .size(font_size)
        .color(Color::WHITE);

    button(content)
}

/// An individual navigation step that can be rendered in a list of steps.
#[derive(Debug, Clone)]
pub struct NavigationStep<Message>
where
    Message: Clone + Default,
{
    pub icon: Bootstrap,
    pub cta: String,
    pub on_press: Message,
    pub active: bool,
    pub disabled: bool,
}

impl<Message> NavigationStep<Message>
where
    Message: Clone + Default,
{
    /// Creates a new navigation step.
    pub fn new(
        icon: Bootstrap,
        cta: &str,
        on_press: Message,
        active: bool,
        disabled: bool,
    ) -> Self {
        Self {
            icon,
            cta: cta.to_string(),
            on_press,
            active,
            disabled,
        }
    }
}

impl Default for NavigationStep<view::ViewMessage> {
    fn default() -> Self {
        Self {
            icon: Bootstrap::Check,
            cta: "Default".to_string(),
            on_press: view::ViewMessage::default(),
            active: false,
            disabled: false,
        }
    }
}

/// Renders a list of navigation steps with a custom icon, label, active flag,
/// and message to emit.
pub fn navigation_steps<'a, Message>(
    title: &str,
    steps: Vec<NavigationStep<Message>>,
) -> Column<'a, Message>
where
    Message: 'a + Clone + Default,
{
    let mut content = Column::new().push(label(title).secondary().build());

    for NavigationStep {
        icon,
        cta,
        on_press,
        active,
        disabled,
    } in steps.into_iter()
    {
        let mut row = Row::new()
            .spacing(Sizes::Sm)
            .align_y(alignment::Alignment::Center);
        if active {
            row = row.push(
                container(Column::new())
                    .width(Length::Fixed(Sizes::Xs.into()))
                    .height(Length::Fixed(Sizes::Xl.into()))
                    .style(|_theme| ExcaliburContainer::indicator().theme()),
            );
        }

        row = row
            .push(iced::widget::text(icon_to_char(icon)).font(BOOTSTRAP_FONT))
            .push(label(&cta).title3().build());

        let bg_color = match active {
            true => SELECTED_CONTAINER_COLOR,
            false => Color::TRANSPARENT,
        };

        let mut row = button(row).padding(Sizes::Sm).width(Length::Fill);

        // Disable the button if it has an empty message.
        if !disabled {
            row = row.on_press(on_press);
        }

        content = content.push(row);
    }

    content.spacing(Sizes::Sm)
}

/// Divides content into four quadrants with centered alignment.
#[allow(dead_code)]
pub fn quadrant_container<'a, Message>(
    top_left: Element<'a, Message>,
    top_right: Element<'a, Message>,
    bottom_left: Element<'a, Message>,
    bottom_right: Element<'a, Message>,
) -> Container<'a, Message>
where
    Message: 'a,
{
    let top_left = Container::new(top_left).center_x(Fill).center_y(Fill);
    let top_right = Container::new(top_right).center_x(Fill).center_y(Fill);
    let bottom_left = Container::new(bottom_left).center_x(Fill).center_y(Fill);
    let bottom_right = Container::new(bottom_right).center_x(Fill).center_y(Fill);

    let mut content = Column::new()
        .push(
            Row::new()
                .height(Length::FillPortion(2))
                .push(top_left.width(Length::FillPortion(2)))
                .push(top_right.width(Length::FillPortion(2)))
                .align_y(alignment::Alignment::Center),
        )
        .push(
            Row::new()
                .height(Length::FillPortion(2))
                .push(bottom_left.width(Length::FillPortion(2)))
                .push(bottom_right.width(Length::FillPortion(2)))
                .align_y(alignment::Alignment::Center),
        )
        .align_x(alignment::Alignment::Center)
        .spacing(Sizes::Md);

    content = content.width(Fill).height(Fill).padding(Sizes::Md);

    container(content)
}
