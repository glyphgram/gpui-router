use gpui::{
    App, AppContext, Context, InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, Styled, Window,
    WindowOptions, div, rgb, white,
};
use gpui_router::{NavLink, Route, Routes, use_params};

struct Basic {}

impl Render for Basic {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .size_full()
            .p_2()
            .bg(rgb(0x2e7d32))
            .text_color(white())
            .child(div().text_xl().child("Basic Example With Params"))
            .child(nav())
            .child(Routes::new().basename("/").children(vec![
                Route::new().index().element(|_, _| home()),
                Route::new().path("user").element(|_, _| user_list()),
                Route::new().path("user/{id}").element(|_, _| User {}),
                Route::new().path("{*not_match}").element(|_, _| not_match()),
            ]))
    }
}

fn nav() -> impl IntoElement {
    div()
        .flex()
        .gap_4()
        .text_lg()
        .child(NavLink::new().to("/").child(div().child("Home")))
        .child(NavLink::new().to("/user").child(div().child("Users")))
        .child(NavLink::new().to("/nothing-here").child(div().child("Not Match")))
}

fn home() -> impl IntoElement {
    div().child("Home")
}

fn user_list() -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(NavLink::new().to("/user/1").child(div().child("User1")))
        .child(NavLink::new().to("/user/2").child(div().child("User2")))
        .child(NavLink::new().to("/user/3").child(div().child("User3")))
}

#[derive(IntoElement)]
pub struct User {}

impl RenderOnce for User {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        div().child(format!("User: {}", params.get("id").unwrap()))
    }
}

fn not_match() -> impl IntoElement {
    div().id("not_match").child(div().child("Nothing to see here!")).child(
        NavLink::new()
            .to("/")
            .child(div().text_decoration_1().child("Go to the home page")),
    )
}

fn main() {
    let application = gpui_platform::application();

    application.run(|cx: &mut App| {
        gpui_router::init(cx);

        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        cx.activate(true);
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_cx| Basic {}))
            .unwrap();
    });
}
