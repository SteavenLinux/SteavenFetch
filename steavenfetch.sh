echo "DE: $XDG_SESSION_DESKTOP"
echo "DS: $XDG_SESSION_TYPE"
echo "Shell: $0"
lscpu | grep "Model name:" | sed -r 's/Model name:\s{1,}//g'
lspci | grep -i VGA
grep MemTotal /proc/meminfo
