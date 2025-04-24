APPDIR=apps
BINDIR=bin

APPS=$(wildcard $(APPDIR)/*.c)
BINS=$(APPS:$(APPDIR)/%.c=$(BINDIR)/%.wasm)

WASI_SDK=/opt/wasi-sdk

CC=$(WASI_SDK)/bin/clang


HOST_CC=gcc
HOST_BINDIR=$(BINDIR)/host
HOST_BINS=$(APPS:$(APPDIR)/%.c=$(HOST_BINDIR)/%.o)

INSTRUMENT_DIR=wasm-instrument
INSTRUMENT_BIN=$(INSTRUMENT_DIR)/instrument

.PHONY: $(INSTRUMENT_BIN) apps clean clean-instrument $(BINDIR)

apps: $(HOST_BINS) $(BINS)
	@echo "Built: $(BINS)"

$(BINDIR):
	mkdir -p $(BINDIR)
	mkdir -p $(HOST_BINDIR)

$(INSTRUMENT_BIN):
	make -C wasm-instrument


.PRECIOUS: $(HOST_BINDIR)/%.o
$(HOST_BINDIR)/%.o: $(APPDIR)/%.c | $(BINDIR)
	$(HOST_CC) $< -o $@

.PRECIOUS: $(BINDIR)/%.wat
$(BINDIR)/%.wasm: $(APPDIR)/%.c | $(BINDIR) $(INSTRUMENT_BIN)
	$(CC) $< -o $@
	$(INSTRUMENT_BIN) --scheme=snapshot -o $@ $@
	wasm2wat $@ -o $(@:.wasm=.wat)


clean:
	rm -rf $(BINDIR)

clean-instrument:
	make -C $(INSTRUMENT_DIR) clean
