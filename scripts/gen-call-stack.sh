rm -f druid/target/thumbv7em-none-eabihf/release/examples/hello*.o

mv Cargo.toml Cargo.toml.OLD
cd druid

cargo call-stack -v --target thumbv7em-none-eabihf --example hello >../call-stack.dot

cd ..
mv Cargo.toml.OLD Cargo.toml

arm-none-eabi-readelf --sections druid/target/thumbv7em-none-eabihf/release/examples/hello*.o
arm-none-eabi-readelf --sections druid/target/thumbv7em-none-eabihf/release/examples/hello
