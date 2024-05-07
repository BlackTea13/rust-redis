#[derive(Debug)]
pub enum Command {
    SELECT,
    GET,
    SET,
    PING,
    EXISTS,
    RPUSH,
    LPUSH,
    BLPOP,
    BRPOP,
}

impl Command {}

struct SELECT {
    database: u8,
}

impl SELECT {}
