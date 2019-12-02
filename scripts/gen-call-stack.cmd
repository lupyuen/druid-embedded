del druid\target\thumbv7em-none-eabihf\release\examples\hello*.o
cd druid
cargo call-stack -v --target thumbv7em-none-eabihf --example hello
cd ..
arm-none-eabi-readelf --sections druid\target\thumbv7em-none-eabihf\release\examples\hello*.o
arm-none-eabi-readelf --sections druid\target\thumbv7em-none-eabihf\release\examples\hello
