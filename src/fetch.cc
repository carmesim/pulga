#include "h/fetch.hh"
#include <fstream>   // ifstream
#include <memory>
#include <algorithm>

using std::ifstream;
using std::string;

static void string_replace( string &s, const string &search, const string &replace ) {
    for( size_t pos = 0; ; pos += replace.length() ) {
        pos = s.find(search, pos);
        if(pos == string::npos) break;
        s.erase(pos, search.length());
        s.insert(pos, replace);
    }
}

static std::string shell_cmd(const char * cmd) {
    char buffer[128];
    std::string result;
    std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
    if (!pipe)
    {
        return "RPIData::shell_cmd failed.";
    }
    while (fgets(buffer, 128, pipe.get()) != nullptr)
    {
        result += buffer;
    }
    return result;
}


std::string ExoFetch::get_kernel()
{
    return shell_cmd("uname -r | sed \"s/-.*//g\"");
}

// TODO: make this better
std::string ExoFetch::get_arch()
{
#if defined(__amd64__) || defined(__amd64) || defined(__x86_64__) || defined(__x86_64)
    return "x86-64";
#elif defined(__aarch64__)
    return "ARM64";
#elif defined(i386) || defined(__i386) || defined(__i386__) || defined(__i386__) || defined(__i486__) || defined(__i586__)  || defined(__i686__)
    return "x86";
#elif defined(__hppa__) || defined(__hppa)
    return "RISC-V";
#elif defined(__ARM_ARCH_7__) || defined(__ARM_ARCH_7A__) || defined(__ARM_ARCH_7R__)|| defined(__ARM_ARCH_7M__) || defined(__ARM_ARCH_7S__)
    return "ARMv7";
#else
#warn "Consider adding your architecture here"
    return "Unknown";
#endif
}


// TODO: non-dpkg systems
std::string ExoFetch::get_packages()
{
    return shell_cmd("dpkg -l | grep -c ^i");
}

std::string ExoFetch::get_distro()
{
    std::string res = shell_cmd("cat /etc/os-release | grep PRETTY_NAME=");
    //res = std::regex_replace(res, std::regex("PRETTY_NAME="), "");
    //res = std::regex_replace(res, std::regex("\""), "");
    string_replace(res, "PRETTY_NAME=", "");
    string_replace(res, "\"", "");
    string_replace(res, "\n", "");

    std::transform(res.begin(), res.end(), res.begin(), [](unsigned char c){ return std::tolower(c); });
    return res;
}

std::string ExoFetch::get_shell()
{
    return shell_cmd("basename $SHELL");
}

std::string ExoFetch::get_uptime()
{
        ifstream in;
        in.open("/proc/uptime");
        unsigned long int proc_uptime;
        in >> proc_uptime;
        int years   =  proc_uptime / 60 / 60 / 24   / 365;
        int days    = (proc_uptime / 60 / 60 / 24)  % 365;
        int hours   = (proc_uptime / 3600) % 24;
        int minutes = (proc_uptime / 60) % 60;
        int seconds = proc_uptime  % 60;

        std::string result;
        if (years)
        {
            result += std::to_string(years);
            result += years > 1 ? " years " : " year ";
        }

        if (days)
        {
            result += std::to_string(days);
            result += days > 1 ? " days " : " day ";
        }

        if (hours)
        {
            result += std::to_string(hours);
            result += hours > 1 ? " hours " : " hour ";
        }

        if (minutes)
        {
            result += std::to_string(minutes);
            result += minutes > 1 ? " minutes " : " minute ";
        }

        if (seconds)
        {
            result += std::to_string(seconds);
            result += seconds > 1 ? " seconds" : " second";
        }
        return result;
}
