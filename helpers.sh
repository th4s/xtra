#!/bin/bash

# Mount chaindata: sshfs giga:/var/lib/geth/geth/chaindata/ancient /home/th4s/chaindata_ancient/ -o reconnect,ro

# Reads the file number for block number X from an index file
# e.g rif ~/chaindata_ancient/headers.cidx 1
rif() {
    tmpvar=$(od -v -A d -t x1 -j $((6 * $2)) -N 6 --endian big $1 | head -n1 |cut -d" " -f2-3 | tr -d " ")
    printf "%d\n" 0x${tmpvar}
}

# Reads the byte offset for blcknumber X from an index file
# e.g rio ~/chaindata_ancient/headers.cidx 1
rio() {
    tmpvar=$(od -v -A d -t x1 -j $((6 * $2)) -N 6 --endian big $1 | head -n1 |cut -d" " -f4-7 | tr -d " ")
    printf "%d\n" 0x${tmpvar}
}

# Reads the file number offset and length for a block number X from an index file
# e.g refor ~/chaindata_ancient/headers.cidx 1
refor() {
    tmpfilevar=$(rif $1 $2)
    tmpoffsetvar=$(rio $1 $2)
    tmpoffsetvar2=$(rio $1 $(($2 + 1)))
    echo "File: ${tmpfilevar} Start: ${tmpoffsetvar} Length: $((${tmpoffsetvar2} - ${tmpoffsetvar}))"
}

# Reads n bytes from a file and skips the first m bytes. Converts to hex
# e.g. rb ~/chaindata_ancient/headers.0000.cdat m n
rb() {
    od -v -A d -t x1 --endian big -j $2 -N $3 $1
}

# Reads n bytes from a file and skips the first m bytes. Attempts to convert to UTF-8
# e.g. rbh ~/chaindata_ancient/headers.0000.cdat m n
rbh() {
    od -v -A d -t c --endian big -j $2 -N $3 $1
}

# Writes n bytes from a file to stdout and skips the first m bytes
# e.g. wb ~/chaindata_ancient/headers.0000.cdat m n
wb() {
    dd if=$1 bs=1 skip=$2 count=$3
}

# Reads n bytes from a file and skips the first m bytes. Decompresses snappy and converts to hex
# e.g. rbu ~/chaindata_ancient/headers.0000.cdat m n
rbu() {
    wb $1 $2 $3 | snzip -d -t raw -c | od -v -A d -t x1 --endian big 
}

# Reads n bytes from a file and skips the first m bytes. Decompresses snappy and attempts to convert to UTF-8
# e.g. rb ~/chaindata_ancient/headers.0000.cdat m n
rbuh() {
    wb $1 $2 $3 | snzip -d -t raw -c | od -v -A d -t c --endian big 
}
