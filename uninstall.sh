if [ "$(id -u)" -ne 0 ]; then
  echo "Error: This script must be run with superuser privileges." >&2
  exit 1
fi

SHELL_WRAPPER="navi-gator-sh"
RUST_APP="navi-gator"
INSTALL_DIR="/usr/local/bin"

rm "$INSTALL_DIR/$SHELL_WRAPPER"
rm "$INSTALL_DIR/$RUST_APP"
