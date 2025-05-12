pub mod article_titles;

use eyre::{Context, Result, bail};
use std::fs::{ReadDir, read_dir, read_to_string};

pub fn get_article_titles() -> Result<Vec<String>> {
    let directory = read_dir("articles").context("reading articles directory")?;
    let mut titles = vec![];

    for dir_object in directory {
        let Ok(object) = dir_object else {
            bail!("could not read directory object")
        };
        let Ok(title) = object.file_name().into_string() else {
            continue;
        };

        titles.push(title);
    }

    Ok(titles)
}

pub fn get_article_content(name: &str) -> Result<Option<String>> {
    println!("reading article content for article {name}");
    let directory =
        get_articles_directory().context("getting articles directory in get_article_content")?;

    for dir_entry in directory {
        let Ok(object) = dir_entry else { continue };
        let Ok(title) = object.file_name().into_string() else {
            continue;
        };

        if title == name {
            let path = object.path();
            let content = read_to_string(path).context("reading article content")?;

            println!("found article content");
            return Ok(Some(content));
        }
    }

    println!("didn't find article content");
    Ok(None)
}

pub fn get_articles_directory() -> Result<ReadDir> {
    println!("getting articles directory");
    read_dir("articles").context("reading articles directory")
}
