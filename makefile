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

test_sim_ping:
	RUST_BACKTRACE=1 cargo test -- --show-output test_ping_server

test_sim:
	RUST_BACKTRACE=1 cargo test -- --show-output test_add_rpc

build_py:
	maturin develop

run_agg:
	python3 run_agg.py

clear_key:
	/tmp/etcd-download-test/etcdctl del --from-key ''

run_worker:
	python3 run_worker.py