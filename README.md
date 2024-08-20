# j4-i2p-rs

i2p rust wrapper


## Getting Started

```bash
git clone --recursive https://github.com/kn0sys/j4-i2p-rs
```

build the jars

```bash
cd i2p
```

```bash
ant buildRouter buildI2PTunnelJars buildSAM jbigi buildAddressbook
```

copy jars to the jassets directory


```bash
cd build
```

```bash
mkidr -p ../../opt/j4-i2p-rs/jassets
```

```bash
for i in addressbook.jar i2ptunnel.jar i2p.jar mstreaming.jar router.jar sam.jar streaming.jar; do cp "$i" "../../opt/j4-i2p-rs/jassets"; done
```

