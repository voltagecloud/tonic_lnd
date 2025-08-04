# LND Configuration
LND_VERSION ?= v0.19.2-beta
LND_REPO_RAW_URL := https://raw.githubusercontent.com/lightningnetwork/lnd/$(LND_VERSION)/lnrpc

# Taproot Assets Configuration
TAPROOT_VERSION ?= v0.6.1
TAPROOT_REPO_RAW_URL := https://raw.githubusercontent.com/lightninglabs/taproot-assets/$(TAPROOT_VERSION)/taprpc

VENDOR_DIR := vendor

# LND Proto Configuration
LND_PROTO_DIRS := invoicesrpc peersrpc routerrpc signrpc verrpc walletrpc
LND_ROOT_PROTOS := lightning.proto stateservice.proto
LND_PROTO_NAMES := invoicesrpc/invoices.proto peersrpc/peers.proto routerrpc/router.proto signrpc/signer.proto verrpc/verrpc.proto walletrpc/walletkit.proto

# Taproot Assets Proto Configuration
TAPROOT_PROTO_DIRS := assetwalletrpc mintrpc priceoraclerpc rfqrpc tapchannelrpc tapdevrpc universerpc
TAPROOT_ROOT_PROTOS := taprootassets.proto
TAPROOT_PROTO_NAMES := assetwalletrpc/assetwallet.proto mintrpc/mint.proto priceoraclerpc/price_oracle.proto rfqrpc/rfq.proto tapchannelrpc/tapchannel.proto tapdevrpc/tapdev.proto universerpc/universe.proto

# Combined Proto Targets
LND_SUB_PROTOS := $(addprefix $(VENDOR_DIR)/, $(LND_PROTO_NAMES))
TAPROOT_SUB_PROTOS := $(addprefix $(VENDOR_DIR)/, $(TAPROOT_PROTO_NAMES))

LND_TARGET_PROTOS := $(addprefix $(VENDOR_DIR)/, $(LND_ROOT_PROTOS)) $(LND_SUB_PROTOS)
TAPROOT_TARGET_PROTOS := $(addprefix $(VENDOR_DIR)/, $(TAPROOT_ROOT_PROTOS)) $(TAPROOT_SUB_PROTOS)

TARGET_PROTOS := $(LND_TARGET_PROTOS) $(TAPROOT_TARGET_PROTOS)

.PHONY: all clean fetch-protos fetch-lnd-protos fetch-taproot-protos lint fmt clippy machete

all: fetch-protos

fetch-protos: fetch-lnd-protos fetch-taproot-protos

fetch-lnd-protos: $(LND_TARGET_PROTOS)

fetch-taproot-protos: $(TAPROOT_TARGET_PROTOS)

# LND Root Proto Rules
LND_ROOT_PROTO_TARGETS := $(addprefix $(VENDOR_DIR)/, $(LND_ROOT_PROTOS))

$(LND_ROOT_PROTO_TARGETS):
	@echo "Fetching $(@) from LND lnrpc/..."
	@mkdir -p $(VENDOR_DIR)
	@curl -sSfL -o $@ "$(LND_REPO_RAW_URL)/$(@F)"

# Taproot Assets Root Proto Rules
TAPROOT_ROOT_PROTO_TARGETS := $(addprefix $(VENDOR_DIR)/, $(TAPROOT_ROOT_PROTOS))

$(TAPROOT_ROOT_PROTO_TARGETS):
	@echo "Fetching $(@) from Taproot Assets taprpc/..."
	@mkdir -p $(VENDOR_DIR)
	@curl -sSfL -o $@ "$(TAPROOT_REPO_RAW_URL)/$(@F)"

# LND Subdirectory Proto Rules
define LND_SUBDIR_RULE_TEMPLATE
$(VENDOR_DIR)/$(1)/%.proto:
	@echo "Fetching $$@ from LND lnrpc/$(1)/..."
	@mkdir -p $$(@D)
	@curl -sSfL -o $$@ "$(LND_REPO_RAW_URL)/$(1)/$$(@F)"
endef

$(foreach dir,$(LND_PROTO_DIRS),$(eval $(call LND_SUBDIR_RULE_TEMPLATE,$(dir))))

# Taproot Assets Subdirectory Proto Rules
define TAPROOT_SUBDIR_RULE_TEMPLATE
$(VENDOR_DIR)/$(1)/%.proto:
	@echo "Fetching $$@ from Taproot Assets taprpc/$(1)/..."
	@mkdir -p $$(@D)
	@curl -sSfL -o $$@ "$(TAPROOT_REPO_RAW_URL)/$(1)/$$(@F)"
endef

$(foreach dir,$(TAPROOT_PROTO_DIRS),$(eval $(call TAPROOT_SUBDIR_RULE_TEMPLATE,$(dir))))

clean:
	@echo "Cleaning vendor directory..."
	@rm -rf $(VENDOR_DIR)

lint: fmt clippy machete

fmt:
	cargo +nightly fmt --all --check

clippy:
	cargo clippy --all-targets --all-features --locked

machete:
	cargo machete --with-metadata

# Example usage:
# make fetch-protos LND_VERSION=v0.17.0-beta TAPROOT_VERSION=v0.5.0
# make fetch-lnd-protos LND_VERSION=v0.17.0-beta
# make fetch-taproot-protos TAPROOT_VERSION=v0.5.0
# make clean
