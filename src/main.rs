struct User {
    username: String,
    email: String,
    active: bool,
}

struct Cli {
    pattern: String,
}

struct Konto {
    name: String,
    balance: f32,
}

impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
        }
    }
}

impl Cli {
    fn new(args: &[String]) -> Result<Cli, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let pattern = args[1].clone();
        Ok(Cli { pattern })
    }
}

impl Konto {
    fn new(name: String, balance: f32) -> Konto {
        Konto { name, balance }
    }
}

fn main() {
    // get the arguments of cli
    let args = std::env::args().collect::<Vec<String>>();
    let cli = Cli::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    // check if the pattern is new
    if cli.pattern == "new" {
        // ask for username and email
        println!("Enter username: ");
        let mut user_name = String::new();
        std::io::stdin()
            .read_line(&mut user_name)
            .expect("Failed to read line");
        println!("Enter email: ");
        let mut user_email = String::new();
        std::io::stdin()
            .read_line(&mut user_email)
            .expect("Failed to read line");

        let user = User::new(user_name, user_email);
        println!("User: {} {}", user.username, user.email);
        println!("User: {}", user.active);

        let mut konto = Konto::new(user.username, 0.0);
        println!("Konto: {} {}", konto.name, konto.balance);

        println!("What would you add to your account?");
        let mut add = String::new();
        std::io::stdin()
            .read_line(&mut add)
            .expect("Failed to read line");
        let add: f32 = add.trim().parse().expect("Please type a number!");
        konto.balance += add;
        println!("Konto: {} {}", konto.name, konto.balance);
        let mut konto_file = std::fs::File::create("konto.txt").expect("create failed");
        std::io::Write::write_all(
            &mut konto_file,
            format!("{} {}", konto.name, konto.balance).as_bytes(),
        )
        .expect("write failed");
    }

    if cli.pattern == "show" {
        let mut konto_file = std::fs::File::open("konto.txt").expect("open failed");
        let mut konto_string = String::new();
        std::io::Read::read_to_string(&mut konto_file, &mut konto_string).expect("read failed");
        println!("{}", konto_string);
    }
}
