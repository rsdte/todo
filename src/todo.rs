use druid::{
    im::Vector,
    widget::{Button, Checkbox, Flex, Label, List, Container, Padding},
    Command, Data, Env, EventCtx, Lens, Widget, WidgetExt, WidgetPod, Insets,
};

use crate::window;

#[derive(Data, Clone, Lens, PartialEq, Eq)]
pub struct TodoItem {
    pub text: String,
    pub done: bool,
    #[data(same_fn = "PartialEq::eq")]
    id: uuid::Uuid,
}

impl TodoItem {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            done: false,
            id: uuid::Uuid::new_v4(),
        }
    }
}

pub struct Todo {
    container: WidgetPod<TodoState, Flex<TodoState>>,
}

#[derive(Data, Clone, Lens, PartialEq, Eq)]
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
            container: WidgetPod::new(flex),
        }
    }

    fn build_item() -> Container<TodoItem> {
        let checkbox = Checkbox::new("").lens(TodoItem::done);
        let label = Label::raw().lens(TodoItem::text);
        let delete_button = Button::new("delete").on_click(
            |ctx: &mut EventCtx, data: &mut TodoItem, _env: &Env| {
                ctx.submit_command(window::REMOVE_TODO_ITEM.with(data.id));
            },
        );
        Container::new(Padding::new(Insets::new(0.,5.,0.,5.), Flex::row()
        .with_flex_child(checkbox,1.0)
        .with_flex_child(label,1.0)
        .with_flex_child(delete_button.fix_size(85., 25.), 2.0)
        ))
    }
}

impl TodoState {
    pub fn new() -> Self {
        Self {
            items: Vector::new(),
        }
    }

    pub fn from_vec(data: Vec<TodoItem>) -> Self {
        Self {
            items: Vector::from(data),
        }
    }

    pub fn append(&mut self, text: &str) {
        self.items.push_back(TodoItem::new(text));
    }

    pub fn remove(&mut self, item_id: &uuid::Uuid) {
        let item = self.items.iter().find(|&p| p.id == *item_id);
        if let Some(t) = item {
            let target = self.items.index_of(t);
            self.items.remove(target.unwrap());
        }
    }
}
