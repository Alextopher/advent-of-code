//! This modules creates a macro `get_input!(directory, day)` that will download the input
//! file if needed, and then read the input file. Returns a `&'static str` containing the contents of the file.

/// This macro will download the input file if needed, and then read the input file.
/// Returns a `&'static str` containing the contents of the file.
///
/// # Arguments
///
/// * `year` - the year of the advent of code challenge
/// * `day` - the day of the advent of code challenge
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};

struct InputArgs {
    year: u32,
    day: u32,
}

impl Parse for InputArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let year: syn::LitInt = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let day: syn::LitInt = input.parse()?;

        Ok(InputArgs {
            year: year.base10_parse()?,
            day: day.base10_parse()?,
        })
    }
}

#[proc_macro]
pub fn input_str(input: TokenStream) -> TokenStream {
    // Parse the input arguments
    let args = syn::parse_macro_input!(input as InputArgs);

    // Construct the input file path
    let input_file = format!("inputs/{}-{:02}.txt", args.year, args.day);
    let input_file = std::path::Path::new(&input_file);

    // Download the input file if it doesn't exist
    if !input_file.exists() {
        // Try to get the session cookie from environment variable first, then from file
        let cookie = get_session_cookie();

        // Download the input
        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            args.year, args.day
        );

        let resp = client
            .get(&url)
            .header("Cookie", format!("session={}", cookie.trim()))
            .send()
            .expect("Failed to send request to Advent of Code");

        // Verify the response status
        if resp.status() != reqwest::StatusCode::OK {
            panic!(
                "Failed to download input for year {} day {}. Status: {}. \
                Make sure your session cookie is valid and you have access to this puzzle.",
                args.year,
                args.day,
                resp.status()
            );
        }

        // Ensure the inputs directory exists
        std::fs::create_dir_all("inputs").expect("Failed to create inputs directory");

        // Write the input file
        let content = resp.text().expect("Failed to read response text");
        std::fs::write(input_file, content).expect("Failed to write input file");
    }

    // Read the input file
    let input = std::fs::read_to_string(input_file).expect("Failed to read input file");

    // Return the input as a raw string literal
    let input = format!("r#\"{}\"#", input);
    input.parse().expect("Failed to parse input as TokenStream")
}

/// Get the session cookie from environment variable or file
fn get_session_cookie() -> String {
    // First, try to get cookie from environment variable
    if let Ok(cookie) = std::env::var("AOC_COOKIE") {
        return cookie;
    }

    // If environment variable is not set, try to read from file
    if let Ok(cookie) = std::fs::read_to_string("inputs/cookie.txt") {
        return cookie;
    }

    // If both fail, provide helpful error message
    panic!(
        "No session cookie found! Please either:\n\
        1. Set the AOC_COOKIE environment variable with your session cookie, or\n\
        2. Create 'inputs/cookie.txt' containing your session cookie.\n\
        \n\
        You can find your session cookie by:\n\
        - Going to https://adventofcode.com\n\
        - Opening browser DevTools (F12)\n\
        - Going to Application/Storage > Cookies\n\
        - Copying the value of the 'session' cookie"
    );
}
