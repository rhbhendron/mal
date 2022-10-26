##
#  Jan's Rust Implementation
#
# @file
# @version 0.1

STEPS = step0_repl

all: $(STEPS)

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@


# end
