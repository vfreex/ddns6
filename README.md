# DDNS6 - One dynamic DNS client for multiple hosts

DDNS6 is a dynamic DNS client for updating dynamic DNS entries for multiple hosts.

*Note: The project is currently in an early stage of development. Don't use in production!*

DDNS6 supports updating dynamic DNS entries for accounts on many dynamic DNS services.
It has the following great advantages:

- Built for IPv4/IPv6 dual-stack.
- One instance for multiple hosts. DDNS6 supports configuring address prefix and suffix separately and combining them together to batch update dynamic DNS entries for multiple hosts.
- Cross-platform. Running on Linux, OpenWrt/LEDE, EdgeOS/Vyatta, Windows, and macOS.
- Great performance with low resource cost by being native and using asynchronous I/O.
