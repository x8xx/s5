#!/bin/bash
#
# deploy-and-test.sh
# Build, deploy to VM, and run tests
#
# Usage:
#   ./deploy-and-test.sh [test_script]
#
# Examples:
#   ./deploy-and-test.sh                    # Run all tests (0001, 0002, ...)
#   ./deploy-and-test.sh 0001               # Run specific test
#   ./deploy-and-test.sh 0002               # Run specific test
#

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
VM_DIR="$PROJECT_ROOT/test/vm"

# VM SSH configuration
SSH_KEY="$VM_DIR/.ssh/id_ed25519"
SSH_PORT=2222
SSH_USER="s5"
SSH_HOST="localhost"
SSH_OPTS="-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -o LogLevel=ERROR"

# Remote directories
REMOTE_HOME="/home/$SSH_USER"
REMOTE_DPLANE="$REMOTE_HOME/dplane"
REMOTE_PKTGEN="$REMOTE_HOME/e_pktgen"
REMOTE_SCRIPTS="$REMOTE_HOME/scripts"
REMOTE_PARSER="$REMOTE_HOME/parser.wasm"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_pass() { echo -e "${GREEN}[PASS]${NC} $1"; }
log_fail() { echo -e "${RED}[FAIL]${NC} $1"; }
log_step() { echo -e "${YELLOW}==>${NC} $1"; }

ssh_cmd() {
    ssh $SSH_OPTS -i "$SSH_KEY" -p "$SSH_PORT" "$SSH_USER@$SSH_HOST" "source ~/.cargo/env 2>/dev/null; $*"
}

scp_to_vm() {
    scp $SSH_OPTS -i "$SSH_KEY" -P "$SSH_PORT" -r "$1" "$SSH_USER@$SSH_HOST:$2"
}

# Check prerequisites
check_prereqs() {
    log_step "Checking prerequisites..."

    if [ ! -f "$SSH_KEY" ]; then
        log_fail "SSH key not found: $SSH_KEY"
        echo "Run 'cd test/vm && make setup-ssh' first"
        exit 1
    fi

    # Check if VM is running
    if ! ssh_cmd "echo ok" >/dev/null 2>&1; then
        log_fail "Cannot connect to VM"
        echo "Run 'cd test/vm && make start' first"
        exit 1
    fi

    log_info "VM is accessible"
}

# Build WASM locally (can be done on macOS)
build_wasm() {
    log_step "Building WASM parser..."
    (cd "$PROJECT_ROOT/test/simple_switch" && bash build.sh 2>&1 | tail -3)
    log_info "WASM build complete"
}

# Deploy source to VM
deploy_source() {
    log_step "Deploying source to VM..."

    # Create remote directories
    ssh_cmd "mkdir -p $REMOTE_DPLANE $REMOTE_PKTGEN $REMOTE_SCRIPTS"

    # Copy dplane source
    log_info "Copying dplane source..."
    scp_to_vm "$PROJECT_ROOT/dplane/Cargo.toml" "$REMOTE_DPLANE/"
    scp_to_vm "$PROJECT_ROOT/dplane/Cargo.lock" "$REMOTE_DPLANE/"
    scp_to_vm "$PROJECT_ROOT/dplane/src" "$REMOTE_DPLANE/"

    # Copy e_pktgen source
    log_info "Copying e_pktgen source..."
    scp_to_vm "$PROJECT_ROOT/test/e_pktgen/Cargo.toml" "$REMOTE_PKTGEN/"
    scp_to_vm "$PROJECT_ROOT/test/e_pktgen/Cargo.lock" "$REMOTE_PKTGEN/"
    scp_to_vm "$PROJECT_ROOT/test/e_pktgen/src" "$REMOTE_PKTGEN/"

    # Copy WASM parser
    log_info "Copying parser.wasm..."
    scp_to_vm "$PROJECT_ROOT/test/simple_switch/parser.wasm" "$REMOTE_PARSER"

    # Copy test scripts
    log_info "Copying test scripts..."
    for script in "$PROJECT_ROOT/test/scripts/"0*.sh; do
        scp_to_vm "$script" "$REMOTE_SCRIPTS/"
    done

    log_info "Deployment complete"
}

# Build on VM with limited parallelism to avoid OOM
build_on_vm() {
    log_step "Building on VM (this may take a while)..."

    # Build with -j1 to reduce memory usage
    log_info "Building dplane..."
    ssh_cmd "cd $REMOTE_DPLANE && CARGO_BUILD_JOBS=1 cargo build --release 2>&1" | tail -10

    log_info "Building e_pktgen..."
    ssh_cmd "cd $REMOTE_PKTGEN && CARGO_BUILD_JOBS=1 cargo build --release 2>&1" | tail -10

    log_info "VM build complete"
}

# Run tests
run_tests() {
    local test_filter="$1"

    log_step "Running tests..."

    local dplane_bin="$REMOTE_DPLANE/target/release/s5-dplane"
    local pktgen_bin="$REMOTE_PKTGEN/target/release/e_pktgen"
    local parser_wasm="$REMOTE_PARSER"

    # Find test scripts
    local tests
    if [ -n "$test_filter" ]; then
        tests=$(ssh_cmd "ls $REMOTE_SCRIPTS/${test_filter}*.sh 2>/dev/null || true")
    else
        tests=$(ssh_cmd "ls $REMOTE_SCRIPTS/0*.sh 2>/dev/null || true")
    fi

    if [ -z "$tests" ]; then
        log_fail "No test scripts found"
        exit 1
    fi

    local passed=0
    local failed=0

    for test in $tests; do
        local test_name=$(basename "$test")
        log_info "Running $test_name..."

        if ssh_cmd "sudo DPLANE_BIN=$dplane_bin PKTGEN_BIN=$pktgen_bin PARSER_WASM=$parser_wasm bash $test"; then
            log_pass "$test_name"
            ((passed++))
        else
            log_fail "$test_name"
            ((failed++))
        fi
        echo ""
    done

    echo "========================================"
    echo "Results: $passed passed, $failed failed"
    echo "========================================"

    if [ "$failed" -gt 0 ]; then
        exit 1
    fi
}

# Main
main() {
    local test_filter="$1"

    echo "========================================"
    echo "S5 Deploy and Test"
    echo "========================================"
    echo ""

    check_prereqs
    build_wasm
    deploy_source
    build_on_vm
    run_tests "$test_filter"
}

main "$@"
