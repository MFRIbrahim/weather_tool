# weather_tool

A simple CLI weather tool. This tool uses the tomorrow.io weather API to get the current overall weather condition, the temperature, the humidity and the wind speed for the given latitude and longitude. For usage it is required to have a tomorrow.io weather api key which you can get from [here](https://www.tomorrow.io/weather-api/) by signing up.

# Usage

To use this tool build it with cargo. Simply [install Rust](https://www.rust-lang.org/tools/install), clone the repository and run ```cargo build --release``` within the repository folder. To run the tool simply use 
```
./target/release/image_comparison <LAT> <LONG> <KEY>
```  
, where LAT is the latitude, LONG is the longitude and KEY is the tomorrow.io weather api key. A description  of the tool can also be accessed in the command line through the usage of the help flag ```-h``` or ```--help```.

