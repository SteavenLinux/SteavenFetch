echo "DE: $XDG_SESSION_DESKTOP"
echo "DS: $XDG_SESSION_TYPE"
echo "Shell: $0"
echo -n "Kernel: " && uname -r
echo -n "CPU: " && lscpu | grep "Model name:" | sed -r 's/Model name:\s{1,}//g'
echo -n "GPU: " && lspci | grep -i VGA
echo -n "RAM: " && grep MemTotal /proc/meminfo
