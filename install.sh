#!/usr/bin/env bash
set -euo pipefail


reset="\033[0m"
red="\033[31m"
green="\033[32m"
yellow="\033[33m"
white="\033[37m"

info="${green}?${reset}"
success="${green}✔${reset}"
error="${red}✖${reset}"
warning="${yellow}⚠${reset}"

# Check if the script is run as root

echo -e "$info Checking if the script is run as root $reset"
if [ "$(id -u)" != "0" ]; then
    echo -e "$info Installi ng as non-root $reset" 1>&2
    
    # else
    
fi


RELEASE_URL="https://api.github.com/repos/lnxcz/leap/releases/latest"






detect_profile() {
    if [ -n "${PROFILE}" ] && [ -f "${PROFILE}" ]; then
        echo "${PROFILE}"
        return
    fi
    
    local DETECTED_PROFILE
    DETECTED_PROFILE=''
    local SHELLTYPE
    SHELLTYPE="$(basename "/$SHELL")"
    
    if [ "$SHELLTYPE" = "bash" ]; then
        if [ -f "$HOME/.bashrc" ]; then
            DETECTED_PROFILE="$HOME/.bashrc"
            elif [ -f "$HOME/.bash_profile" ]; then
            DETECTED_PROFILE="$HOME/.bash_profile"
        fi
        elif [ "$SHELLTYPE" = "zsh" ]; then
        DETECTED_PROFILE="$HOME/.zshrc"
        elif [ "$SHELLTYPE" = "fish" ]; then
        DETECTED_PROFILE="$HOME/.config/fish/config.fish"
    fi
    
    if [ -z "$DETECTED_PROFILE" ]; then
        if [ -f "$HOME/.profile" ]; then
            DETECTED_PROFILE="$HOME/.profile"
            elif [ -f "$HOME/.bashrc" ]; then
            DETECTED_PROFILE="$HOME/.bashrc"
            elif [ -f "$HOME/.bash_profile" ]; then
            DETECTED_PROFILE="$HOME/.bash_profile"
            elif [ -f "$HOME/.zshrc" ]; then
            DETECTED_PROFILE="$HOME/.zshrc"
            elif [ -f "$HOME/.config/fish/config.fish" ]; then
            DETECTED_PROFILE="$HOME/.config/fish/config.fish"
        fi
    fi
    
    if [ ! -z "$DETECTED_PROFILE" ]; then
        echo "$DETECTED_PROFILE"
    fi
}


detect_arch() {
    local ARCH
    ARCH="$(uname -m)"
    if [ "$ARCH" = "x86_64" ]; then
        echo "amd64"
        elif [ "$ARCH" = "i386" ] || [ "$ARCH" = "i686" ]; then
        echo "386"
        elif [ "$ARCH" = "armv5*" ] || [ "$ARCH" = "armv6*" ] || [ "$ARCH" = "armv7*" ]; then
        echo "arm"
        elif [ "$ARCH" = "aarch64" ]; then
        echo "arm64"
        elif [ "$ARCH" = "ppc64le" ]; then
        echo "ppc64le"
        elif [ "$ARCH" = "s390x" ]; then
        echo "s390x"
    else
        echo "unknown"
    fi
}

