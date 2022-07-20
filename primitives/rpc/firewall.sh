#!/bin/bash
# Delete old /usr/bin/flock /var/lock/iptables -c "/sbin/iptables rules"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -P OUTPUT ACCEPT"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -P INPUT ACCEPT"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -P FORWARD ACCEPT"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -F"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -t nat -F PREROUTING"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -t nat -F POSTROUTING"

#configuration of default policies
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -P OUTPUT ACCEPT"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -P INPUT DROP   "
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -P FORWARD DROP"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -A INPUT -s 127.0.0.1 -j ACCEPT"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -A INPUT -p tcp --dport ssh -j ACCEPT"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -A INPUT -p tcp --dport 30333 -j ACCEPT"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -A INPUT -p tcp --dport 30334 -j ACCEPT"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -A INPUT -p tcp --dport 443 -j ACCEPT"


# Keep state of connections
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -A OUTPUT -m state --state NEW -o eth0 -j ACCEPT"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -A FORWARD -m state --state NEW -o eth0 -j ACCEPT"
/usr/bin/flock /var/lock/iptables -c "/sbin/iptables -A FORWARD -m state --state ESTABLISHED,RELATED -j ACCEPT"
