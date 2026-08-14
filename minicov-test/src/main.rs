fn foo() {
    println!("a");
}
fn bar() {
    println!("b");
}

#[inline(never)]
fn call_indirect(f: fn()) {
    f();
}

fn do_stuff(x: bool) {
    if x {
        foo()
    } else {
        bar()
    }
}

fn main() {
    println!("built with coverage: {}", minicov::coverage_enabled());

    do_stuff(false);
    let f: fn() = if std::hint::black_box(false) {
        foo
    } else {
        bar
    };
    call_indirect(std::hint::black_box(f));

    let mut coverage = vec![];
    unsafe {
        minicov::capture_coverage(&mut coverage).unwrap();
        minicov::reset_coverage();
        minicov::merge_coverage(&coverage).unwrap();
    }
    let mut merged_coverage = vec![];
    unsafe {
        minicov::capture_coverage(&mut merged_coverage).unwrap();
    }
    std::fs::write("output.profraw", merged_coverage).unwrap();
}
