// TODO: implementing the following typeclasses
// - Monads
// - Alternatives
// - Functors
// - Applicative

// TODO: ???
pub trait Functor<A, B>{

    fn fmap(f: &Fn(A)->B) -> Self;
}


// pub trait App
// pub trait Alt
// pub trait Monad
// pub trait Kleisli??