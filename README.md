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

copy jars to the jassets directory (`export J4I2PRS_BASE_PATH=/path/to/jassets`)

```bash
mkdir -p /tmp/opt/j4-i2p-rs/jassets && cp build/* /tmp/opt/j4-i2p-rs/jassets/
```

