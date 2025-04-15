APPDIR=apps
BINDIR=bin

APPS=$(wildcard $(APPDIR)/*.c)
BINS=$(APPS:$(APPDIR)/%.c=$(BINDIR)/%.wasm)

WASI_SDK=/opt/wasi-sdk

CC=$(WASI_SDK)/bin/clang

HOST_CC=gcc
HOST_BINDIR=$(BINDIR)/host
HOST_BINS=$(APPS:$(APPDIR)/%.c=$(HOST_BINDIR)/%.o)

.PHONY: apps clean $(BINDIR)

apps: $(HOST_BINS) $(BINS)
	@echo "Built: $(BINS)"

$(BINDIR):
	mkdir -p $(BINDIR)
	mkdir -p $(HOST_BINDIR)

.PRECIOUS: $(HOST_BINDIR)/%.o
$(HOST_BINDIR)/%.o: $(APPDIR)/%.c | $(BINDIR)
	$(HOST_CC) $< -o $@

.PRECIOUS: $(BINDIR)/%.wat
$(BINDIR)/%.wasm: $(APPDIR)/%.c | $(BINDIR)
	$(CC) $< -o $@
	./instrument --scheme=snapshot -o $@ $@
	wasm2wat $@ -o $(@:.wasm=.wat)


clean:
	rm -rf $(BINDIR)
