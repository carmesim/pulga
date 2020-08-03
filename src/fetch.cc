#include "h/fetch.hh"
#include <fstream>   // ifstream
#include <memory>    // FILE, unique_tr
#include <algorithm> // find_if, transform
#include <unistd.h>  // gethostname
#include <pwd.h>     // struct passwd, getpwuid_r

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

static inline void ltrim(std::string &s) {
    s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](int ch) {
        return !std::isspace(ch);
    }));
}


std::string ExoFetch::get_cpu_model()
{
    std::string res = shell_cmd("cat /proc/cpuinfo | grep \"model name\" -m 1");
    string_replace(res, "model name	:", "");
    ltrim(res);
    return res;
}


std::string ExoFetch::get_username()
{
    // Gets the effective ID of the user
    uid_t uid = geteuid();
    struct passwd pwent;
    struct passwd *pwent_ptr;
    char buffer[1024];

    // Looks for the UID on the password databank and saves the result on pwent
    getpwuid_r(uid, &pwent, buffer, sizeof buffer, &pwent_ptr);
    return pwent.pw_name;
}

std::string ExoFetch::get_hostname()
{
    char buffer[64];
    gethostname(buffer, 64);
    return buffer;
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
