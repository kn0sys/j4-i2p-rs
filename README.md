# j4-i2p-rs

i2p rust wrapper


## Getting Started

`git clone --recursive https://github.com/kn0sys/j4-i2p-rs`

build the jars

`cd i2p`
`ant buildRouter buildI2PTunnelJars buildSAM jbigi buildAddressbook`

copy jars to the jassets directory


`cd build`
`mkidr ../../opt/j4-i2p-rs/jassets`
`for i in addressbook.jar i2ptunnel.jar i2p.jar mstreaming.jar router.jar sam.jar streaming.jar; do cp "$i" "../../opt/j4-i2p-rs/jassets"; done`

