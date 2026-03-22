#!/bin/bash
set -e

DPDK_VERSION="22.03"
DPDK_DIR="/opt/dpdk"

echo "=== DPDK ${DPDK_VERSION} Setup Script ==="

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root (sudo)"
    exit 1
fi

# Download and extract DPDK
echo "[1/4] Downloading DPDK ${DPDK_VERSION}..."
cd /tmp
if [ ! -f "dpdk-${DPDK_VERSION}.tar.xz" ]; then
    wget https://fast.dpdk.org/rel/dpdk-${DPDK_VERSION}.tar.xz
fi
tar xf dpdk-${DPDK_VERSION}.tar.xz

# Build DPDK
echo "[2/4] Building DPDK..."
cd dpdk-${DPDK_VERSION}
meson setup build --prefix=${DPDK_DIR}
cd build
ninja
ninja install

# Configure environment
echo "[3/4] Configuring environment..."
cat > /etc/profile.d/dpdk.sh << 'EOF'
export DPDK_DIR=/opt/dpdk
export PKG_CONFIG_PATH=$DPDK_DIR/lib/x86_64-linux-gnu/pkgconfig:$PKG_CONFIG_PATH
export LD_LIBRARY_PATH=$DPDK_DIR/lib/x86_64-linux-gnu:$LD_LIBRARY_PATH
EOF

# Update ldconfig
echo "${DPDK_DIR}/lib/x86_64-linux-gnu" > /etc/ld.so.conf.d/dpdk.conf
ldconfig

# Verify hugepages
echo "[4/4] Verifying hugepages..."
grep -i huge /proc/meminfo

echo ""
echo "=== DPDK Setup Complete ==="
echo "DPDK installed to: ${DPDK_DIR}"
echo "Run 'source /etc/profile.d/dpdk.sh' to update environment"
echo ""
echo "To bind virtio interfaces to DPDK (after reboot):"
echo "  ${DPDK_DIR}/sbin/dpdk-devbind.py --status"
echo "  ${DPDK_DIR}/sbin/dpdk-devbind.py -b vfio-pci <pci_addr>"
