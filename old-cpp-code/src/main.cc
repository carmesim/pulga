#include <iostream>
#include "../h/fetch.hh"

//std::string auto_spacer(int word_length){
//    /* system arch is the biggest "word", that's 11 chars and I want at least
//     one space after each description */
//    int word_max = 12;
//    std::string spaces = std::string(word_max - word_length, ' ') + "> ";
//    return spaces;
//}

#define COLOR1 = "\033[0;35m" // titles
#define _COLOR2 = "\033[0;31m" // ascii logo
#define _COLOR3 = "\033[1;32m" // info
#define _RESET  = "\033[0m"    // reset coloring

static inline void print_info(){
    // the variables are only because I need to access their lengths
    const char * color1 = "\033[0;35m";
    const char * color3 = "\033[1;32m";
    unsigned char max_width = 12;
    char buffer[2048];
    int length = 0;
    length += sprintf(buffer, "%s%-*s> %s%s\n", color1, max_width, "username",   color3, ExoFetch::get_username().c_str());
    length += sprintf(buffer + length, "%s%-*s> %s%s\n", color1, max_width, "hostname",   color3, ExoFetch::get_hostname().c_str());
    length += sprintf(buffer + length, "%s%-*s> %s%s\n", color1, max_width, "os",         color3, ExoFetch::get_distro().c_str());
    length += sprintf(buffer + length, "%s%-*s> %s%s\n", color1, max_width, "kernel",     color3, ExoFetch::get_kernel().c_str());
    length += sprintf(buffer + length, "%s%-*s> %s%s\n", color1, max_width,"system arch", color3, ExoFetch::get_arch().c_str());
    length += sprintf(buffer + length, "%s%-*s> %s%s\n", color1, max_width, "uptime",     color3, ExoFetch::get_uptime().c_str());
    length += sprintf(buffer + length, "%s%-*s> %s%s\n", color1, max_width,"packages",    color3, ExoFetch::get_packages().c_str());
    length += sprintf(buffer + length, "%s%-*s> %s%s\n", color1, max_width, "shell",      color3, ExoFetch::get_shell().c_str());
    length += sprintf(buffer + length, "%s%-*s> %s%s\n", color1, max_width, "cpu",        color3, ExoFetch::get_cpu_model().c_str());
    puts(buffer);
}

int main()
{
    print_info();
    printf("%s", "\033[0m");
    return 0;
}
