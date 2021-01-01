#!/bin/sh

# the binary is going to be installed to the following dir:
INSTALL_PATH="$HOME/.local/bin/"

# make the directory if it doesn't exist already
if [ -d $INSTALL_PATH ]; then
    mkdir ~/.local/bin/ -pv # pv so it's recursive (.local may not exist) and
                            # v so it prints it to the terminal screen
fi

# check if .local/bin is in the path env variable
if [ $(echo $PATH | grep -i $INSTALL_PATH) ] ; then
    echo "The directory is not in the PATH variable, I'll add it for you..."
    echo "export PATH=\"$PATH:$HOME/.local/bin\"" >> ~/.profile # the profile file will be
                                                                # executed on user login
    export PATH="$PATH:$HOME/.local/bin" # export it for the current terminal session too
    echo "Done!"
    sleep 1;
else
    echo "The directory is already in the PATH variable, cool!" # nothing to do
    sleep 1;
fi

# compile the thing
echo "Compiling pulga..."
sleep 1;
cargo build --release

# move the binary to the directory previously created
echo "Moving the pulga binary..."
sleep 1;
cp target/release/pulga $INSTALL_PATH && \
    chmod +x  $INSTALL_PATH/pulga # make it executable, just in case...

# notify the user when the installation is done, and what to do next
echo " Try running the 'pulga' command now."
