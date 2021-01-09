// Length of entries in the Distro enum.
pub const DISTROS: i32 = 4;

#[allow(dead_code)]
pub enum Distro {
    Arch,
    Manjaro,
    Debian,
    Fedora,
}

pub fn choose_art(distro: Distro) -> (u16, &'static str) {
    match distro {
        Distro::Arch => ARCH_LOGO,
        Distro::Manjaro => MANJARO_LOGO,
        Distro::Debian => DEBIAN_LOGO,
        Distro::Fedora => FEDORA_LOGO,
    }
}

const ARCH_LOGO: (u16, &str) = (
    20,
    "
{c}                  -`
                 .o+`
                `ooo/
               `+oooo:
              `+oooooo:
              -+oooooo+:
            `/:-:++oooo+:
           `/++++/+++++++:
          `/++++++++++++++:
         `/+++ooooooooooooo/`
        ./ooosssso++osssssso+`
       .oossssso-````/ossssss+`
      -osssssso.      :ssssssso.
     :osssssss/        osssso+++.
    /ossssssss/        +ssssooo/-
  `/ossssso+/:-        -:/+osssso+-
 `+sso+:-`                 `.-/+oso:
`++:.                           `-/+/
.`                                 `/
",
);

const MANJARO_LOGO: (u16, &str) = (
    20,
    "
{g} ██████████████████  ████████
 ██████████████████  ████████
 ██████████████████  ████████
 ██████████████████  ████████
 ████████            ████████
 ████████  ████████  ████████
 ████████  ████████  ████████
 ████████  ████████  ████████
 ████████  ████████  ████████
 ████████  ████████  ████████
 ████████  ████████  ████████
 ████████  ████████  ████████
 ████████  ████████  ████████
 ████████  ████████  ████████
 ",
);

const DEBIAN_LOGO: (u16, &str) = (
    33,
    "
{r}        _,met$$$$$gg.
     ,g$$$$$$$$$$$$$$$P.
   ,g$$P\"\"       \"\"\"Y$$.\".
  ,$$P'              `$$$.
',$$P       ,ggs.     `$$b:
`d$$'     ,$P\"'   .    $$$
 $$P      d$'     ,    $$P
 $$:      $$.   -    ,d$$'
 $$;      Y$b._   _,d$P'
 Y$$.    `.`\"Y$$$$P\"'
 `$$b      \"-.__
  `Y$$b
   `Y$$.
     `$$b.
       `Y$$b.
         `\"Y$b._
             `\"\"\"\"\n",
);

const FEDORA_LOGO: (u16, &str) = (
    20,
    "
{b}        /:-------------:\\
       :-------------------::
     :-----------/{w}shhOHbmp{b}---:\\
   /-----------{w}omMMMNNNMMD{b}   ---:
  :-----------{w}sMMMMNMNMP.{b}     ---:
 :-----------{w}:MMMdP{b}-------    ---\\
,------------{w}:MMMd{b}--------    ---:
:------------{w}:MMMd{b}-------    .---:
:----    {w}oNMMMMMMMMMNho{b}     .----:
:--     {w}.+shhhMMMmhhy++{b}   .------/
:-    -------{w}:MMMd{b}--------------:
:-   --------{w}/MMMd{b}-------------;
:-    ------{w}/hMMMy{b}------------:
:-- {w}:dMNdhhdNMMNo{b}------------;
:---{w}:sdNMMMMNds{b}:------------:
:------{w}:://:{b}-------------::
 :---------------------://
",
);
