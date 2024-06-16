pub mod node;
pub use node::*;
#[macro_use]
pub mod rule;

#[cfg(test)]
mod tests {

    #[test]
    fn parse_expression() {
        let identifier = seq!("identifier" =>
        sor!(
            char!('_')
        )
        );
    }
}
