#!/bin/bash
#
# 0002_parser_test.sh
# Test: Parser passes IPv4/IPv6 frames and drops other EtherTypes
#
# Topology:
#   e_pktgen -> veth0 -> veth1 -> [dplane+parser] -> veth2 -> veth3 -> e_pktgen(rx)
#

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Allow override via environment variables
DPLANE_BIN="${DPLANE_BIN:-${PROJECT_ROOT}/dplane/target/debug/s5-dplane}"
PKTGEN_BIN="${PKTGEN_BIN:-${PROJECT_ROOT}/test/e_pktgen/target/debug/e_pktgen}"
PARSER_WASM="${PARSER_WASM:-${PROJECT_ROOT}/test/simple_switch/parser.wasm}"

PACKET_COUNT="${PACKET_COUNT:-10}"
TIMEOUT_MS="${TIMEOUT_MS:-2000}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

log_pass() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

log_fail() {
    echo -e "${RED}[FAIL]${NC} $1"
}

cleanup() {
    log_info "Cleaning up..."

    # Stop dplane
    if [ -n "$DPLANE_PID" ]; then
        kill "$DPLANE_PID" 2>/dev/null || true
        wait "$DPLANE_PID" 2>/dev/null || true
    fi

    # Delete veth pairs
    ip link delete veth0 2>/dev/null || true
    ip link delete veth2 2>/dev/null || true

    log_info "Cleanup complete"
}

# Set trap for cleanup on exit
trap cleanup EXIT

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "This script must be run as root"
    exit 1
fi

# Check if binaries exist
if [ ! -x "$DPLANE_BIN" ]; then
    echo "dplane binary not found: $DPLANE_BIN"
    echo "Run 'cargo build' in dplane directory first"
    exit 1
fi

if [ ! -x "$PKTGEN_BIN" ]; then
    echo "e_pktgen binary not found: $PKTGEN_BIN"
    echo "Run 'cargo build' in test/e_pktgen directory first"
    exit 1
fi

if [ ! -f "$PARSER_WASM" ]; then
    echo "Parser WASM not found: $PARSER_WASM"
    echo "Run 'bash build.sh' in test/simple_switch directory first"
    exit 1
fi

echo "========================================"
echo "Test: 0002_parser_test"
echo "========================================"

# Step 1: Create veth pairs
log_info "Creating veth pairs..."
ip link add veth0 type veth peer name veth1
ip link add veth2 type veth peer name veth3
ip link set veth0 up
ip link set veth1 up
ip link set veth2 up
ip link set veth3 up
log_info "veth pairs created: veth0<->veth1, veth2<->veth3"

# Step 2: Start dplane with parser
log_info "Starting dplane with parser (veth1 -> veth2)..."
"$DPLANE_BIN" --rx veth1 --tx veth2 --parser "$PARSER_WASM" &
DPLANE_PID=$!
sleep 1

# Verify dplane is running
if ! kill -0 "$DPLANE_PID" 2>/dev/null; then
    log_fail "dplane failed to start"
    exit 1
fi
log_info "dplane started with parser (PID: $DPLANE_PID)"

# Test Case 1: IPv4 frames should pass through
log_info "Test 1: Sending $PACKET_COUNT IPv4 frames (ethertype=0800)..."
if "$PKTGEN_BIN" \
    --interface veth0 \
    --rx-interface veth3 \
    --count "$PACKET_COUNT" \
    --frame-size 60 \
    --ethertype 0800 \
    --timeout "$TIMEOUT_MS"; then
    log_pass "Test 1: IPv4 frames passed through"
else
    log_fail "Test 1: IPv4 frames should pass through"
    exit 1
fi

# Test Case 2: IPv6 frames should pass through
log_info "Test 2: Sending $PACKET_COUNT IPv6 frames (ethertype=86dd)..."
if "$PKTGEN_BIN" \
    --interface veth0 \
    --rx-interface veth3 \
    --count "$PACKET_COUNT" \
    --frame-size 60 \
    --ethertype 86dd \
    --timeout "$TIMEOUT_MS"; then
    log_pass "Test 2: IPv6 frames passed through"
else
    log_fail "Test 2: IPv6 frames should pass through"
    exit 1
fi

# Test Case 3: ARP frames should be dropped
log_info "Test 3: Sending $PACKET_COUNT ARP frames (ethertype=0806, should be dropped)..."
if "$PKTGEN_BIN" \
    --interface veth0 \
    --rx-interface veth3 \
    --count "$PACKET_COUNT" \
    --frame-size 60 \
    --ethertype 0806 \
    --expect 0 \
    --timeout "$TIMEOUT_MS"; then
    log_pass "Test 3: ARP frames were dropped"
else
    log_fail "Test 3: ARP frames should be dropped"
    exit 1
fi

# Test Case 4: Short frames (< 14 bytes) should be dropped
log_info "Test 4: Sending $PACKET_COUNT short frames (10 bytes, should be dropped)..."
if "$PKTGEN_BIN" \
    --interface veth0 \
    --rx-interface veth3 \
    --count "$PACKET_COUNT" \
    --frame-size 10 \
    --expect 0 \
    --timeout "$TIMEOUT_MS"; then
    log_pass "Test 4: Short frames were dropped"
else
    log_fail "Test 4: Short frames should be dropped"
    exit 1
fi

log_pass "0002_parser_test: All tests passed"
exit 0
