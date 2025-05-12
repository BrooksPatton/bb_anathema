use std::{fs::read_to_string, thread::spawn};

use anathema::{
    prelude::{Backend, Document, ToSourceKind, TuiBackend},
    runtime::Runtime,
};
use storybook::{
    app::{App, AppState},
    articles::{
        article_titles::{ArticleTitles, ArticleTitlesState},
        get_article_titles,
    },
    main_section::{Main, MainState},
    messages::Message,
    stories::article_title::ArticleTitle,
};

fn main() {
    let left_menu_template = read_to_string("templates/left_menu.aml").unwrap();
    let library_menu_template = read_to_string("templates/library_menu.aml").unwrap();
    let article_template = read_to_string("templates/article.aml").unwrap();

    let doc = Document::new("@app [meow: 5]");
    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);

    builder
        .component("main", "templates/main.aml", Main, MainState::new())
        .unwrap();
    let app_id = builder
        .component("app", "templates/app.aml", App, AppState::new().unwrap())
        .unwrap();
    builder.template("left_menu", left_menu_template.to_template());
    builder.template("library_menu", library_menu_template.to_template());
    builder
        .component(
            "article_titles",
            "templates/article_titles.aml",
            ArticleTitles::new(),
            ArticleTitlesState::new(),
        )
        .unwrap();
    builder
        .component(
            "article_title",
            "templates/article_title.aml",
            ArticleTitle,
            (),
        )
        .unwrap();
    builder.template("article", article_template);

    let emitter = builder.emitter();

    let _load_filenames_handle = spawn(move || {
        let article_names = get_article_titles().unwrap();

        emitter
            .emit(app_id, Message::ArticleTitlesLoaded(article_names))
            .unwrap();
    });

    builder.finish(|runtime| runtime.run(&mut backend)).unwrap();
}
