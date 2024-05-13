use std::vec;


#[derive(Clone)]
pub struct Category {
    pub name: String,
    pub games: Vec<String>
}


pub async fn parse_games_list(text: &str) -> Vec<Category> {
    let mut categories = vec![];

    let lines = text.lines();

    let mut current_category: Option<Category> = None;

    for line in lines.into_iter() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("* ") {
            current_category.clone().unwrap().games.push(line.to_string());
        } else {
            if let Some(category) = current_category {
                categories.push(category);
            }

            current_category = Some(Category {
                name: line.to_string(),
                games: vec![]
            });
        }
    }

    categories.push(current_category.unwrap());

    categories
}


pub async fn add_game(
    mut categories: Vec<Category>,
    category: &str,
    game_line: &str
) -> Vec<Category> {
    let category_number = ["points", "paids", "gifts"]
        .iter()
        .position(|&x| x == category)
        .unwrap();

    categories[category_number].games.push(game_line.to_string());

    categories
}

pub async fn delete_game(
    mut categories: Vec<Category>,
    game_name: &str
) -> Vec<Category> {
    let prefix = format!("* {}", game_name);

    for category in categories.iter_mut() {
        category.games.retain(|game| !game.starts_with(&prefix));
    }

    categories
}


pub async fn format_games_list(categories: Vec<Category>) -> String {
    let mut result = String::new();

    for category in categories.iter() {
        result.push_str(&format!("{}\n", category.name));

        for game in category.games.iter() {
            result.push_str(&format!("* {}\n", game));
        }
        result.push_str("\n\n");
    }

    result
}
