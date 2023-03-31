use std::{slice, marker::PhantomData, mem};
use eclair::Has;
use eclair::alloc::Variable;
use eclair::alloc::mode::Public;
use eclair::alloc::mode::Secret;
use eclair::bool::Assert;
use eclair::num;
use eclair::alloc;
use eclair::num::{AssertWithinBitRange, UnsignedInteger};
use eclair::cmp::PartialEq;
use eclair::ops::HasAdd;

fn main() {
    println!("Hello, world!");
    // TODO: Make this compile!
    // let j = <() as Variable<Secret, ()>>::new_unknown(&mut ());
    // let k: <() as Variable<Secret, ()>>::Type;
    // check_eq(j, k, &mut ())
}

fn check_eq<L, R, COM>(leaf: L, root: R, compiler: &mut COM)
where
    L: Variable<Secret, COM> + PartialEq<R, COM> + HasAdd<R>,
    R: Variable<Public, COM>,
    COM: Has<bool> + Assert {
        let res = leaf.eq(&root, compiler);
        compiler.assert(&res);
        // hash `leaf` with its sibling
        // hash the result with the next sibling in `path`
        // ...
        // assert that final result equals `root`
}