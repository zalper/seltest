use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let mut caps = DesiredCapabilities::firefox();
     caps.add_firefox_arg("--enable-automation")?;
     let driver = WebDriver::new("http://172.17.0.1:4444", caps).await?;

     // Navigate to https://wikipedia.org.
     driver.goto("https://wikipedia.org").await?;

     // Find search form
     let elem_form = driver.find(By::Id("search-form")).await?;

     // Find form input
     let elem_text = elem_form.find(By::Id("searchInput")).await?;

     // Type in the search terms.
     elem_text.send_keys("selenium").await?;

     // Click the search button.
     let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
     elem_button.click().await?;

     // Look for header to implicitly wait for the page to load.
     driver.find(By::ClassName("firstHeading")).await?;
     assert_eq!(driver.title().await?, "Selenium - Wikipedia");

     driver.goto("https://translate.google.com/").await?;

     let trform = driver.find(By::ClassName("er8xn")).await?;
     trform.send_keys("Hello").await?;

     driver.quit().await?;

     Ok(())
}
