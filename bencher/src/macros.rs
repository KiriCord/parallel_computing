#[macro_export]
macro_rules! generate_bench {
    ($repeat_count:expr, $thread_counts:expr, $bench_fn:ident, $($rest:tt)*) => {{
        let single_threaded_result = bencher::bench_result::BenchResultSingleThreaded::bench_rayon(
            move || {
                std::hint::black_box($bench_fn($($rest)*));
            },
            $repeat_count,
        );

        println!("{}\n", single_threaded_result);
        for thread_count in $thread_counts {
            let bench_result = bencher::bench_result::BenchResultMultiThreaded::bench_rayon(
                move || {
                    std::hint::black_box($bench_fn($($rest)*));
                },
                thread_count,
                $repeat_count,
                &single_threaded_result,
            );
            println!("{}\n", bench_result);
        }
    }};
}
