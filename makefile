TESTS = test_udp_comm \
		test_config

test:
	for name in $(TESTS); do \
  		cargo test -- --show-output $$name ; \
  	done

build:
	cargo build


clean:
	cargo clean


run:
	cargo run

check:
	cargo check

test_config:
	RUST_BACKTRACE=1 cargo test -- --show-output test_config

test_sim:
	RUST_BACKTRACE=1 cargo test -- --show-output test_ping_server