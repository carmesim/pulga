#include <iostream>
#include "h/fetch.hh"

int main()
{
    std::cout << ExoFetch::get_kernel();
    std::cout << ExoFetch::get_arch() << '\n';
    std::cout << ExoFetch::get_distro() << '\n';
    std::cout << ExoFetch::get_uptime() << '\n';
    std::cout << ExoFetch::get_packages();
    std::cout << ExoFetch::get_shell();
    return 0;
}
