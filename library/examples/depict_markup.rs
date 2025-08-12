use {anstream::println, kutil::cli::depict::*};

pub fn main() {
    let markup = "This is a test of |symbol|depiction| |error|markup|. Escape: \\|pipes\\|.";

    println!("Styled:");
    DEFAULT_THEME.print_depiction_markup(markup);
    println!();

    println!("Remove markup:");
    print_depiction_markup(markup, None);
    println!();
}
