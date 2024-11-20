os=$(uname -o)
if [ "$os" = "Msys" ]; then
	start cargo run --bin server
else
	nohup cargo run --bin server &
fi
