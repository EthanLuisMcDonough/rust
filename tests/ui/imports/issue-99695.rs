//@ run-rustfix
//@ revisions: edition_2015 edition_2018
//@ [edition_2015] edition: 2015
//@ [edition_2018] edition: 2018

#![allow(unused, nonstandard_style)]
mod m {
    #[macro_export]
    macro_rules! nu {
        {} => {};
    }

    pub struct other_item;

    pub use self::{nu, other_item as _};
    //~^ ERROR unresolved import `self::nu` [E0432]
    //~| HELP a macro with this name exists at the root of the crate
}

fn main() {}
