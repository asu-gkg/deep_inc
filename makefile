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

test_config:
	cargo test -- --show-output test_config
