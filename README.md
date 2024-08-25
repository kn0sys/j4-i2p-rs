[![build](https://github.com/kn0sys/j4-i2p-rs/actions/workflows/build.yml/badge.svg)](https://github.com/kn0sys/j4-i2p-rs/actions/workflows/build.yml)

# j4-i2p-rs

i2p rust wrapper


## Getting Started

```bash
git clone --recursive https://github.com/kn0sys/j4-i2p-rs
```

install dependencies

```bash
sudo apt install ant gettext
```
build the jars

```bash
cd i2p && ant buildRouter buildI2PTunnelJars buildSAM jbigi buildAddressbook
```

copy jars to the jassets directory

```bash
mkdir -p ../opt/j4-i2p-rs/jassets && cp build/* ../opt/j4-i2p-rs/jassets/
```

