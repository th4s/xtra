// Mount chaindata: sshfs thojest@giga:/var/lib/geth/geth/chaindata/ancient ~/chaindata_ancient/ -o ro,reconnect
// Unmount: fusermount -u ~/chaindata_ancient

pub mod freezer;
