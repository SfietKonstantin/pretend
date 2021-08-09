use pretend::pretend;

#[pretend]
trait Test {
    type Item;

    #[request(method = "GET", path = "/get")]
    async fn test(&self) -> Result<()>;

}

fn main() {}
