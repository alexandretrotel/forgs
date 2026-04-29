use std::io::{self, IsTerminal};

use crate::models::organization::OrganizationRank;
use crate::models::scan_result::ScanResult;

pub fn write_results(results: &[ScanResult]) {
    let colors = io::stdout().is_terminal();

    for (index, result) in results.iter().enumerate() {
        if index > 0 {
            println!();
        }

        print_result(result, colors);
    }
}

fn print_result(result: &ScanResult, colors: bool) {
    println!(
        "{}",
        style(
            &format!("{}/{}", result.repository.owner, result.repository.name),
            colors,
            "\x1b[1;36m",
        )
    );
    println!(
        "{}",
        style(
            &format!("Organizations found: {}", result.organizations.len()),
            colors,
            "\x1b[2m",
        )
    );
    println!();

    for (rank, organization) in result.organizations.iter().enumerate() {
        print_organization(rank + 1, organization, colors);
    }
}

fn print_organization(rank: usize, organization: &OrganizationRank, colors: bool) {
    let rank_label = style(&format!("{rank:>2}."), colors, "\x1b[2m");
    let name = style(&organization.name, colors, "\x1b[1;32m");
    let followers = style(
        &format!("{} followers", organization.followers),
        colors,
        "\x1b[1;33m",
    );
    let url = style(&organization.organization_url, colors, "\x1b[2;34m");

    println!("{rank_label} {name}  {followers}");
    println!("    {url}");
}

fn style(text: &str, colors: bool, prefix: &str) -> String {
    if colors {
        format!("{prefix}{text}\x1b[0m")
    } else {
        text.to_string()
    }
}
