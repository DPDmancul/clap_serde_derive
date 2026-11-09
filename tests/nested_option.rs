use clap_serde_derive::ClapSerde;

#[derive(ClapSerde)]
struct A {
    #[clap_serde]
    #[command(flatten)]
    b: B,
}

#[derive(ClapSerde)]
struct B {
    #[clap_serde]
    #[command(flatten)]
    c: C,

    #[arg(long)]
    b_arg: String,
}

#[derive(ClapSerde)]
struct C {
    #[clap_serde]
    #[command(flatten)]
    d: D,

    #[arg(long)]
    c_arg: String,

    #[clap_serde]
    #[command(flatten)]
    e: E,
}

#[derive(ClapSerde)]
struct D {
    #[arg(long)]
    d_arg: Option<String>,

    #[arg(long)]
    d_arg2: Option<String>,
}

#[derive(ClapSerde)]
struct E {
    #[arg(long)]
    e_arg: u32,
}

#[test]
fn nested_option() {
    let a = A::from(serde_yaml::from_str::<<A as ClapSerde>::Opt>(r#"
        b:
          c:
            c_arg: no
            d:
              d_arg: no
              d_arg2: yes
    "#).unwrap()).merge_clap_with_args(&["test", "--b-arg", "b", "--c-arg", "c", "--d-arg", "d", "--e-arg", "1"]);
    assert_eq!("b", a.b.b_arg);
    assert_eq!("c", a.b.c.c_arg);
    assert_eq!(Some("d".to_owned()), a.b.c.d.d_arg);
    assert_eq!(Some("yes".to_owned()), a.b.c.d.d_arg2);
    assert_eq!(1, a.b.c.e.e_arg);
}