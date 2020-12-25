use indoc::indoc;

#[allow(dead_code)]
pub enum Logo {
    Arch,
    Manjaro,
    Debian,
    Fedora,
}

pub fn choose_logo(logo: Logo) -> &'static str {
    match logo {
        Logo::Arch => ARCH_LOGO,
        Logo::Manjaro => MANJARO_LOGO,
	Logo::Debian => DEBIAN_LOGO,
	Logo::Fedora => FEDORA_LOGO,
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

const MANJARO_LOGO: &str = indoc! {"\n
    {g} ██████████████████  ████████
    {g} ██████████████████  ████████
    {g} ██████████████████  ████████
    {g} ██████████████████  ████████
    {g} ████████            ████████
    {g} ████████  ████████  ████████
    {g} ████████  ████████  ████████
    {g} ████████  ████████  ████████
    {g} ████████  ████████  ████████
    {g} ████████  ████████  ████████
    {g} ████████  ████████  ████████
    {g} ████████  ████████  ████████
    {g} ████████  ████████  ████████
    {g} ████████  ████████  ████████\n"};

const DEBIAN_LOGO: &str = indoc! {"\n
{r}        _,met$$$$$gg.
{r}     ,g$$$$$$$$$$$$$$$P.
{r}   ,g$$P\"\"       \"\"\"Y$$.\".
{r}  ,$$P'              `$$$. 
{r}',$$P       ,ggs.     `$$b:
{r}`d$$'     ,$P\"'   .    $$$
{r} $$P      d$'     ,    $$P
{r} $$:      $$.   -    ,d$$'
{r} $$;      Y$b._   _,d$P'
{r} Y$$.    `.`\"Y$$$$P\"' 
{r} `$$b      \"-.__
{r}  `Y$$b
{r}   `Y$$.
{r}     `$$b.
{r}       `Y$$b.
{r}         `\"Y$b._
{r}             `\"\"\"\"\n"};

const FEDORA_LOGO: &str = indoc! {"\n
{b}        /:-------------:\\
{b}       :-------------------::
{b}     :-----------/{w}shhOHbmp{b}---:\\
{b}   /-----------{w}omMMMNNNMMD{b}   ---:
{b}  :-----------{w}sMMMMNMNMP.{b}     ---:
{b} :-----------{w}:MMMdP{b}-------    ---\\
{b},------------{w}:MMMd{b}--------    ---:
{b}:------------{w}:MMMd{b}-------    .---:
{b}:----    {w}oNMMMMMMMMMNho{b}     .----:
{b}:--     {w}.+shhhMMMmhhy++{b}   .------/
{b}:-    -------{w}:MMMd{b}--------------:
{b}:-   --------{w}/MMMd{b}-------------;
{b}:-    ------{w}/hMMMy{b}------------:
{b}:-- {w}:dMNdhhdNMMNo{b}------------;
{b}:---{w}:sdNMMMMNds{b}:------------:
{b}:------{w}:://:{b}-------------::
{b} :---------------------://
"};
