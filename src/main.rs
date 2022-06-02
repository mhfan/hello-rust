
use std::{env, fmt, io::{self, Write}, cmp::Ordering, rc::Rc/*, error::Error*/};
//pub use A::B:C as D;

//#[allow(unused_macros)]
//macro_rules! var_args { ($($args:expr),*) => {{ }} }  //$(f($args);)*   // XXX
//macro_rules! printvar { ($var:expr) => { println!("{}: {:?}", stringify!($var), $var); } }

// src/main.rs (default application entry point)
fn main()/* -> Result<(), Box<dyn Error>>*/ {
    print!("{} v{}, args:", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    env::args().skip(1).for_each(|it| print!(" {it:?}") );
    //println!(" {:?}", env::args().collect::<Vec<String>>());

    //env::var("CASE_INSENSITIVE").is_err();   //option_env!("ENV_VAR_NAME");

    println!("\nHello, world!\n");  //panic!("Test a panic.");

    //use std::time::Duration;
    //std::thread::sleep(Duration::from_secs(1));

    //let x: Result<u32, &str> = Err("Emergency Failure");  //x.expect("Testing expect");

    //let _a = [1, 2, 3, 4, 5];
    //let _a = [1; 5]; //_a.len();
    //for i in _a { println!("{i:?}"); }
    //for i in (1..5).rev() { println!("{i:?}"); }

    compute_24();
    guess_number();
    //_calc_pi();

    //Ok(())
}


#[allow(dead_code)]
//#[allow(clippy::logic_bug)]
fn  compute_24_algo<ST: AsRef<str>, T: Iterator<Item = ST> +
        fmt::Debug>(goal: i32, nums: T) /*-> Result<(), std::error::Error>*/ {
    //#[derive(Clone, Debug)]
    //struct Rational(i32, i32);
    //struct Rational { n: i32, d: i32 }

    //enum Value { Void, Valid, R(Rational) }
    //type Value = Option<Rational>;

    type Rational = (i32, i32);
    type Oper = char;

    #[derive(Debug)]
    struct Expr { v: Rational, e: Option<(Rc<Expr>, Oper, Rc<Expr>)> }

    // TODO: Zero, One, Rule, Sum, Product, Star, Cross, ...

    impl Expr {
        fn new(a: &Rc<Expr>, op: Oper, b: &Rc<Expr>) -> Self {
            Self { v: Expr::operate(a, op, b),
                   e: Some((Rc::clone(a), op, Rc::clone(b))) }
        }

        fn from(num: i32) -> Self { Self { v: /*Rational*/(num, 1), e: None } }
        fn operate(a: &Expr, op: Oper, b: &Expr) -> Rational {
            let mut val = a.v;  // just for initialize val

            match op {
                '+' => {
                    val.0 = a.v.0 * b.v.1 + a.v.1 * b.v.0;
                    val.1 = a.v.1 * b.v.1;
                }
                '-' => {
                    val.0 = a.v.0 * b.v.1 - a.v.1 * b.v.0;
                    val.1 = a.v.1 * b.v.1;
                }
                '*' => {
                    val.0 = a.v.0 * b.v.0;
                    val.1 = a.v.1 * b.v.1;
                }
                '/' =>   if b.v.1 != 0 {
                    val.0 = a.v.0 * b.v.1;
                    val.1 = a.v.1 * b.v.0;
                } else { val.1 = 0; }  // invalidation

                _ => unimplemented!("operator '{}'", op)
            }

            if  val.1 != 0 && val.1 != 1 && val.0 % val.1 == 0 {
                val.0 /= val.1;   val.1  = 1;
            }   val
        }

        fn eqn(&self, n: i32) -> bool { self.v.1 != 0 && self.v.0 == self.v.1 * n }
    }

    impl fmt::Display for Expr {   // XXX: how to reuse for Debug?
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if let Some((a, op, b)) = &self.e {
                let (mut ls, mut rs) = (a.to_string(), b.to_string());
                let lop = op;

                if let Some((_, op, ..)) = &a.e {
                    let braket = matches!(lop, '*' | '/') && matches!(op, '+' | '-');
                    if  braket { ls = format!(r"({})", ls); }
                }

                if let Some((_, op, .. )) = &b.e {
                    let braket = matches!(lop, '/') && matches!(op, '*' | '/') ||
                                      !matches!(lop, '+') && matches!(op, '+' | '-');
                    if  braket { rs = format!(r"({})", rs); }
                }

                write!(f, r"{}{}{}", ls, lop, rs)
            } else { //write!(f, r"{}", self.v)
                let val = &self.v;
                if  val.1 == 0 { write!(f, r"(INV)") } else {
                    let bracket = val.0 * val.1 < 0;
                    write!(f, r"{}{}{}{}", if bracket { r"(" } else { r"" },   val.0,
                        if val.1 == 1 { String::new() } else { format!(r"/{}", val.1) },
                                           if bracket { r")" } else { r"" })
                }
            }
        }
    }

    impl std::cmp::Eq for Expr { }
    impl std::cmp::PartialEq for Expr {
        fn eq(&self, r: &Expr) -> bool {
            self.v.1 != 0 && r.v.1 != 0 && self.v.0 * r.v.1 == self.v.1 * r.v.0
        }
    }

    impl std::cmp::Ord for Expr {
        fn cmp(&self, r: &Self) -> Ordering {
            let (a, b) = (self.v.0 * r.v.1, self.v.1 * r.v.0);
            a.cmp(&b)
        }
    }

    impl std::cmp::PartialOrd for Expr {
        fn partial_cmp(&self, r: &Self) -> Option<Ordering> {
            if self.v.1 == 0 || r.v.1 == 0 { None } else {
                let (a, b) = (self.v.0 * r.v.1, self.v.1 * r.v.0);
                a.partial_cmp(&b)
            }
        }
    }

    impl std::hash::Hash for Expr {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            if let Some((a, op, b)) = &self.e {
                a.hash(state);  b.hash(state);  op.hash(state);
            } else { // XXX:
                self.v.0.hash(state);     self.v.1.hash(state);
            }
        }
    }

    const OPS: [Oper; 4] = [ '+', '-', '*', '/' ];
    let nums: Vec<_> = nums.map(|str| str.as_ref().parse::<i32>())
            .inspect(|res| if let Err(e) = res { eprintln!("Error parsing data: {e}")})
            .filter_map(Result::ok).collect();
    //nums.sort_unstable_by(/* descending */|a, b| b.cmp(a));

    let mut exps: Vec<Rc<Expr>> = Vec::new();
    compute_24_recursive(goal, &nums.iter()
        .map(|n| Rc::new(Expr::from(*n))).collect::<Vec<_>>(), &mut exps);
    exps.iter().for_each(|e| println!("{}", e));
    eprintln!("Got {} results!", exps.len());

    fn compute_24_recursive(goal: i32, nv: &[Rc<Expr>], exps: &mut Vec<Rc<Expr>>) {
        if nv.len() == 1 { if nv[0].eqn(goal) { exps.push(nv[0].clone()); } return }

        use std::collections::HashSet;
        let mut hs = HashSet::new();
        nv.iter().enumerate().for_each(|(i, a)|
            nv.iter().skip(i+1).enumerate().for_each(|(j, b)| {
                let (a, b) = if a < b { (a, b) } else { (b, a) };
                if hs.insert((a, b)) {
                    let j = i + 1 + j;
                    let nv: Vec<_> = nv.iter().enumerate().filter_map(|(k, e)|
                        if k != i && k != j { Some(e.clone()) } else { None }).collect();

                    //eprintln!("-> ({} ? {})", a.val, b.val);
                    OPS.iter().for_each(|op| {
                        if let Some((_, aop, ..)) = &a.e {
                            if aop == op { return; }
                        }

                        if let Some((ba, bop, ..)) = &b.e {
                            match (op, bop) {
                                ('+', '+') | ('*', '*') => if ba < a { return }
                                ('-', '-') | ('/', '/') => return,
                                _ => ()
                            }
                        }

                        if matches!(op, '-' | '/') {    // for order mattered operators
                            let mut nv = nv.to_vec();
                            nv.push(Rc::new(Expr::new(b, *op, a)));
                            compute_24_recursive(goal, &nv, exps);
                        }

                        let mut nv = nv.to_vec();
                        nv.push(Rc::new(Expr::new(a, *op, b)));
                        compute_24_recursive(goal, &nv, exps);
                    });
                }
            }));
    }

    //todo!();
    //Ok(())
}

#[allow(dead_code)]
fn  compute_24() {
    let mut goal = 24;

    let mut nums = env::args().peekable();
    nums.next();    // skip the executable path
    if let Some(opt) = nums.peek() {
        if opt == "-g" {    nums.next();
            if let Some(gs) = &nums.next() {
                match gs.parse::<i32>() {
                    Ok(_goal) => goal = _goal,
                    Err(e) => eprintln!("Error parsing GOAL: {e}"),
                }
            } else { eprintln!("Lack parameter for GOAL!"); }
        }

        compute_24_algo(goal, nums);
    }

    println!("### Game {goal} computation ###");
    loop {  print!("\nInput a data series: ");

        let mut nums = String::new();
        io::stdout().flush().expect("Failed to flush!"); //.unwrap();
        io::stdin().read_line(&mut nums).expect("Failed to read!");
        let mut nums  = nums.trim().split(' ').filter(|s| !s.is_empty()).peekable();

        if let Some(first) = nums.peek() {
            if first.starts_with(&['g', 'G']) {
                match first[1..].parse::<i32>() {
                    Ok(_goal) => println!("\n### Reset GOAL to {} ###", goal = _goal),
                    Err(e) => eprintln!("Error parsing GOAL: {e}"),
                }   nums.next();
            } else if first.eq_ignore_ascii_case("quit") { break }
        }

        compute_24_algo(goal, nums);
    }
}

#[allow(dead_code)]
fn  guess_number() {    // interactive function
    //struct Param { max: i32, lang: bool }; let param = Param { max: 100, lang: true };
    //struct Param(i32, bool); let param = Param(100, true); //let param = (100, true);
    let (max, lang) = (100, false);

    struct _Tips<'a> { title: &'a str, prompt: &'a str,
            too_big: &'a str, too_small: &'a str, bingo: &'a str }
    let [ title, prompt, too_big, too_small, bingo ] = if lang {
        [ "猜数字游戏", "输入你猜的数字: ", "太大了", "太小了", "对了!" ]
    } else {
        [ "Guess the number", "Input a number you guess: ",
            "Too large", "Too small", "Bingo!" ]
    };  // i18n mechanism?

    use rand::Rng;
    let secret = rand::thread_rng().gen_range(1..=max); //dbg!(secret);
    println!("### {title} (1~{max}) ###");

    let _result = 'label: loop {    // unused prefixed with underscore
        print!("\n{prompt}");

        let mut guess = String::new();
        io::stdout().flush().expect("Failed to flush!"); //.unwrap();
        io::stdin().read_line(&mut guess).expect("Failed to read!");
        let guess = guess.trim();

        //let guess: i32 = guess.parse().expect("Please type a number");
        //match guess.parse::<i32>() { Ok(_guess) => { }, _ => () }
        if let Ok(guess) = guess.parse::<i32>() { // isize
            //if (guess < secret) { } else if (secret < guess) { } else { }
            match guess.cmp(&secret) {
                Ordering::Greater =>    println!("[{too_big}]"),
                Ordering::Less    =>    println!("[{too_small}]"),
                Ordering::Equal   => {  println!("[{bingo}]"); break 1 }
            }
        } else if guess.eq_ignore_ascii_case("quit") { break 'label 0 }
        //guess.make_ascii_lowercase();  //guess.to_lowercase();
    };
}

#[allow(dead_code)]
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    // for &item in list {}
    for item in list { if  largest < item { largest = item; } }
    largest
}

fn  _calc_pi() {    // a streaming/spigot algorithm     // https://rosettacode.org/wiki/Pi
    use num_bigint::BigInt;
    let mut first = true;

    let mut q = BigInt::from(1);
    let mut r = BigInt::from(0);
    let mut t = BigInt::from(1);
    let mut k = BigInt::from(1);
    let mut n = BigInt::from(3);
    let mut l = BigInt::from(3);

    loop {
        if &q * 4 + &r - &t < &n * &t {
            print!("{}", n);
            if first { print!("."); first = false; }
            let nr = (&r - &n * &t) * 10;
            n = (&q * 3 + &r) * 10 / &t - &n * 10;
            q *= 10;
            r = nr;
        } else {
            let nr = (&q * 2 + &r) * &l;
            let nn = (&q * &k * 7 + 2 + &r * &l) / (&t * &l);
            q *= &k;
            t *= &l;
            l += 2;
            k += 1;
            n = nn;
            r = nr;
        }
    }
}
