#ifndef __EXO_FETCH
#define __EXO_FETCH

#include <string>

// os:       linux mint
// kernel:   version 5.4.0
// wm:
// packages: 3185
// sysarch:  x86_64
// uptime:   11 hours, 37 minutes

namespace ExoFetch {
    std::string get_uptime();
    std::string get_kernel();
    std::string get_arch();
    std::string get_distro();
    std::string get_packages();
    std::string get_shell();
}

#endif
