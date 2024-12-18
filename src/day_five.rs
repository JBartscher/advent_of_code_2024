use crate::read_input;

pub fn first_task() {
    let rules = read_input("./input/5_page_ordering_rules");
    let pages = read_input("./input/5_pages_to_produce");

    let rules = rules
        .iter()
        .map(|l| {
            let (a, b) = l.split_at(l.find('|').unwrap());
            let b = b.replace("|", "");
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    let pages = pages
        .iter()
        .map(|l| {
            l.split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

   // println!("{:?}", pages);

    for p in pages.iter(){
        for r in &rules {
            if is_rule_applicable(p, r) {

            }
        }

    }

    println!("Answer 1/2: {}", 0);
}

pub fn second_task() {
    println!("Answer 2/2: {}", 0);
}

fn is_rule_applicable(page_ordering: &[u32], rule: &(u32, u32)) -> bool {
    page_ordering.contains(&rule.0) && page_ordering.contains(&rule.1)
}
