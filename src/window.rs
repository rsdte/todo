use std::ops::Add;

use druid::{
    widget::{Button, Container, Controller, Flex, TextBox},
    AppDelegate, Data, Env, Event, EventCtx, Lens, Widget, WidgetExt, WidgetPod, Selector,
};

use crate::todo::{Todo, TodoItem, TodoState};

pub const ADD_NEW_TODO: Selector<String> = Selector::new("Add_New_Todo");

pub struct TodoWindow {
    contain: WidgetPod<TodoWindowState, Container<TodoWindowState>>,
}

#[derive(Data, Clone, Lens)]
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
        );
        Self {
            contain: WidgetPod::new(Container::new(
                Flex::column()
                    .with_child(
                        Flex::row()
                            .with_flex_child(
                                TextBox::new()
                                    .with_placeholder("请输入新内容！")
                                    .lens(TodoWindowState::new_text),
                                1.0,
                            )
                            .with_child(btn),
                    )
                    .with_flex_child(Todo::new().lens(TodoWindowState::todo), 1.0),
            )),
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

        druid::Handled::No
    }
}
