# S5 Integration Test VM Environment
# QEMU + Ubuntu cloud image

# Configuration
VM_NAME := s5-test
VM_MEMORY := 1024
VM_CPUS := 4
VM_DISK_SIZE := 20G
SSH_PORT := 2222

# Detect host architecture
UNAME_M := $(shell uname -m)
ifeq ($(UNAME_M),arm64)
    ARCH := aarch64
    QEMU := qemu-system-aarch64
    MACHINE := virt
    CPU := host
    ACCEL := hvf
    UBUNTU_IMAGE := ubuntu-24.04-server-cloudimg-arm64.img
    UBUNTU_URL := https://cloud-images.ubuntu.com/releases/24.04/release/ubuntu-24.04-server-cloudimg-arm64.img
else ifeq ($(UNAME_M),aarch64)
    ARCH := aarch64
    QEMU := qemu-system-aarch64
    MACHINE := virt
    CPU := host
    ACCEL := kvm
    UBUNTU_IMAGE := ubuntu-24.04-server-cloudimg-arm64.img
    UBUNTU_URL := https://cloud-images.ubuntu.com/releases/24.04/release/ubuntu-24.04-server-cloudimg-arm64.img
else
    ARCH := x86_64
    QEMU := qemu-system-x86_64
    MACHINE := q35
    CPU := host
    # Detect accelerator
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Darwin)
        ACCEL := hvf
    else
        ACCEL := kvm
    endif
    UBUNTU_IMAGE := ubuntu-24.04-server-cloudimg-amd64.img
    UBUNTU_URL := https://cloud-images.ubuntu.com/releases/24.04/release/ubuntu-24.04-server-cloudimg-amd64.img
endif

# Directories
VM_DIR := $(shell pwd)
IMAGE_DIR := $(VM_DIR)/images
CLOUD_INIT_DIR := $(VM_DIR)/cloud-init

# Files
BASE_IMAGE := $(IMAGE_DIR)/$(UBUNTU_IMAGE)
VM_IMAGE := $(IMAGE_DIR)/$(VM_NAME).qcow2
CLOUD_INIT_ISO := $(IMAGE_DIR)/cloud-init.iso
PID_FILE := $(VM_DIR)/.vm.pid

# SSH key
SSH_KEY := $(VM_DIR)/.ssh/id_ed25519

.PHONY: all download create-image start stop ssh status clean help setup-ssh

all: help

## Download Ubuntu cloud image
download: $(BASE_IMAGE)

$(BASE_IMAGE):
	@mkdir -p $(IMAGE_DIR)
	@echo "Downloading Ubuntu 24.04 cloud image ($(ARCH))..."
	curl -L -o $(BASE_IMAGE) $(UBUNTU_URL)

## Create VM disk image from base
create-image: $(VM_IMAGE)

$(VM_IMAGE): $(BASE_IMAGE)
	@echo "Creating VM disk image..."
	qemu-img create -f qcow2 -F qcow2 -b $(BASE_IMAGE) $(VM_IMAGE) $(VM_DISK_SIZE)

## Create cloud-init ISO
$(CLOUD_INIT_ISO): $(CLOUD_INIT_DIR)/user-data $(CLOUD_INIT_DIR)/meta-data
	@echo "Creating cloud-init ISO..."
	@mkdir -p $(IMAGE_DIR)
	mkisofs -output $(CLOUD_INIT_ISO) -volid cidata -joliet -rock $(CLOUD_INIT_DIR)

## Start VM
start: $(VM_IMAGE) $(CLOUD_INIT_ISO)
	@if [ -f $(PID_FILE) ] && kill -0 $$(cat $(PID_FILE)) 2>/dev/null; then \
		echo "VM is already running (PID: $$(cat $(PID_FILE)))"; \
		exit 1; \
	fi
	@echo "Starting VM ($(ARCH))..."
ifeq ($(ARCH),aarch64)
	$(QEMU) \
		-name $(VM_NAME) \
		-machine $(MACHINE) \
		-accel $(ACCEL) \
		-cpu $(CPU) \
		-m $(VM_MEMORY) \
		-smp $(VM_CPUS) \
		-drive if=virtio,file=$(VM_IMAGE),format=qcow2 \
		-drive if=virtio,file=$(CLOUD_INIT_ISO),format=raw \
		-device virtio-net-pci,netdev=net0 \
		-netdev user,id=net0,hostfwd=tcp::$(SSH_PORT)-:22 \
		-device virtio-net-pci,netdev=net1 \
		-netdev user,id=net1 \
		-serial null \
		-monitor none \
		-display none \
		-bios /opt/homebrew/share/qemu/edk2-aarch64-code.fd \
		-daemonize \
		-pidfile $(PID_FILE) \
		|| (rm -f $(PID_FILE) && exit 1)
else
	$(QEMU) \
		-name $(VM_NAME) \
		-machine $(MACHINE) \
		-accel $(ACCEL) \
		-cpu $(CPU) \
		-m $(VM_MEMORY) \
		-smp $(VM_CPUS) \
		-drive if=virtio,file=$(VM_IMAGE),format=qcow2 \
		-drive if=virtio,file=$(CLOUD_INIT_ISO),format=raw \
		-device virtio-net-pci,netdev=net0 \
		-netdev user,id=net0,hostfwd=tcp::$(SSH_PORT)-:22 \
		-device virtio-net-pci,netdev=net1 \
		-netdev user,id=net1 \
		-serial null \
		-monitor none \
		-display none \
		-daemonize \
		-pidfile $(PID_FILE) \
		|| (rm -f $(PID_FILE) && exit 1)
endif
	@echo "VM started. SSH port: $(SSH_PORT)"
	@echo "Waiting for VM to boot (this may take a minute)..."
	@sleep 5
	@echo "Use 'make ssh' to connect"

## Stop VM
stop:
	@if [ -f $(PID_FILE) ]; then \
		PID=$$(cat $(PID_FILE)); \
		if kill -0 $$PID 2>/dev/null; then \
			echo "Stopping VM (PID: $$PID)..."; \
			kill $$PID; \
			rm -f $(PID_FILE); \
			echo "VM stopped"; \
		else \
			echo "VM process not found, cleaning up PID file"; \
			rm -f $(PID_FILE); \
		fi \
	else \
		echo "VM is not running"; \
	fi

## SSH into VM
ssh:
	@echo "Connecting to VM..."
	ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -i $(VM_DIR)/.ssh/id_ed25519 -p $(SSH_PORT) s5@localhost

## Show VM status
status:
	@if [ -f $(PID_FILE) ] && kill -0 $$(cat $(PID_FILE)) 2>/dev/null; then \
		echo "VM is running (PID: $$(cat $(PID_FILE)))"; \
	else \
		echo "VM is not running"; \
	fi

## Clean up all generated files
clean: stop
	@echo "Cleaning up..."
	rm -f $(VM_IMAGE)
	rm -f $(CLOUD_INIT_ISO)
	rm -f $(PID_FILE)

## Clean up everything including base image
clean-all: clean
	rm -rf $(IMAGE_DIR)

## Generate SSH key pair
setup-ssh: $(SSH_KEY)

$(SSH_KEY):
	@echo "Generating SSH key pair..."
	@mkdir -p $(VM_DIR)/.ssh
	ssh-keygen -t ed25519 -f $(SSH_KEY) -N "" -C "s5-test-vm"
	@echo ""
	@echo "Add this public key to cloud-init/user-data:"
	@cat $(SSH_KEY).pub

## Show help
help:
	@echo "S5 Integration Test VM"
	@echo ""
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@echo "  download     Download Ubuntu cloud image"
	@echo "  start        Start VM"
	@echo "  stop         Stop VM"
	@echo "  ssh          SSH into VM"
	@echo "  status       Show VM status"
	@echo "  clean        Remove VM image and cloud-init ISO"
	@echo "  clean-all    Remove all images including base"
	@echo "  setup-ssh    Generate SSH key pair"
	@echo ""
	@echo "Configuration:"
	@echo "  Architecture: $(ARCH)"
	@echo "  Memory:       $(VM_MEMORY) MB"
	@echo "  CPUs:         $(VM_CPUS)"
	@echo "  Disk:         $(VM_DISK_SIZE)"
	@echo "  SSH Port:     $(SSH_PORT)"
