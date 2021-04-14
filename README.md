# Voronoi Diagram Generator
This program randomly generates Voronoi diagrams using settings loaded from a config file.

### Usage
First, create a config file for the program to load; for this, you can just make a copy of `example_config.ini` and then change the values to what you want.

Once you've created your config file, run the program, it will prompt for the path to the config file, enter the path, and the program will run based on the settings in the file, given that the settings are valid.

### Custom color generators
As of v0.2.0, the program supports custom color generation scripts. In order to use this feature, write a script that, when run, will output three numbers in the form `a b c` (numbers separated by spaces) to stdout. These numbers are the RGB values of a color, so they should be in the range 0-255 (inclusive). The program will run your script once for each color. Once the script is created, enter the command to run it in your config file as `custom_color_generator`.
