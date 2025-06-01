#[macro_use]
mod macros;
mod ast;
mod tests;
mod exblackjack;
mod expoker;
mod memory;

fn main() {
   exblackjack::run();
   // expoker::run();
}
