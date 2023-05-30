extern crate random_choice;
use self::random_choice::random_choice;

fn main() {
    let samples = vec!["hi", "this", "is", "a", "test!"];
    let weights: Vec<f64> = vec![5.6, 7.8, 9.7, 1.1, 2.0];

    let number_choices = 100;
    let choices = random_choice().random_choice_f64(&samples, &weights, number_choices);

    for choice in choices {
        println!("{}, ", choice);
    }
}
