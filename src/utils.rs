#[derive(Clone)]
pub struct Category {
    pub name: String,
    pub games: Vec<String>
}


pub async fn parse_games_list(text: &str) -> Vec<Category> {
    let mut categories = vec![];

    for line in text.lines() {
        if line.is_empty() {
            continue;
        }

        if !line.starts_with("* ") {
            let category_name = line;
            let category = Category {
                name: category_name.to_string(),
                games: vec![]
            };

            categories.push(category);
        } else {
            let game_line = line.trim();
            let last_category = categories.last_mut().unwrap();
            last_category.games.push(game_line.to_string());
        }
    }

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
    for category in categories.iter_mut() {
        category.games.retain(|game| !game.starts_with(game_name));
    }

    categories
}


pub async fn format_games_list(categories: Vec<Category>) -> String {
    let mut result = String::new();

    for category in categories.iter() {
        result.push_str(&format!("{}\n", category.name));

        for game in category.games.iter() {
            result.push_str(&format!("{}\n", game));
        }
        result.push_str("\n\n");
    }

    result
}
