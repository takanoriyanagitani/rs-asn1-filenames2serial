#!/bin/sh

ls |
	sort |
	head -3 |
	wazero \
		run \
		./rs-asn1-filenames2serial.wasm |
	fq \
		-d asn1_ber \
		'.constructed[].constructed[].value'
