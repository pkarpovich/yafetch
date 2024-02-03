#include "yafetch.h"
#include <lauxlib.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pwd.h>
#include <unistd.h>

#include <curl/curl.h>
#include <netdb.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>


#include <sys/sysinfo.h>
#include <sys/utsname.h>
#include <sys/mount.h>
#include <sys/param.h>

#define LFUNC(N) int lua_##N(lua_State *L)

/* yafetch.public_ip() */
/* Returns public ip address */
/* make a http request to a website that returns public ip */
LFUNC(public_ip) {
    struct addrinfo hints = {0}, *addrs;
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_protocol = 0;

    int rval = getaddrinfo("whatismyip.akamai.com", "80", &hints, &addrs);
    if (rval != 0) {
        fprintf(stderr,"getaddrinfo failed: %s\n", gai_strerror(rval));
        return 1;
    }

    int s = socket(addrs->ai_family, addrs->ai_socktype, addrs->ai_protocol);
    if (s == -1) {
        perror("socket failed");
        return 1;
    }
    rval = connect(s, addrs->ai_addr, addrs->ai_addrlen);
    if (rval == -1) {
        perror("connect failed");
        return 1;
    }

    char cmd[] = "GET / HTTP/1.1\nHost: whatismyip.akamai.com\n\n";
    rval = send(s, cmd, strlen(cmd), 0);
    if (rval == -1) {
        perror("send failed");
        return 1;
    }

    char buf[1000] = {0};
    rval = recv(s, buf, sizeof(buf), 0);
    if (rval == -1) {
        perror("recv failed");
        return 1;
    }

    char *start = buf, *end;
    end = strchr(start, '\n');
    if (!strncmp(start, "HTTP/1.1 200 OK", end - start - 1)) {
        while (!(end[1] == '\r' && end[2] == '\n')) {
            start = end + 2;
            end = strchr(start, '\n');
        }
        start = end + 3;
        end = strchr(start, '\n');
        if (end) *end = 0;
    }

    close(s);
    freeaddrinfo(addrs);

    lua_pushstring(L, start);

    return 1;
}

/* yafetch.local_ip() */
/* Returns local ip address */
LFUNC(local_ip) {
    char host[256];
    char *IP;
    struct hostent *host_entry;
    int hostname;

    //find the host name
    hostname = gethostname(host, sizeof(host));

    //find host information
    host_entry = gethostbyname(host);

    // convert ip to string
    IP = inet_ntoa(*((struct in_addr*) host_entry->h_addr_list[0]));

    lua_pushstring(L, IP);

    return 1;
}

/* yafetch.user() */
/* Returns username */
LFUNC(user) {

    uid_t uid = geteuid();
    struct passwd *pw = getpwuid(uid);

    if (pw) {
        lua_pushstring(L, pw->pw_name);
    } else {
        lua_pushstring(L, "unknown");
    }

    return 1;
}

/* yafetch.distro() */
/* Returns name of linux distribution */
LFUNC(distro) {
    char *def = malloc(512);
    char *new = malloc(512);
    int line = 0;

    FILE *f = fopen("/etc/os-release", "rt");

    while (fgets(def, 512, f)) {
        snprintf(new, 512, "%.*s", 511, def + 4);
        if (strncmp(new, "=", 1) == 0)
            break;
        line++;
    }

    fclose(f);
    free(def);

    if (strncmp(new, "=", 1) == 0) {
        int len = strlen(new);
        for (int i = 0; i < len; i++) {
            if (new[i] == '\"' || new[i] == '\'' || new[i] == '=') {
                for (int ii = 0; ii < len; ii++)
                    new[ii] = new[ii + 1];
                new[strlen(new) - 1] = '\0';
            }
        }
    }

    if (new) {
        lua_pushstring(L, new);
    } else {
        lua_pushstring(L, "unknown");
    }
    return 1;
}

/* yafetch.hostname() */
/* Returns hostname of the machine */
LFUNC(hostname) {
    /* Maximum characters of the hostname can be 255 on linux(+1 0 terminator)
     */
    char hostname[255];
    gethostname(hostname, 255);

    lua_pushstring(L, hostname);
    return 1;
}

/* yafetch.pkgs() */
/* Returns number of installed packages */
LFUNC(pkgs) {
    int apt, dnf, emerge, nix, pacman, rpm, xbps, bonsai, apk, total = 0;

    FILE *file[9];
    file[0] = popen(
        "dpkg-query -f '${binary:Package}\n' -W 2> /dev/null | wc -l", "r");
    file[1] = popen("dnf list installed 2> /dev/null | wc -l", "r");
    file[2] = popen("qlist -I 2> /dev/null | wc -l", "r");
    file[3] = popen(
        "nix-store -q --requisites /run/current-system/sw 2> /dev/null | wc -l",
        "r");
    file[4] = popen("pacman -Qq 2> /dev/null | wc -l", "r");
    file[5] = popen("rpm -qa --last 2> /dev/null | wc -l", "r");
    file[6] = popen("xbps-query -l 2> /dev/null | wc -l", "r");
    file[7] = popen("bonsai list  2> /dev/null | wc -l", "r");
    file[8] = popen("apk info 2> /dev/null | wc -l", "r");

    fscanf(file[0], "%d", &apt);
    fscanf(file[1], "%d", &dnf);
    fscanf(file[2], "%d", &emerge);
    fscanf(file[3], "%d", &nix);
    fscanf(file[4], "%d", &pacman);
    fscanf(file[5], "%d", &rpm);
    fscanf(file[6], "%d", &xbps);
    fscanf(file[7], "%d", &bonsai);
    fscanf(file[8], "%d", &apk);
    for (int i = 0; i < 9; i++)
        fclose(file[i]);

    if (apt > 0)
        total += apt;
    if (dnf > 0)
        total += dnf;
    if (emerge > 0)
        total += emerge;
    if (nix > 0)
        total += nix;
    if (pacman > 0)
        total += pacman;
    if (rpm > 0)
        total += rpm;
    if (xbps > 0)
        total += xbps;
    if (bonsai > 0)
        total += bonsai;
    if (apk > 0)
        total += apk;

    if (total) {
        lua_pushinteger(L, total);
    } else {
        lua_pushinteger(L, 0);
    }

    return 1;
}

/* yafetch.kernel() */
/* Returns kernel version */
LFUNC(kernel) {
    struct utsname sys;
    uname(&sys);

    char *kernel = sys.release;
    if (kernel) {
        lua_pushstring(L, kernel);
    } else {
        lua_pushstring(L, "unknown");
    }
    return 1;
}

/* yafetch.cpu() */
/* Returns cpu model name */
LFUNC(cpu) {
    char *cpu_info;
    char *end;

    FILE *fp = fopen("/proc/cpuinfo", "r");
    if(!fp) return 1;

    char *buf = malloc(0x10001);
    buf[fread(buf, 1, 0x10000, fp)] = 0;
    fclose(fp);

    cpu_info = buf;

    cpu_info = strstr(cpu_info, "model name");
    if(!cpu_info) {
        return 1;
        free(cpu_info);
    }

    cpu_info += 13;

    end = strstr(cpu_info, " @");
    if(end)
        *end = 0;
    else {
        end = strchr(cpu_info, '\n');
        if(!end) {
            return 1;
            free(cpu_info);
        }
            
        *end = 0;
    }
    ++end;

    lua_pushstring(L, cpu_info);

    return 1;
}

/* yafetch.shell() */
/* Returns path of shell */
LFUNC(shell) {

    lua_getglobal(L, "yafetch");
    lua_getfield(L, -1, "shell_base");

    const int shell_full = lua_toboolean(L, -1);
    char *shell = getenv("SHELL");

    /* Get basename of shell by looking for last '/' */
    char *slash = strrchr(shell, '/');

    if (shell) {
        if (shell_full == 1) {
            shell = slash + 1;
        }
        lua_pushstring(L, shell);
    } else {
        lua_pushstring(L, "unknown");
    }

    return 1;
}

/* yafetch.header() */
LFUNC(header) {

    lua_getglobal(L, "yafetch");

    lua_getfield(L, -1, "header_sep");
    const char *sep = lua_tostring(L, -1);

    if (lua_isnil(L, -1) == 1) {
        sep = "@";
    }

    lua_getglobal(L, "yafetch");

    lua_getfield(L, -1, "header_sep_color");
    const char *sep_color = lua_tostring(L, -1);
    lua_pop(L, 0);

    if (lua_isnil(L, -1) == 1) {
        sep_color = "";
    }

    lua_getglobal(L, "yafetch");

    lua_getfield(L, -1, "header_format");
    const char *fmt = lua_tostring(L, -1);
    lua_pop(L, 0);

    if (lua_isnil(L, -1) == 1) {
        fmt = "";
    } else if (lua_isnone(L, -1) == 1) {
        fmt = "";
    }

    /* Get arguments from lua function */
    /* Header color */
    const char *h1_col;
    h1_col = "\033[0m";
    /* Second header color */
    const char *h2_col;
    h2_col = "\033[0m";

    /* Get hostname */
    char hostname[255];
    gethostname(hostname, 255);

    /* Get username */
    uid_t uid = geteuid();
    struct passwd *pw = getpwuid(uid);

    printf("%s%s%s%s%s%s%s%s%s%s\n", h1_col, fmt, pw->pw_name, reset, sep_color,
           sep, reset, h2_col, hostname, reset);

    return 1;
}

/* yafetch.format() */
/* Formats given strings. */
/* Helpers function to output information */
LFUNC(format) {

    lua_getglobal(L, "yafetch");

    lua_getfield(L, -1, "sep");
    const char *sep = lua_tostring(L, -1);

    lua_getglobal(L, "yafetch");

    lua_getfield(L, -1, "sep_color");
    const char *sep_color = lua_tostring(L, -1);
    lua_pop(L, 0);

    /* Get arguments from lua function */
    /* Icon */
    const char *col_icon = lua_tostring(L, 1);
    const char *icon = lua_tostring(L, 2);

    /* Info */
    const char *col_info = lua_tostring(L, 3);
    const char *info = lua_tostring(L, 4);

    printf("%7s%s%s%s%s%s%s%s%s\n", col_icon, icon, reset, sep_color, sep,
           reset, col_info, info, reset);
    return 1;
}

/* Register functions in lua */
void func_reg(void) {
#define REG(N)                                                                 \
    lua_pushcfunction(L, lua_##N);                                             \
    lua_setfield(L, -2, #N);
    lua_newtable(L);
    REG(shell)
    REG(local_ip)
    REG(public_ip)
    REG(user)
    REG(hostname)
    REG(distro)
    REG(kernel)
    REG(cpu)
    REG(pkgs)
    REG(format)
    REG(header)
    lua_setglobal(L, "yafetch");

    luaL_newmetatable(L, "yafetch");
    lua_newtable(L);
    lua_setfield(L, -2, "__index");
    lua_pop(L, 1);
#undef REG
}
