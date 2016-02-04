-include ../tools.mk

REPLACEMENT := s/[0-9][0-9]*\#[0-9][0-9]*//g

all:
	# pretty-expand both input.rs and input_pp.rs
	$(RUSTC) -o $(TMPDIR)/input.pretty -Z unstable-options \
		--pretty expanded,hygiene input.rs
	$(RUSTC) -o $(TMPDIR)/input_pp.pretty -Z unstable-options \
		--pretty expanded,hygiene input_pp.rs

	# the name/ctxt numbers are very internals-dependent and thus
	# change relatively frequently, and testing for their exact values
	# them will fail annoyingly, so we just remove them
	#
	# (These need to be out-of-place because OSX/BSD & GNU sed
	# differ.)
	sed "$(REPLACEMENT)" $(TMPDIR)/input.pretty    > $(TMPDIR)/input.replaced
	sed "$(REPLACEMENT)" $(TMPDIR)/input_pp.pretty > $(TMPDIR)/input_pp.replaced

	diff -u $(TMPDIR)/input.replaced $(TMPDIR)/input_pp.replaced
