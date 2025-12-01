1. Create a Mojo project
To install Mojo, we recommend using pixi (for other options, see the install guide).

If you don't have pixi, you can install it with this command:

curl -fsSL https://pixi.sh/install.sh | sh

Navigate to the directory where you want to create the project and execute:

pixi init life \
  -c https://conda.modular.com/max-nightly/ -c conda-forge \
  && cd life

This creates a project directory named life, adds the Modular conda package channel, and navigates into the directory.

You can skip the -c options if you add these channels as defaults.

Install the mojo package:

pixi add mojo

Now let's list the project contents:

ls -A

.gitattributes
.gitignore
.pixi
pixi.lock
pixi.toml
You should see that the project directory contains:

An initial pixi.toml manifest file, which defines the project dependencies and other features

A lock file named pixi.lock, which specifies the transitive dependencies and actual package versions installed in the project's virtual environment

Never edit the lock file directly. The pixi command automatically updates the lock file if you edit the manifest file.

A .pixi subdirectory containing the conda virtual environment for the project

Initial .gitignore and .gitattributes files that you can optionally use if you plan to use Git version control with the project

Let's verify that our project is configured correctly by checking the version of Mojo that's installed in our project's virtual environment. pixi run executes a command in the project's virtual environment, so let's use it to execute mojo --version:

pixi run mojo --version

You should see a version string indicating the version of Mojo installed, which by default should be the latest version. You can view and edit the version for your project in the dependencies list in the pixi.toml file.

Great! Now let's write our first Mojo program.

2. Create a "Hello, world" program
In the project directory, create a file named life.mojo containing the following lines of code:

life.mojo
# My first Mojo program!
def main():
    print("Hello, World!")

You can use any editor or IDE that you like. If you're using Visual Studio Code you can take advantage of the Mojo for Visual Studio Code extension, which provides features like syntax highlighting, code completion, and debugging support. For Cursor and other editors that support VS Code extensions, you can install the Mojo for Visual Studio Code extension from the Open VSX Registry. See Add the VS Code extension for more information.

If you've programmed in Python before, this should look familiar.

We're using the def keyword to define a function named main.
You can use any number of spaces or tabs for indentation as long as you use the same indentation for the entire code block. We'll follow the Python style guide and use 4 spaces.
This print() function is a Mojo built-in, so it doesn't require an import.
An executable Mojo program requires you to define a no-argument main() function as its entry point. Running the program automatically invokes the main() function, and your program exits when the main() function returns.

To run the program, we first need to start a shell session in our project's virtual environment:

pixi shell

Later on, when you want to exit the virtual environment, just type exit.

Now we can use the mojo command to run our program.

mojo life.mojo

Hello, World!
Mojo is a compiled language, not an interpreted one like Python. When we run our program like this, mojo performs just-in-time compilation (JIT) and then runs the result.

We can also compile our program into an executable file using mojo build like this:

mojo build life.mojo

By default, this saves an executable file named life to the current directory.

./life

Hello, World!