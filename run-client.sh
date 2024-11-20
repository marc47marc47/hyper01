os=$(uname -o)
if [ "$os" = "Msys" ]; then
	cargo run --bin client
else
	cargo run --bin client
fi
