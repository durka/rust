-include ../tools.mk

all:
	# pretty-expand both input.rs and input_pp.rs
	$(RUSTC) -o $(TMPDIR)/input.pretty -Z unstable-options \
		--pretty expanded input.rs
	$(RUSTC) -o $(TMPDIR)/input_pp.pretty -Z unstable-options \
		--pretty expanded input_pp.rs

	diff -u $(TMPDIR)/input.pretty $(TMPDIR)/input_pp.pretty
