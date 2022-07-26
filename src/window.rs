use druid::{
    widget::{Button, Container, Controller, Flex, TextBox, Align},
    AppDelegate, Data, Env, Event, EventCtx, Lens, Widget, WidgetExt, WidgetPod, Selector,
};

use crate::todo::{Todo, TodoItem, TodoState};

pub const ADD_NEW_TODO: Selector<String> = Selector::new("Add_New_Todo");
pub const REMOVE_TODO_ITEM: Selector<uuid::Uuid> = Selector::new("Remove_Todo_Item");
pub struct TodoWindow {
    contain: WidgetPod<TodoWindowState, Container<TodoWindowState>>,
}

#[derive(Data, Clone, Lens, PartialEq, Eq)]
pub struct TodoWindowState {
    pub todo: TodoState,
    pub new_text: String,
}

impl Widget<TodoWindowState> for TodoWindow {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut TodoWindowState,
        env: &druid::Env,
    ) {
        self.contain.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &TodoWindowState,
        env: &druid::Env,
    ) {
        self.contain.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &TodoWindowState,
        data: &TodoWindowState,
        env: &druid::Env,
    ) {
        self.contain.update(ctx, data, env)
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &TodoWindowState,
        env: &druid::Env,
    ) -> druid::Size {
        self.contain.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &TodoWindowState, env: &druid::Env) {
        self.contain.paint(ctx, data, env);
    }
}

impl TodoWindow {
    pub fn new() -> Self {
        let btn = Button::new("添加").on_click(
            |ctx: &mut EventCtx, data: &mut TodoWindowState, _env: &Env|{
                ctx.submit_command(crate::window::ADD_NEW_TODO.with(data.new_text.clone()));
                ctx.set_handled();
            },
        )
        .expand_width();
        let mut container = Container::new(
            Flex::column()
                .with_child(
                    Flex::row()
                        .with_flex_child(
                            TextBox::new()
                                .with_placeholder("请输入新内容！")
                                .expand_width()
                                .lens(TodoWindowState::new_text),
                            6.0,
                        )
                        .with_default_spacer()
                        .with_flex_child(btn,1.0)
                        ,
                )
                .with_flex_child(Todo::new().lens(TodoWindowState::todo), 4.0)
                .padding(10.0)
        );
        Self {
            contain: WidgetPod::new(container),
        }
    }
}

impl<T, W> Controller<T,W> for TodoWindow where W: Widget<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
 
    }
}


impl TodoWindowState {
    pub fn new() -> Self {
        Self {
            todo: TodoState::new(),
            new_text: "".into(),
        }
    }
    pub fn from_vec(data: Vec<TodoItem>) -> Self {
        Self {
            todo: TodoState::from_vec(data),
            new_text: "".into(),
        }
    }
}

pub struct WindowDelegate;

impl AppDelegate<TodoWindowState> for WindowDelegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut TodoWindowState,
        env: &Env,
    ) -> druid::Handled {
        if cmd.is(ADD_NEW_TODO) {
            let value = cmd.get(ADD_NEW_TODO).unwrap();
            data.todo.append(value);
            data.new_text = "".into();
            return druid::Handled::Yes;
        }
        else if cmd.is(REMOVE_TODO_ITEM) {
            let id = cmd.get(REMOVE_TODO_ITEM).unwrap();
            data.todo.remove(id);
            return druid::Handled::Yes;
        }



        druid::Handled::No
    }
}
