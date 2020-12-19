use indoc::indoc;

#[allow(dead_code)]
pub enum Logo {
    Arch,
    Manjaro,
}

pub fn choose_logo(logo: Logo) -> &'static str {
    match logo {
        Logo::Arch => ARCH_LOGO,
        Logo::Manjaro => MANJARO_LOGO,
    }
}

const ARCH_LOGO: &str = indoc! {"
    {c}                  -`
    {c}                 .o+`
    {c}                `ooo/
    {c}               `+oooo:
    {c}              `+oooooo:
    {c}              -+oooooo+:
    {c}            `/:-:++oooo+:
    {c}           `/++++/+++++++:
    {c}          `/++++++++++++++:
    {c}         `/+++ooooooooooooo/`
    {c}        ./ooosssso++osssssso+`
    {c}       .oossssso-````/ossssss+`
    {c}      -osssssso.      :ssssssso.
    {c}     :osssssss/        osssso+++.
    {c}    /ossssssss/        +ssssooo/-
    {c}  `/ossssso+/:-        -:/+osssso+-
    {c} `+sso+:-`                 `.-/+oso:
    {c}`++:.                           `-/+/
    {c}.`                                 `/"};

const MANJARO_LOGO: &str = indoc! {"
    {g}██████████████████  ████████
    {g}██████████████████  ████████
    {g}██████████████████  ████████
    {g}██████████████████  ████████
    {g}████████            ████████
    {g}████████  ████████  ████████
    {g}████████  ████████  ████████
    {g}████████  ████████  ████████
    {g}████████  ████████  ████████
    {g}████████  ████████  ████████
    {g}████████  ████████  ████████
    {g}████████  ████████  ████████
    {g}████████  ████████  ████████
    {g}████████  ████████  ████████"};
