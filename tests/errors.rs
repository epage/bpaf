use bpaf::*;

#[test]
fn this_or_that_odd() {
    let a = short('a').req_flag(());
    let b = short('b').req_flag(());
    let ab = construct!(a, b);
    let a = short('a').req_flag(());
    let c = short('c').req_flag(());
    let cd = construct!(a, c);
    let parser = construct!([ab, cd]).to_options();

    let res = parser
        .run_inner(&["-a", "-b", "-c"])
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(res, "`-c` cannot be used at the same time as `-b`");
}

#[test]
fn no_argument() {
    let a = short('a').argument::<i32>("N");
    let b = short('2').switch();
    let parser = construct!(a, b).to_options();

    let r = parser.run_inner(&["-a", "-42"]).unwrap();
    assert_eq!(r, (-42, false));

    //    let r = parser.run_inner(&["-a", "-4"])).unwrap();
    //    assert_eq!(r, (-4, flse));
    let r = parser.run_inner(&["-a", "-2"]).unwrap_err().unwrap_stderr();
    assert_eq!(
        r,
        "`-a` requires an argument `N`, got a flag `-2`, try `-a=-2` to use it as an argument"
    );
}

#[test]
fn cannot_be_used_partial_arg() {
    let a = short('a').req_flag(10);
    let b = short('b').argument::<usize>("ARG");
    let parser = construct!([a, b]).to_options();

    // TODO - error message can be improved...
    let res = parser.run_inner(&["-b", "-a"]).unwrap_err().unwrap_stderr();
    assert_eq!(res, "`-b` is not expected in this context");

    let res = parser.run_inner(&["-a", "-b"]).unwrap_err().unwrap_stderr();
    assert_eq!(res, "`-b` is not expected in this context");
}

#[test]
fn better_error_with_enum() {
    #[derive(Debug, Clone, Bpaf)]
    #[bpaf(options)]
    enum Foo {
        Alpha,
        Beta,
        Gamma,
    }

    let res = foo()
        .run_inner(&["--alpha", "--beta"])
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(res, "`--beta` cannot be used at the same time as `--alpha`");

    let res = foo()
        .run_inner(&["--alpha", "--gamma"])
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        res,
        "`--gamma` cannot be used at the same time as `--alpha`"
    );

    let res = foo()
        .run_inner(&["--beta", "--gamma"])
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(res, "`--gamma` cannot be used at the same time as `--beta`");

    let res = foo()
        .run_inner(&["--alpha", "--beta", "--gamma"])
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(res, "`--beta` cannot be used at the same time as `--alpha`");
}

#[test]
fn guard_message() {
    let parser = short('a')
        .argument::<u32>("N")
        .guard(|n| *n <= 10u32, "too high")
        .to_options();

    let res = parser.run_inner(&["-a", "30"]).unwrap_err().unwrap_stderr();

    assert_eq!(res, "`30`: too high");
}

#[test]
fn strict_positional_argument() {
    let a = short('a').argument::<usize>("N");
    let parser = a.to_options();

    let r = parser
        .run_inner(&["-a", "--", "10"])
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "`-a` requires an argument `N`");
}

#[test]
fn not_expected_at_all() {
    let a = short('a').switch();
    let parser = a.to_options();

    let r = parser
        .run_inner(&["--megapotato"])
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "`--megapotato` is not expected in this context");

    let r = parser
        .run_inner(&["megapotato"])
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "`megapotato` is not expected in this context");
}

#[test]
fn cannot_be_used_twice() {
    let a = short('a').switch();
    let b = short('b').switch().many();
    let parser = construct!(a, b).to_options();

    let r = parser
        .run_inner(&["-a", "-b", "-a"])
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        r,
        "argument `-a` cannot be used multiple times in this context"
    );

    let r = parser.run_inner(&["-a", "-a"]).unwrap_err().unwrap_stderr();
    assert_eq!(
        r,
        "argument `-a` cannot be used multiple times in this context"
    );

    let r = parser.run_inner(&["-abba"]).unwrap_err().unwrap_stderr();
    assert_eq!(
        r,
        "argument `-a` cannot be used multiple times in this context"
    );
}

#[test]
fn should_not_split_adjacent_options() {
    let a1 = short('a').req_flag(0);
    let a2 = short('a').argument::<usize>("x");
    let a = construct!([a2, a1]);
    let c = pure(()).to_options().command("hello");
    let parser = construct!(a, c).to_options();

    let r = parser.run_inner(&["-a=hello"]).unwrap_err().unwrap_stderr();
    assert_eq!(
        r,
        "expected `COMMAND ...`, got `hello`. Pass `--help` for usage information"
    );

    let r = parser.run_inner(&["-ahello"]).unwrap_err().unwrap_stderr();
    assert_eq!(
        r,
        "expected `COMMAND ...`, got `hello`. Pass `--help` for usage information"
    );

    // this one is okay
    let r = parser.run_inner(&["-a", "hello"]).unwrap();
    assert_eq!(r, (0, ()));
}

#[test]
fn adjacent_option_complains_to() {
    let parser = short('a').argument::<usize>("A").to_options();

    let r = parser.run_inner(&["-ayam"]).unwrap_err().unwrap_stderr();

    // TODO - this should point to the whole "-ayam" thing
    assert_eq!(r, "couldn't parse `yam`: invalid digit found in string");
}
