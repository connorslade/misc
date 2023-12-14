use scraper::Html;
use ureq::AgentBuilder;

const BASE_PAGE: &str = "https://parents.c2.genesisedu.net/bernardsboe";

const USERNAME: &str = "";
const PASSWORD: &str = "";
const STUDENT_ID: &str = "";

fn main() {
    let agent = AgentBuilder::new()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:120.0) Gecko/20100101 Firefox/120.0")
        .build();

    agent.get(&format!("{BASE_PAGE}/sis/view")).call().unwrap();
    agent
        .get(&format!("{BASE_PAGE}/sis/j_security_check"))
        .query("j_username", USERNAME)
        .query("j_password", PASSWORD)
        .call()
        .unwrap();
    let assignments = agent.get(&format!("https://parents.c2.genesisedu.net/bernardsboe/parents?tab1=studentdata&tab2=gradebook&tab3=listassignments&action=form&studentid={STUDENT_ID}")).call().unwrap();

    let document = Html::parse_document(&assignments.into_string().unwrap());
    let items = document.select(&scraper::Selector::parse("table.list tr[style]").unwrap());

    // Date: `.cellCenter div:nth-child(2)`
    // Class: `.cellLeft[height] div:nth-child(1)`
    // Assignment: `.cellLeft:not([height]) b`
    // Grade: `.cellLeft[nowrap]`
}
