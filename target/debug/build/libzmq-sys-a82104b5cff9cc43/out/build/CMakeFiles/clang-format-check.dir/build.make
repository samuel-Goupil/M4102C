# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.19

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Disable VCS-based implicit rules.
% : %,v


# Disable VCS-based implicit rules.
% : RCS/%


# Disable VCS-based implicit rules.
% : RCS/%,v


# Disable VCS-based implicit rules.
% : SCCS/s.%


# Disable VCS-based implicit rules.
% : s.%


.SUFFIXES: .hpux_make_needs_suffix_list


# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/iut/.cargo/registry/src/github.com-1ecc6299db9ec823/zeromq-src-0.1.10+4.3.2/vendor

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/iut/M4102C/target/debug/build/libzmq-sys-a82104b5cff9cc43/out/build

# Utility rule file for clang-format-check.

# Include the progress variables for this target.
include CMakeFiles/clang-format-check.dir/progress.make

CMakeFiles/clang-format-check:
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --blue --bold --progress-dir=/home/iut/M4102C/target/debug/build/libzmq-sys-a82104b5cff9cc43/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Checking correct formatting according to .clang-format file using clang-format"
	chmod +x clang-format-check.sh
	./clang-format-check.sh

clang-format-check: CMakeFiles/clang-format-check
clang-format-check: CMakeFiles/clang-format-check.dir/build.make

.PHONY : clang-format-check

# Rule to build all files generated by this target.
CMakeFiles/clang-format-check.dir/build: clang-format-check

.PHONY : CMakeFiles/clang-format-check.dir/build

CMakeFiles/clang-format-check.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/clang-format-check.dir/cmake_clean.cmake
.PHONY : CMakeFiles/clang-format-check.dir/clean

CMakeFiles/clang-format-check.dir/depend:
	cd /home/iut/M4102C/target/debug/build/libzmq-sys-a82104b5cff9cc43/out/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/iut/.cargo/registry/src/github.com-1ecc6299db9ec823/zeromq-src-0.1.10+4.3.2/vendor /home/iut/.cargo/registry/src/github.com-1ecc6299db9ec823/zeromq-src-0.1.10+4.3.2/vendor /home/iut/M4102C/target/debug/build/libzmq-sys-a82104b5cff9cc43/out/build /home/iut/M4102C/target/debug/build/libzmq-sys-a82104b5cff9cc43/out/build /home/iut/M4102C/target/debug/build/libzmq-sys-a82104b5cff9cc43/out/build/CMakeFiles/clang-format-check.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/clang-format-check.dir/depend

