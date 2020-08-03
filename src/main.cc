#include <iostream>
#include "../h/fetch.hh"

/* this will give the correct ammount of space
 based on the word length */
std::string auto_spacer(int word_length){
    /* system arch is the biggest "word", that's 11 chars and I want at least 
     one space after each description */
    int word_max = 12; 
    std::string spaces="";

    // the for loop generates the spaces for each word length
    for(int s=0; s<(word_max - word_length); s++){
        spaces+=" ";
    }
    // at the end, just add the separator
    spaces += "> ";
    return spaces;
}

void print_info(){
    // the variables are only because I need to access their lengths
    std::string username = "username";
    std::string hostname = "hostname";
    std::string os       = "os";
    std::string kernel   = "kernel";
    std::string sysarch  = "system arch";
    std::string uptime   = "uptime";
    std::string packages = "packages";
    std::string shell    = "shell";
    std::string cpu      = "cpu";

    std::cout << username << auto_spacer(username.length()) << ExoFetch::get_username() << '\n';
    std::cout << hostname << auto_spacer(hostname.length()) << ExoFetch::get_hostname() << '\n';
    std::cout << os       << auto_spacer(os.length())       << ExoFetch::get_distro() << '\n';
    std::cout << kernel   << auto_spacer(kernel.length())   << ExoFetch::get_kernel();
    std::cout << sysarch  << auto_spacer(sysarch.length())  << ExoFetch::get_arch() << '\n';
    std::cout << uptime   << auto_spacer(uptime.length())   << ExoFetch::get_uptime() << '\n';
    std::cout << packages << auto_spacer(packages.length()) << ExoFetch::get_packages();
    std::cout << shell    << auto_spacer(shell.length())    << ExoFetch::get_shell();
    std::cout << cpu      << auto_spacer(cpu.length())      << ExoFetch::get_cpu_model();
}

int main()
{
    print_info();
    return 0;
}
