#!/bin/bash

# Mount chaindata: sshfs giga:/var/lib/geth/geth/chaindata/ancient /home/thojest/chaindata_ancient/ -o reconnect,ro

rif() {
    tmpvar=$(od -v -A d -t x1 -j $((6 * $2)) -N 6 --endian big $1 | head -n1 |cut -d" " -f2-3 | tr -d " ")
    printf "%d\n" 0x${tmpvar}
}

rio() {
    tmpvar=$(od -v -A d -t x1 -j $((6 * $2)) -N 6 --endian big $1 | head -n1 |cut -d" " -f4-7 | tr -d " ")
    printf "%d\n" 0x${tmpvar}
}

refor() {
    tmpfilevar=$(rif $1 $2)
    tmpoffsetvar=$(rio $1 $2)
    tmpoffsetvar2=$(rio $1 $(($2 + 1)))
    echo "File: ${tmpfilevar} Start: ${tmpoffsetvar} Length: $((${tmpoffsetvar2} - ${tmpoffsetvar}))"
}

rb() {
    od -v -A d -t x1 --endian big -j $2 -N $3 $1
}

rbh() {
    od -v -A d -t c --endian big -j $2 -N $3 $1
}

wb() {
    dd if=$1 bs=1 skip=$2 count=$3
}

rbu() {
    wb $1 $2 $3 | snzip -d -t raw -c | od -v -A d -t x1 --endian big 
}
