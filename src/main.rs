mod data;

use crate::data::card::{Card, CardBuilder};
use csv::Writer;
use pdf_extract::extract_text;
use std::env::current_exe;
use std::error::Error;
use std::path::{Path, PathBuf};

fn read_pdf() -> String {
    let csv_path: PathBuf = read_pdf_path();
    println!("Reading file in path [{:?}]", &csv_path);

    let _pdf: String = match extract_text(csv_path) {
        Ok(pdf_output) => pdf_output,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    return _pdf;
}

fn root_path() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = match current_exe() {
        Ok(path_buf) => path_buf,
        Err(error) => return Err(error.into()),
    };

    let parent = exe_path
        .parent()
        .ok_or("Unable to identify the current root folder.");

    Ok(parent?.to_path_buf())
}

fn read_pdf_path() -> PathBuf {
    let root_path = root_path().unwrap();
    Path::join(&root_path, "order.pdf")
}

fn parse_pdf(file_text: &str) -> Vec<Card> {
    let pdf_2 = file_text.split("\n");
    let mut is_start_point = false;
    let mut line_counter: i8 = 1;
    let mut cards_builder: Vec<Card> = vec![];

    let mut card_builder: CardBuilder = CardBuilder::new();

    for pdf_line in pdf_2 {
        if is_start_point {
            match line_counter {
                1 | 3 | 5 | 8 => {
                    line_counter += 1;
                    continue;
                }
                2 => {
                    card_builder = card_builder.clone().name(pdf_line.clone());
                }
                4 => {
                    card_builder = card_builder.clone().edition(pdf_line.clone());
                }
                6 => {
                    card_builder = card_builder.clone().vendor(pdf_line.clone());
                }
                7 => {
                    card_builder = card_builder.clone().rarity(pdf_line.clone());
                }
                9 => {
                    let final_line = parse_final_line(&pdf_line);
                    if &final_line.1 == &-999.0 || &final_line.2 == &-999 {
                        line_counter = 1;
                        is_start_point = false;
                        card_builder = CardBuilder::new();
                        continue;
                    }
                    card_builder = card_builder.clone().condition(&final_line.0.clone());
                    card_builder = card_builder.clone().price(final_line.1.clone());
                    card_builder = card_builder.clone().quantity(final_line.2.clone());
                }
                _ => {}
            };

            if line_counter >= 9 {
                let card: Card = card_builder.clone().build();
                cards_builder.push(card);
                card_builder = CardBuilder::new();
                line_counter = 1;
            } else {
                line_counter += 1;
            }
        }

        if pdf_line.contains(&"ITEMS DET") {
            is_start_point = true;
        } else if pdf_line.contains(&"C o m m u n i t y   f o r  A l l") {
            is_start_point = false;
        }
    }

    cards_builder
}

fn parse_final_line(line: &str) -> (String, f32, i32) {
    let mut condition = String::from("");
    let price: f32;
    let quantity: i32;

    let line_chunks: Vec<&str> = line.split(" ").collect();

    let price_index = if &line_chunks.len() > &2 {
        &line_chunks.len() - 2
    } else {
        0
    };

    let quantity_index = if &line_chunks.len() > &2 {
        &line_chunks.len() - 1
    } else {
        0
    };

    let price_str = line_chunks
        .get(price_index)
        .unwrap_or_else(|| &"-999.0")
        .to_string()
        .replace("$", "");

    let quantity_str = line_chunks.get(quantity_index).unwrap_or_else(|| &"-999");

    price = price_str.parse::<f32>().unwrap_or_else(|_| -999.0);
    quantity = quantity_str.parse::<i32>().unwrap_or_else(|_| -999);

    for index in 0..price_index {
        condition = condition + " " + line_chunks.get(index).unwrap_or_else(|| &"");
    }

    (condition, price, quantity)
}

fn write_csv(cards: &Vec<Card>) -> Result<(), Box<dyn Error>> {
    let csv_path = write_csv_path();
    println!("Writing csv file in path [{}]", csv_path.display());

    let mut wtr = Writer::from_path(csv_path)?;

    wtr.write_record(&["Card", "Rarity", "Condition", "Price", "Quantity", "Total"])?;

    for card in cards {
        let card_name =
            card.clone().name + "\n" + &card.edition.trim() + "\n" + &card.vendor.trim();

        let total: f32 = card.price * card.quantity as f32;
        let rounded_price = format!("{:.2}", card.price);
        let rounded_total = format!("{:.2}", total);

        match wtr.write_record(&[
            card_name,
            card.rarity.trim().to_string(),
            card.condition.trim().to_string(),
            rounded_price,
            card.quantity.to_string(),
            rounded_total,
        ]) {
            Ok(_) => {}
            Err(error) => panic!("[write_record] Error: {:?}", error),
        }
    }

    wtr.flush()?;

    Ok(())
}

fn write_csv_path() -> PathBuf {
    let root_path = root_path().unwrap();
    Path::join(&root_path, "order.csv")
}

fn main() {
    let file_text = read_pdf();
    let cards = parse_pdf(&file_text);
    match write_csv(&cards) {
        Ok(_) => println!("Process was completed"),
        Err(err) => panic!("[write_csv] Error: {:?}", err),
    };
}
