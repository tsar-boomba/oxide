use std::sync::Arc;

use iced::{
    color,
    widget::{column, container, scrollable},
    Command, Element, Length,
};

#[derive(Debug)]
pub struct ScrollableList<A: iced::Application> {
    items: Vec<ListItem<A>>,
    selected: usize,
    id: scrollable::Id,
}

pub struct ListItem<A: iced::Application> {
    children: Arc<dyn for<'a> Fn(&'a A) -> Element<'static, A::Message> + Send + Sync>,
    action: Arc<dyn Fn(&mut A, A::Message) -> Command<A::Message> + Send + Sync>,
}

pub enum Message {
    Up,
    Down,
    Other,
}

impl<A: iced::Application> ListItem<A> {
    pub fn new(
        children: impl for<'a> Fn(&'a A) -> Element<'static, A::Message> + Send + Sync + 'static,
        action: impl Fn(&mut A, A::Message) -> Command<A::Message> + Send + Sync + 'static,
    ) -> Self {
        Self {
            children: Arc::new(children),
            action: Arc::new(action),
        }
    }
}

impl<A: iced::Application> std::fmt::Debug for ListItem<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListItem").finish()
    }
}

impl<A: iced::Application> ScrollableList<A>
where
    A::Message: 'static,
{
    pub fn new(items: Vec<ListItem<A>>) -> Self {
        Self {
            items,
            selected: 0,
            id: scrollable::Id::unique(),
        }
    }

    /// Handle message
    pub fn update(
        &mut self,
        app: &mut A,
        og_message: A::Message,
        mapped_message: Message,
    ) -> Command<A::Message> {
        if self.items.len() == 0 {
            return Command::none();
        }

        match &mapped_message {
            Message::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                } else {
                    self.selected = self.items.len() - 1;
                }
                Command::none()
            }
            Message::Down => {
                if self.selected < self.items.len() - 1 {
                    self.selected += 1;
                } else {
                    self.selected = 0;
                }
                Command::none()
            }
            Message::Other => {
                let ListItem { action, .. } = &self.items[self.selected];
                (action)(app, og_message)
            }
        }
    }

    pub fn view(&self, app: &A) -> Element<'static, A::Message> {
        let Self {
            id,
            selected,
            items,
        } = self;
        let selected = selected.clone();

        scrollable(column(
            items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    container((item.children)(app))
                        .padding(8)
                        .width(Length::Fill)
                        .style(move |_theme: &'_ iced::Theme| container::Appearance {
                            background: if selected == i {
                                Some(iced::Background::Color(color!(255, 0, 0)))
                            } else {
                                None
                            },
                            ..Default::default()
                        })
                        .into()
                })
                .collect(),
        ))
        .id(id.clone())
        .into()
    }
}

impl<A: iced::Application> Clone for ScrollableList<A> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            items: self.items.clone(),
            selected: self.selected.clone(),
        }
    }
}

impl<A: iced::Application> Clone for ListItem<A> {
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
            action: self.action.clone(),
        }
    }
}
