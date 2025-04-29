use super::util::draw_sep;
use crate::r#impl::Calculator;
use ansiterm::Colour::{BrightRed, White, BrightYellow};


/// Runs a calculator as a TUI, as a sort of default runner
/// 
/// This function takes a calculator as input, and then executes it as a TUI.
/// However, it doesn't handle anything more than running the calculator, and,
/// for example, won't show a selector.
/// 
/// Example:
/// ```ignore
/// run_calc(&DemoCalc);
/// ```
pub fn run_calc(calc: &dyn Calculator){
    // Get calculator title and description
    let info = calc.get_info();

    // Draw title and description
    draw_sep(80);
    println!("{: ^1$}", info.title, 80);
    draw_sep(80);
    println!("{}",info.description);

    // Get the arguments that need to be queried
    let args = calc.get_signature();

    // Get inputs
    let args = args.iter().map(|x|{
        let mut result = x.query_arg();

        // If it can't parse it, rerun, stating that the input was invalid.
        while result.is_err() {
            println!("{}",White.dimmed().paint("Invalid input."));
            result = x.query_arg();
        }

        result.unwrap()
    }).collect();

    // Run actual calculation
    let results = calc.calc(args);

    // Print errors, warnings, and outputs
    match results{
        Ok(x) => {
            // If there are no warnings, don't insert extra spaces
            if !x.0.is_empty(){
                println!();
                for i in x.1 {
                    println!("{}", BrightYellow.paint(format!("⚠  {}",i)));
                }
            }

            // Print each value
            for i in x.0 {
                println!("{}", i);
            }
        },
        Err(x) =>{
            // Print errors
            print!("{}", BrightRed.paint(x));
        }
    }  
}