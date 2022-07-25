use std::clone;

use druid::{im::Vector, widget::{Flex, List, Checkbox, Label}, Widget, WidgetPod, Data, WidgetExt, Lens};

#[derive(Data, Clone, Lens)]
pub struct TodoItem {
    pub text: String,
    pub done: bool,
}

impl TodoItem {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            done: false,
        }
    }
}

pub struct Todo {
    container: WidgetPod<TodoState, Flex<TodoState>>,
}

#[derive(Data, Clone,Lens)]
pub struct TodoState {
    pub items: Vector<TodoItem>,
}

impl Widget<TodoState> for Todo {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut TodoState,
        env: &druid::Env,
    ) {
        self.container.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &TodoState,
        env: &druid::Env,
    ) {
        self.container.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &TodoState,
        data: &TodoState,
        env: &druid::Env,
    ) {
        self.container.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &TodoState,
        env: &druid::Env,
    ) -> druid::Size {
        self.container.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &TodoState, env: &druid::Env) {
        self.container.paint(ctx, data, env);
    }
}


impl Todo {
    pub fn new() -> Self {
        let mut flex = Flex::row();
        flex.add_child(List::new(Self::build_item).lens(TodoState::items));
        Self {
            container: WidgetPod::new(flex)
        }
    }

    fn build_item() -> Flex<TodoItem> {
        let checkbox = Checkbox::new("")
            .lens(TodoItem::done);
        let label = Label::raw().lens(TodoItem::text);
        Flex::row().with_child(checkbox).with_flex_child(label,1.0)
    }
}

impl TodoState {
    pub fn new() -> Self {
        Self { items:  Vector::new() }
    }

    pub fn from_vec(data: Vec<TodoItem>) -> Self {
        Self {
            items: Vector::from(data)
        }
    }

    pub fn append(&mut self, text: &str) {
        self.items.push_back(TodoItem::new(text));
    }
}