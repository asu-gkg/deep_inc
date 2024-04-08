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
	cargo test -- --show-output test_config

test_sim:
	cargo test -- --show-output simulate_it