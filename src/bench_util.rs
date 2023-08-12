/// Ultra mega overcomplicated macro to parse initialization fields of benches
#[macro_export]
macro_rules! setup_single_bench {
    ($name: ident { $($field: ident: $value: expr,)* }) => {
        frand::setup_single_bench! {
            @munch $name { $([$field $value])* }
            {}
        }
    };
    (
        @munch $name: ident { [iteration $iteration: expr] }
        { $([$init_fields: ident $init_values: expr])* }
    ) => {
        #[bench]
        fn $name(bencher: &mut Bencher) {
            $(let mut $init_fields = $init_values;)*
            bencher.iter(|| {
                for _ in 0..ITERATIONS {
                    black_box($iteration);
                }
            });
        }
    };
    (
        @munch $name: ident { [$init_field: ident $init_value: expr] $([$field: ident $value: expr])* }
        { $([$init_fields: ident $init_values: expr])* }
    ) => {
        frand::setup_single_bench! {
            @munch $name { $([$field $value])* }
            { [$init_field $init_value] $([$init_fields $init_values])* }
        }
    };
}

#[macro_export]
macro_rules! setup_benches {
    ($($name: ident { $($field: ident: $value: expr,)* })*) => {
        extern crate test;
        use std::hint::black_box;

        use test::Bencher;

        const ITERATIONS: usize = 100_000;

        $( frand::setup_single_bench! { $name { $($field: $value,)* } } )*
    };
}
