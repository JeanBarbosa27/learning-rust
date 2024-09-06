use crate::cli;

pub fn run() {
    #[derive(Debug)]
    enum Roles {
        Admin,
        Moderator,
        User,
    }

    #[derive(Debug)]
    enum Permitions {
        Create,
        Delete,
        Read,
        Update,
    }

    impl Roles {
        fn get_role_permitions(role: Roles) -> Vec<Permitions> {
            match role {
                Roles::Admin => Vec::from([
                    Permitions::Create,
                    Permitions::Read,
                    Permitions::Update,
                    Permitions::Delete,
                ]),
                Roles::Moderator => {
                    Vec::from([Permitions::Read, Permitions::Update, Permitions::Delete])
                }
                Roles::User => Vec::from([Permitions::Read]),
            }
        }
    }

    println!("{:?}", [Roles::Admin, Roles::Moderator, Roles::User]);
    let chosen_role: String =
        cli::input::from_user::generic("Pick one of them to see the permitions:");
    match &chosen_role.to_lowercase()[..] {
        "admin" => println!(
            "Permitions for admin: {:?}",
            Roles::get_role_permitions(Roles::Admin)
        ),
        "moderator" => println!(
            "Permitions for moderator: {:?}",
            Roles::get_role_permitions(Roles::Moderator)
        ),
        "user" => println!(
            "Permitions for user: {:?}",
            Roles::get_role_permitions(Roles::User)
        ),
        _ => println!(
            "{} isn't a valid role, so there isn't any permition to list.",
            chosen_role
        ),
    };
}
