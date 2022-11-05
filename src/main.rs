use exitfailure::ExitFailure;
use structopt::StructOpt;

pub mod weather;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = weather::Cli::from_args();
    println!("{:?}", args);
    let responseFromAPI = weather::Forecast::get(&args.city, &args.country_code).await?;
    println!("{:?}", responseFromAPI.main.humidity);
    Ok(())
}
