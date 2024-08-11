mod command;

use cursive:: {
    traits::Nameable,
    views::{Dialog, EditView},
    Cursive
};

use command::{Command, CopyCmd, CutCmd, PasteCmd};

#[derive(Default)]
struct AppContext {
    clipboard: String,
    history: Vec<Box<dyn Command>>
}

fn main() {
    let mut app = cursive::default();

    app.set_user_data(AppContext::default());
    app.add_layer(
        Dialog::around(EditView::default().with_name("Editor"))
            .title("Type and use buttons")
            .button("Copy", |s| execute(s, CopyCmd))
            .button("Cut", |s| execute(s, CutCmd::default()))
            .button("Paste", |s| execute(s, PasteCmd::default()))
            .button("Undo", undo)
            .button("Quit", |s| s.quit())
    );

    app.run();
}

fn execute(app: &mut Cursive, mut command: impl Command + 'static) {
    if command.execute(app) {
        app.with_user_data(|context: &mut AppContext| {
            context.history.push(Box::new(command))
        });
    }
}

fn undo(app: &mut Cursive) {
    let mut context = app.take_user_data::<AppContext>().unwrap();
    if let Some(mut command) = context.history.pop() {
        command.undo(app);
    }
    app.set_user_data(context);
}