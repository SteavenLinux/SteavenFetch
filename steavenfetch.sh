echo "OS: Linux"
echo -n "Linux Distro: " && grep -E '^(VERSION|NAME)=' /etc/os-release
echo -n "Kernel: " && uname -r
echo "DE: $XDG_SESSION_DESKTOP"
echo "DS: $XDG_SESSION_TYPE"
echo "Shell: $0"
echo ""
echo -n "Model: " && sudo dmidecode -s system-product-name
echo -n "Serial Number: " && sudo dmidecode -s system-serial-number
echo -n "CPU: " && lscpu | grep "Model name:" | sed -r 's/Model name:\s{1,}//g'
echo -n "GPU: " && lspci | grep -i VGA
echo -n "RAM: " && grep MemTotal /proc/meminfo
echo ""
echo -n "Uptime: " && uptime -p
