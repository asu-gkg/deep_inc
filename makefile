TESTS = test_udp_comm \
#		test_config

test:
	for name in $(TESTS); do \
  		cargo test -- --show-output $$name ; \
  	done

clean:
	cargo clean